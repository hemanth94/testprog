// use chrono::{Datelike, NaiveDate};
// use fastrand::Rng;
// use polars::prelude::ParquetWriter;
// use polars::prelude::*;
// // use smartstring::SmartString;
// use std::error::Error;
// use std::fs::File;
// use std::iter::repeat_with;
// use std::path::PathBuf;
// use std::{env, fs};

// const TEST_DIR_DATA: &str = "tmp/data";

// fn generate_and_save_test_data() {
//     fastrand::seed(42);
//     let mut rng = Rng::new();
//     let base_directory = TEST_DIR_DATA.to_string();
//     let source = 1;
//     let start_date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
//     let end_date = NaiveDate::from_ymd_opt(2021, 1, 2).unwrap();
//     let mut date = start_date.clone() - chrono::Duration::days(1);

//     while date < end_date {
//         date += chrono::Duration::days(1);
//         let dir_path = format!("{}/source={}/year={}", base_directory, source, date.year());

//         if fs::metadata(dir_path.clone()).is_ok() {
//             // Omit the creation of the data frames if the directory already exists
//             continue;
//         } else {
//             // Create the directory if it does not exist
//             match fs::create_dir_all(dir_path.clone()) {
//                 Ok(_) => (),
//                 Err(e) => println!("Error creating directory: {}", e),
//             }
//         }

//         let mut df = create_dataframe(&mut rng, date.clone());
//         let file_path = format!("{}/{}.parquet", dir_path, date);
//         let file = fs::File::create(file_path).unwrap();
//         ParquetWriter::new(file)
//             .with_row_group_size(Some(1024))
//             .finish(&mut df)
//             .unwrap();
//     }
// }

// fn create_dataframe(rng: &mut Rng, date: NaiveDate) -> DataFrame {
//     let frame_interval_ms = 4000;

//     let start = date
//         .and_hms_opt(0, 0, 0)
//         .unwrap()
//         .and_utc()
//         .timestamp_millis();
//     let end = date
//         .and_hms_opt(23, 59, 59)
//         .unwrap()
//         .and_utc()
//         .timestamp_millis();

//     let timestamps: Vec<i64> = (start..end)
//         .filter(|x| *x % frame_interval_ms == 0)
//         .collect();

//     let x: Vec<f32> = repeat_with(|| rng.f32()).take(timestamps.len()).collect();
//     let y: Vec<f32> = repeat_with(|| rng.f32()).take(timestamps.len()).collect();
//     let z: Vec<f32> = vec![0.0; timestamps.len()];

//     df!(
//         "timestamp" => &timestamps.clone(),
//         "x" => &x.clone(),
//         "y" => &y.clone(),
//         "z" => &z.clone(),
//     )
//     .unwrap()
// }

// #[test]
// fn test_polars_parquet_scan() {
//     generate_and_save_test_data();

//     let schema_path = format!("{}/*/*/*.parquet", TEST_DIR_DATA.to_string());
//     let schema_path = PathBuf::from(schema_path);

//     let lf = LazyFrame::scan_parquet(schema_path, ScanArgsParquet::default()).unwrap();
//     let lf = lf
//         .filter(
//             col("year")
//                 .eq(lit(2021))
//                 .and(col("x").gt(lit(0.7)))
//                 .and(col("y").lt(lit(0.5)))
//                 .and(col("z").eq(lit(0))),
//         )
//         .collect()
//         .unwrap();
//     lf.get_columns()
//         .iter()
//         .for_each(|s| println!("{} {}", s.name(), s.len()));
//     println!("Result:\n{}", lf);
// }

// #[cfg(test)]
// mod tests {
//     use polars::prelude::{col, Column, DataFrame, IntoLazy, NamedFrom, Series};
//     use std::env;

//     #[test]
//     fn should_work_with_polars() {
//         env::set_var("POLARS_VERBOSE", "1");
//         env::set_var("POLARS_BACKTRACE_IN_ERR", "1"); // <-- added just for debugging

//         let my_col = Series::new("my_col".into(), [None, Some(1)].as_ref());
//         let df = DataFrame::new(vec![Column::from(my_col)]).unwrap();
//         let polars_plan = df
//             //
//             .lazy()
//             .filter(col("my_col").is_not_null());

//         let result = polars_plan.profile();

//         match result {
//             Ok(dataframe) => {
//                 println!("Everything is fine");
//             }
//             Err(err) => {
//                 eprintln!("Error: {}", err);
//                 panic!("Failed to execute plan");
//             }
//         }
//     }
// }

// fn test3() -> Result<(), Box<dyn Error>> {
//     let df = df!(
//         "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
//         "tall" => [false, true, true, true],
//         "heavy" => [false, true, false, true],
//         "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
//         "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
//     )?;
//     // let df = df
//     //     .lazy()
//     //     .with_columns([
//     //         any_horizontal([col("tall"), col("heavy")])?.alias("tall_or_heavy"),
//     //         all_horizontal([col("tall"), col("heavy")])?.alias("tall_and_heavy"),
//     //         // The compiler can't find the function mean_horizontal
//     //         mean_horizontal([col("weight"), col("height")])?.alias("mean"),
//     //     ])
//     //     .collect()?;
//     // println!("{}", df);
//     Ok(())
// }
// fn test4() -> Result<(), Box<dyn Error>> {
//     let df = df!(
//         "keys" => &["a","b", "c"],
//         "values" => &[10, 7, 1],
//     )
//     .unwrap()
//     .lazy();

//     let df2 = df!(
//         "keys" => &["a","b", "c"],
//         "values" => &[10, 7, 1],
//     )
//     .unwrap()
//     .lazy();

//     let result = df.join(
//         df2,
//         [col("keys")],
//         [col("keys")],
//         JoinArgs {
//             how: JoinType::Inner,
//             validation: JoinValidation::default(),
//             suffix: Some("_2".into()),
//             slice: None,
//             join_nulls: false,
//             coalesce: JoinCoalesce::CoalesceColumns,
//             maintain_order: MaintainOrderJoin::Left,
//         },
//     );

//     println!("{:?}", result.collect().unwrap().head(Some(3)));
//     Ok(())
// }

// fn test5() -> Result<(), Box<dyn Error>> {
//     // let mut schema = Schema::with_capacity(3);
//     // schema
//     //     .insert_at_index(0, SmartString::from("UInt32"), DataType::UInt32)
//     //     .expect("error");
//     // schema
//     //     .insert_at_index(1, SmartString::from("UInt16"), DataType::UInt16)
//     //     .expect("error");
//     // schema
//     //     .insert_at_index(2, SmartString::from("UInt8"), DataType::UInt8)
//     //     .expect("error");

//     // let _empty_df = DataFrame::from(&schema);
//     Ok(())
// }

// fn test6() -> Result<(), Box<dyn Error>> {
//     let mut p =
//         ParquetReader::new(File::open("/home/hemanth/Downloads/flights-1m.parquet").unwrap())
//             .set_low_memory(true)
//             .batched(100)
//             .unwrap();
//     // while !p.is_finished() {
//     //     // assert!(task::block_on(p.next_batches(1)).is_ok());
//     // }
//     Ok(())
// }

// fn test7() -> Result<(), Box<dyn Error>> {
//     env::set_var("POLARS_FMT_TABLE_ROUNDED_CORNERS", "1");
//     env::set_var("POLARS_FMT_MAX_COLS", "50");
//     env::set_var("POLARS_FMT_MAX_ROWS", "1000");
//     env::set_var("POLARS_FMT_STR_LEN", "500");

//     let lf = df!(
//         "A" => [1, 1, 2, 4],
//         "B" => ["a", "a", "b", "b"],
//         "C" => [None, None, Some("spam"), Some("spam")]
//     )
//     .unwrap()
//     .lazy();

//     let df = lf
//         .clone()
//         .select([col("A"), col("B"), col("C")])
//         .unique_stable(None, UniqueKeepStrategy::Any)
//         .select([as_struct(vec![all()]).alias("foo")])
//         .collect()?;

//     println!("shows 3 correct unique entries: {:?}", df);

//     let series = lf
//         .select([col("A"), col("B"), col("C")])
//         .unique_stable(None, UniqueKeepStrategy::First)
//         .select([as_struct(vec![all()]).alias("foo")])
//         .collect()?
//         .column("foo")?
//         .struct_()?
//         .clone()
//         .into_series();

//     println!("column {:?}", series);

//     // .struct_()?
//     // .clone()
//     // .into_series();

//     for e in series.iter() {
//         match e {
//             AnyValue::Struct(_length, entries, field) => {
//                 println!("Row: {:?}", entries);
//                 println!("Fields {:?}", field);
//             }
//             _ => panic!("Unexpected value"),
//         }
//     }

//     //     // .for_each(|e| match e {
//     //     //     AnyValue::Struct(length, entries, field) => {
//     //     //         println!(
//     //     //             "Incorrectly iterates 3 times the same 3 unique entries:\n{:?}\n",
//     //     //             entries
//     //     //         );
//     //     //     }
//     //     //     _ => panic!(""),
//     //     // });
//     Ok(())
// }

// fn test1() -> Result<(), Box<dyn Error>> {
//     // let df1 = df!("key1" => &["A", "B", "C"],
//     //           "key2" => &["X", "Y", "Z"],
//     //           "val1" => &[1, 2, 3])?;
//     // let df2 = df!("key1" => &["A", "B", "D"],
//     //           "key2" => &["X", "Y", "W"],
//     //           "val2" => &[4, 5, 6])?;

//     // join on "key1" and "key2"
//     // let lazy1_joined_df = df1
//     //     .clone()
//     //     .lazy()
//     //     .join(
//     //         df2.clone().lazy(),
//     //         [col("key1")],
//     //         [col("key1")],
//     //         JoinArgs::new(JoinType::Inner),
//     //     )
//     //     .collect()?;
//     // println!("{:?}", lazy1_joined_df);

//     // join on "key1" and "key2"
//     // let lazy2_joined_df = df1
//     //     .clone()
//     //     .lazy()
//     //     .join(
//     //         df2.clone().lazy(),
//     //         [cols(["key1", "key2"])],
//     //         [cols(["key1", "key2"])],
//     //         JoinArgs::new(JoinType::Inner),
//     //     )
//     //     .collect()?;
//     // println!("{:?}", lazy2_joined_df);

//     // // Same for this one
//     // let lazy_joined_df = df1
//     //     .clone()
//     //     .lazy()
//     //     .join(
//     //         df2.clone().lazy(),
//     //         [col("key1"), col("key2")],
//     //         [col("key1"), col("key2")],
//     //         JoinArgs::new(JoinType::Inner),
//     //     )
//     //     .collect()?;
//     Ok(())
// }

// fn test8() -> Result<(), Box<dyn Error>> {
//     let cloud_path = "/home/hemanth/Downloads/flights-1m.parquet";
//     // enlarged_output.parquet

//     let args = ScanArgsParquet {
//         parallel: ParallelStrategy::Prefiltered,
//         ..Default::default()
//     };

//     // ScanArgsParquet::default()
//     let df = LazyFrame::scan_parquet(cloud_path, args)
//         .unwrap()
//         .filter(col("AIR_TIME").eq(lit(1)))
//         .collect()
//         .unwrap();

//     let res = df!("d" => &["2300-01-01T00:00:00Z".to_string()])?
//         .lazy()
//         .with_column(col("d").str().strptime(
//             DataType::Datetime(TimeUnit::Nanoseconds, None),
//             StrptimeOptions {
//                 format: Some("%Y-%m-%dT%H:%M:%SZ".to_string().into()),
//                 strict: false,
//                 exact: true,
//                 cache: true,
//             },
//             lit("null"),
//         ))
//         .collect();
//     println!("{}", res.unwrap());
//     Ok(())
// }

// fn test9() -> Result<(), Box<dyn Error>> {
//     let file_path = "/home/hemanth/Downloads/flights-1m.parquet"; // Path to your 6 MB Parquet file
//     let df = LazyFrame::scan_parquet(file_path, ScanArgsParquet::default())?.collect()?;

//     // Step 2: Determine how many times to duplicate the rows
//     let original_size_mb = 6; // Original file size in MB
//     let target_size_mb = 6000; // Target file size in MB
//     let duplication_factor = (target_size_mb / original_size_mb) as usize;

//     // Step 3: Duplicate the rows
//     let mut enlarged_df = df.clone();
//     for _ in 1..duplication_factor {
//         enlarged_df = enlarged_df.vstack(&df)?;
//     }

//     // Step 4: Write the enlarged DataFrame to a new Parquet file
//     let output_file_path = "/home/hemanth/Downloads/enlarged_output.parquet";
//     let mut file = File::create(output_file_path)?;
//     ParquetWriter::new(&mut file).finish(&mut enlarged_df)?;

//     println!(
//         "Enlarged DataFrame written to {} with {} rows.",
//         output_file_path,
//         enlarged_df.height()
//     );

//     Ok(())
// }
// fn main() {
//     let res = test8();
// }

// fn test2() -> Result<(), Box<dyn std::error::Error>> {
//     // Create a sample DataFrame with a single row and 5 columns
//     let mut df = df!(
//         "column1" => &[1],
//         "column2" => &["A"],
//         "column3" => &[3.14],
//         "column4" => &[true],
//         "column5" => &[Some("example")],
//         "column6" => &[1],
//         "column7" => &["A"],
//         "column8" => &[3.14],
//         "column9" => &[1],
//         "column10" => &["A"],
//         "column11" => &[3.14],

//     )?;

//     // Print the DataFrame to verify
//     println!("DataFrame:\n{:?}", df);

//     // Serialize the DataFrame to a CSV file
//     let mut file = File::create("output.csv")?;

//     let mut buffer = Vec::new();
//     CsvWriter::new(&mut buffer)
//         .include_header(true) // Include headers in the CSV
//         .finish(&mut df)?;

//     println!("DataFrame serialized to 'output.csv'");
//     Ok(())
// }

use external::api_call;
use hashbrown::HashMap;

fn main() {}

mod external {

    /// performs a call to a third party API
    pub async fn api_call(query_param: &str) -> String {
        unimplemented!();
    }
    use core::fmt::Debug;
    use core::marker::PhantomData;
    use std::sync::LazyLock;

    use hashbrown::HashMap;
    pub struct Stream<T: Send + 'static + Debug> {
        _pd: PhantomData<T>,
    }
    impl<T: Send + 'static + Debug> Stream<T> {
        /// returns the next element of the stream
        pub async fn next(&self) -> Option<T> {
            unimplemented!();
        }
    }
}
type Query = String;
pub async fn do_stuff_here(stream: external::Stream<Query>) {
    // let res = stream
    //     .next()
    //     .await

    let mut mapx: HashMap<String, String> = HashMap::new();
    loop {
        let dres = stream.next().await;

        match dres {
            Some(x) => {
                if (mapx.contains_key(&x)) {
                    let resp = mapx.get(&x).unwrap();
                    println!("ALready in cache, no need for api call. response {}", *resp);
                } else {
                    let res = api_call(&x).await;
                    println!("Response from the api {}", res);
                    //add to cache
                    mapx.insert(x, res);
                }
            }
            None => break,
        }
    }
    // here we want to process the queries we get from the stream
    // and println! the response we get from the API
    // but to reduce the network traffic, we want to add a caching mechanism
    // to avoid making request for the same query over and over
}
