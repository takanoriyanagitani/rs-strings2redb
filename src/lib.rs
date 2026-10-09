use std::io;
use std::path::Path;

use io::BufRead;

use redb::Database;
use redb::TableDefinition;

pub const TABLE_NAME_DEFAULT: &str = "strings";

pub const TABLE_DEF_DEFAULT: TableDefinition<u32, &str> = TableDefinition::new(TABLE_NAME_DEFAULT);

pub fn strings2redb<I>(
    strings: I,
    redb: &Database,
    tdef: TableDefinition<u32, &str>,
) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    let tx = redb.begin_write().map_err(io::Error::other)?;

    let mut tab = tx.open_table(tdef).map_err(io::Error::other)?;
    for pair in strings.enumerate() {
        let (ix, rstr) = pair;
        let s: String = rstr?;
        let serial: u32 = ix.try_into().map_err(io::Error::other)?;

        tab.insert(serial, s.as_str()).map_err(io::Error::other)?;
    }
    drop(tab);

    tx.commit().map_err(io::Error::other)
}

pub fn strings2redb_default<I>(strings: I, redb: &Database) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    strings2redb(strings, redb, TABLE_DEF_DEFAULT)
}

pub struct Db(pub Database);

impl Db {
    pub fn into_string_sink<I>(
        self,
        tdef: TableDefinition<u32, &str>,
    ) -> impl FnMut(I) -> Result<(), io::Error>
    where
        I: Iterator<Item = Result<String, io::Error>>,
    {
        move |strings: I| strings2redb(strings, &self.0, tdef)
    }
}

impl Db {
    pub fn into_string_sink_default<I>(self) -> impl FnMut(I) -> Result<(), io::Error>
    where
        I: Iterator<Item = Result<String, io::Error>>,
    {
        self.into_string_sink(TABLE_DEF_DEFAULT)
    }
}

pub fn path2db<P>(p: P) -> Result<Database, io::Error>
where
    P: AsRef<Path>,
{
    Database::create(&p)
        .map_err(|e| format!("unable to create the db for path {:#?}: {e}", p.as_ref()))
        .map_err(io::Error::other)
}

pub fn path2sink_default<P, I>(p: P) -> Result<impl FnMut(I) -> Result<(), io::Error>, io::Error>
where
    P: AsRef<Path>,
    I: Iterator<Item = Result<String, io::Error>>,
{
    let d: Database = path2db(p)?;
    Ok(Db(d).into_string_sink_default())
}

pub fn brdr2strings<R>(brdr: R, split: u8) -> impl Iterator<Item = Result<String, io::Error>>
where
    R: BufRead,
{
    brdr.split(split).map(|r| {
        r.map_err(io::Error::other).map(|v: Vec<u8>| {
            let rs = String::from_utf8(v);
            match rs {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("warn: non-utf8 value got: {e}");
                    e.into_utf8_lossy()
                }
            }
        })
    })
}

pub fn brdr2strings_default<R>(brdr: R) -> impl Iterator<Item = Result<String, io::Error>>
where
    R: BufRead,
{
    brdr2strings(brdr, b'\n')
}

pub fn stdin2strings2path2sink_default(p: String) -> impl FnOnce() -> Result<(), io::Error> {
    move || {
        let mut sink = path2sink_default(p)?;
        let strings = brdr2strings_default(io::stdin().lock());
        sink(strings)
    }
}
