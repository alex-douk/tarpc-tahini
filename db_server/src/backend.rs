use database_tahini_utils::types::PolicyError;
use sesame::context::Context;
use sesame_mysql::SesameConn;
use sesame_mysql::{PConOpts, PConParams, PConStatement, PConValue};
use std::collections::HashMap;
use std::error::Error;
use std::result::Result;

pub struct MySqlBackend {
    pub handle: SesameConn,
    // pub log: slog::Logger,
    //_schema: String,
    prep_stmts: HashMap<String, PConStatement>,
    db_user: String,
    db_password: String,
    db_name: String,
}

impl MySqlBackend {
    pub fn new(
        user: &str,
        password: &str,
        dbname: &str,
        // log: Option<slog::Logger>,
        prime: bool,
    ) -> Result<Self, Box<dyn Error>> {
        // let log = match log {
        //     None => slog::Logger::root(slog::Discard, o!()),
        //     Some(l) => l,
        let schema = std::fs::read_to_string("./resources/schema.sql")?;
        //
        // debug!(
        //     log,
        //     "Connecting to MySql DB and initializing schema {}...", dbname
        // );
        // let password = "";
        // println!("password is `{}`", password);
        let mut db = SesameConn::new(
            // this is the user and password from the config.toml file
            PConOpts::from_url(&format!("mysql://{}:{}@127.0.0.1/", user, password)).unwrap(),
        )
        .unwrap();
        assert_eq!(db.ping(), true);

        if prime {
            db.query_drop(format!("DROP DATABASE IF EXISTS {};", dbname))
                .unwrap();
            db.query_drop(format!("CREATE DATABASE {};", dbname))
                .unwrap();
            db.query_drop(format!("USE {};", dbname)).unwrap();
            for line in schema.lines() {
                if line.starts_with("--") || line.is_empty() {
                    continue;
                }
                println!("line is : {}", line);
                db.query_drop(line).unwrap();
            }
        } else {
            db.query_drop(format!("USE {};", dbname)).unwrap();
        }

        Ok(MySqlBackend {
            handle: db,
            // log: log,
            // _schema: schema.to_owned(),
            prep_stmts: HashMap::new(),
            db_user: String::from(user),
            db_password: String::from(password),
            db_name: String::from(dbname),
        })
    }

    fn reconnect(&mut self) {
        self.handle = SesameConn::new(
            PConOpts::from_url(&format!(
                "mysql://{}:{}@127.0.0.1/{}",
                self.db_user, self.db_password, self.db_name
            ))
            .unwrap(),
        )
        .unwrap();
    }

    pub fn prep_exec<P: Into<PConParams>>(
        &mut self,
        sql: &str,
        params: P,
        context: Context<()>,
    ) -> Vec<Vec<PConValue>> {
        if !self.prep_stmts.contains_key(sql) {
            let stmt = self
                .handle
                .prep(sql)
                .expect(&format!("failed to prepare statement \'{}\'", sql));
            self.prep_stmts.insert(sql.to_owned(), stmt);
        }

        let params: PConParams = params.into();
        match self
            .handle
            .exec_iter(self.prep_stmts[sql].clone(), params, context.clone())
        {
            Err(e) => {
                eprintln!("query \'{}\' failed ({}) ", sql, e);
                panic!("Query_failed")
            }
            Ok(res) => {
                let mut rows = vec![];
                for row in res {
                    rows.push(row.unwrap().unwrap());
                }
                //debug!(self.log, "executed query {}, got {} rows", sql, rows.len());
                return rows;
            }
        }
    }

    fn do_insert<P: Into<PConParams>>(
        &mut self,
        table: &str,
        vals: P,
        replace: bool,
        context: Context<()>,
    ) -> Result<(), PolicyError> {
        let vals: PConParams = vals.into();
        let mut param_count = 0;
        if let PConParams::Positional(vec) = &vals {
            param_count = vec.len();
        }

        let op = if replace { "REPLACE" } else { "INSERT" };
        let q = format!(
            "{} INTO {} VALUES ({})",
            op,
            table,
            (0..param_count)
                .map(|_| "?")
                .collect::<Vec<&str>>()
                .join(",")
        );
        if let Err(e) = self.handle.exec_drop(q.clone(), vals, context.clone()) {
            eprintln!(
                "failed to insert into {}, query {} ({}), reconnecting to database",
                table, q, e
            );
            if e.to_string().contains("policy check") {
                return Err(PolicyError);
            }
        }
        Ok(())
    }

    pub fn insert<P: Into<PConParams>>(
        &mut self,
        table: &str,
        vals: P,
        context: Context<()>,
    ) -> Result<(), PolicyError> {
        self.do_insert(table, vals, false, context)
    }

    // pub fn replace<P: Into<BBoxParams>>(
    //     &mut self,
    //     table: &str,
    //     vals: P,
    //     context: Context<()>,
    // ) -> Result<(), PolicyError> {
    //     self.do_insert(table, vals, true, context)
    // }
}

#[derive(Debug)]
pub struct DbConnError;

impl std::fmt::Display for DbConnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Db connection error")
    }
}

impl std::error::Error for DbConnError {}

pub struct MySqlBackendManager {
    db_user: String,
    db_password: String,
    db_name: String,
    prime: bool,
}

impl MySqlBackendManager {
    pub fn new(db_user: &str, db_password: &str, db_name: &str, prime: bool) -> Self {
        Self {
            db_user: String::from(db_user),
            db_password: String::from(db_password),
            db_name: String::from(db_name),
            prime,
        }
    }
}

impl r2d2::ManageConnection for MySqlBackendManager {
    type Connection = MySqlBackend;
    type Error = DbConnError;
    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        MySqlBackend::new(
            self.db_user.as_str(),
            self.db_password.as_str(),
            self.db_name.as_str(),
            self.prime,
        )
        .map_err(|_| DbConnError)
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }

    fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        Ok(())
    }
}
