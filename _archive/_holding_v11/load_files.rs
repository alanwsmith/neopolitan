use rusqlite::Connection;
use rusqlite::Result;
use std::fs;
// use walkdir::DirEntry;
use std::time::UNIX_EPOCH;
use walkdir::Error;
use walkdir::WalkDir;

pub fn load_files(source_dir: &str, db_path: &str) -> Result<(), Error> {
    let mut count: i32 = 0;
    for entry in WalkDir::new(source_dir).into_iter() {
        let p = entry?.path().to_path_buf();
        if let Some(ext) = p.extension() {
            if ext == "neo" {
                let conn = Connection::open(db_path).unwrap();
                let mut get_data = conn
                    .prepare("SELECT last_updated FROM pages WHERE path = ?1")
                    .unwrap();
                let response = get_data
                    .query_map(["alfa"], |row| {
                        Ok((row.get::<usize, String>(0), row.get::<usize, String>(1)))
                    })
                    .unwrap();
                let metadata = fs::metadata(&p);
                let timestamp = metadata
                    .unwrap()
                    .modified()
                    .unwrap()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                dbg!(timestamp);

                if response.size_hint().1 == Some(0) {
                    dbg!("Needs to be made");
                    let insert_data = "
                        INSERT INTO pages (last_updated, path) VALUES (?1, ?2)";
                    conn.execute(insert_data, (timestamp, p.display().to_string()))
                        .unwrap();
                }

                // for item in response {
                //     println!("{}", item.unwrap().0.unwrap());
                // }

                println!("{:?}", p);

                //         let mut sf = SourceFile::new();
                //         sf.input_path = Some(p.clone());
                //         dbg!(&sf.input_path);
                //         sf.raw = Some(
                //             fs::read_to_string(
                //                 sf.input_path
                //                     .as_ref()
                //                     .unwrap()
                //                     .as_os_str()
                //                     .to_str()
                //                     .unwrap(),
                //             )
                //             .unwrap(),
                //         );
                //         let parsed_data = parse(sf.raw.as_ref().unwrap().as_str());
                //         match parsed_data {
                //             Err(e) => {}
                //             Ok(data) => {
                //                 sf.parsed = data.1;
                //                 &self.source_files.push(sf);
                //             }
                //         }
                //         // sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
                //         // &self.source_files.push(sf);
            }
        }
        count += 1;
        println!("{}", count);
    }
    Ok(())
}
