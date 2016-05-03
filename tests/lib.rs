extern crate warc_parser;
extern crate nom;

mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use nom::{IResult, Needed, FileProducer};
    use nom::{Producer, Consumer, ConsumerState, Input, Move, MemProducer, HexDisplay};
    fn read_sample_file(sample_name: &str) -> Vec<u8> {
        let full_path = "sample/".to_string() + sample_name;
        let mut f = File::open(full_path).unwrap();
        let mut s = Vec::new();
        f.read_to_end(&mut s).unwrap();
        s
    }
    use warc_parser;
    #[test]
    fn it_stream_parses_file() {
        let mut producer = FileProducer::new("sample/plethora.warc", 50000).unwrap();
        let examples = read_sample_file("plethora.warc");
        //let mut producer =  MemProducer::new(&examples, 49999);
        let mut consumer = warc_parser::WarcConsumer {
            state: warc_parser::State::Beginning,
            c_state: ConsumerState::Continue(Move::Consume(0)),
            counter: 0,
            records: Vec::new(),
        };
        while let &ConsumerState::Continue(_) = producer.apply(&mut consumer) {
            println!("record count:{:?}", consumer.records.len());
        }

        assert_eq!(consumer.counter, 8);
        assert_eq!(consumer.state, warc_parser::State::Done);
    }

    #[test]
    fn it_parses_a_plethora() {
        let examples = read_sample_file("plethora.warc");
        let parsed = warc_parser::records(&examples);
        assert!(parsed.is_done());
        match parsed {
            IResult::Error(_) => assert!(false),
            IResult::Incomplete(_) => assert!(false),
            IResult::Done(i, records) => {
                let empty: Vec<u8> = Vec::new();
                assert_eq!(empty, i);
                assert_eq!(8, records.len());
            }
        }
    }

    #[test]
    fn it_parses_single() {
        let bbc = read_sample_file("bbc.warc");
        let parsed = warc_parser::record(&bbc);
        assert!(parsed.is_done());
        match parsed {
            IResult::Error(_) => assert!(false),
            IResult::Incomplete(_) => assert!(false),
            IResult::Done(i, record) => {
                let empty: Vec<u8> = Vec::new();
                assert_eq!(empty, i);
                assert_eq!(13, record.headers.len());
            }
        }
    }

    #[test]
    fn it_parses_incomplete() {
        let bbc = read_sample_file("bbc.warc");
        let parsed = warc_parser::record(&bbc[..bbc.len() - 10]);
        assert!(!parsed.is_done());
        match parsed {
            IResult::Error(_) => assert!(false),
            IResult::Incomplete(needed) => assert_eq!(Needed::Size(10), needed),
            IResult::Done(_, _) => assert!(false),
        }
    }
}
