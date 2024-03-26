pub mod formats;
pub mod lava;

use tokio::task::JoinSet;
use rand::{thread_rng, Rng};
use formats::io::{AsyncReader, get_operator_and_filename_from_file, READER_BUFFER_SIZE};


const PAGE_SIZE:u64 = 512 * 1024;
const TOTAL_ITERATIONS:u64 = 2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let filenames = [
        "part-00000-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00001-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00002-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00003-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00004-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00005-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00006-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00007-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00008-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00009-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00010-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00011-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00012-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00013-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00014-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
        "part-00015-befcd7c9-87c8-40a4-a02d-eccf8dea8312-c000.snappy.parquet",
    ];

    let files = filenames.iter().map(|filename|format!("s3://quokka-asof-parquet/quotes/{}", filename)).collect::<Vec<_>>();

    let mut operators = files.iter().map(|file| get_operator_and_filename_from_file(file.to_string())).collect::<Vec<_>>();
    let mut join_set = JoinSet::new();
    for (operator, filename) in operators.iter_mut() {
        let mut operator = operator.clone();
        let filename = filename.clone();
        

        join_set.spawn(async move {
            let mut i = 0;
            let count = TOTAL_ITERATIONS;
            
            let filesize = operator.stat(&filename).await.unwrap().content_length() as usize;
            let mut reader: AsyncReader = operator
                .clone()
                .reader_with(&filename)
                .buffer(READER_BUFFER_SIZE)
                .await.unwrap()
                .into();

            println!("thread id {:?}", std::thread::current().id());
            while i < count {
                i += 1;
                let from = thread_rng().gen_range(0..(filesize as u64 - PAGE_SIZE));
                let to = from + PAGE_SIZE;
                let res = reader.read_range(from, to).await.unwrap();
                println!("Read {} bytes from {}", res.len(), filename);
            }
        });
    }

    while let Some(_) = join_set.join_next().await {
        println!("Task completed");
    }

    Ok(())
}