mod common;

use common::{MockIndex, MockIndexWriter, NoCacher, NoCompressor};
use test_log::test;
use value_log::{Config, IndexWriter, ValueLog};

#[test]
fn basic_recovery() -> value_log::Result<()> {
    let folder = tempfile::tempdir()?;
    let vl_path = folder.path();

    let index = MockIndex::default();

    let items = ["a", "b", "c", "d", "e"];

    {
        let value_log = ValueLog::open(
            vl_path,
            Config::<_, _, NoCompressor>::new(NoCacher, NoCacher),
        )?;

        {
            let mut index_writer = MockIndexWriter(index.clone());
            let mut writer = value_log.get_writer()?;

            for key in &items {
                let value = key.repeat(10_000);
                let value = value.as_bytes();

                let key = key.as_bytes();

                let vhandle = writer.get_next_value_handle();
                index_writer.insert_indirect(key, vhandle, value.len() as u32)?;

                writer.write(key, value)?;
            }

            value_log.register_writer(writer)?;
        }

        {
            assert_eq!(1, value_log.segment_count());

            let segments = value_log.manifest.list_segments();

            assert_eq!(items.len() as u64, segments.first().unwrap().len());
            assert_eq!(0, segments.first().unwrap().gc_stats.stale_items());
        }

        for (key, (vhandle, _)) in index.read().unwrap().iter() {
            let item = value_log.get(vhandle)?.unwrap();
            assert_eq!(&*item, &*key.repeat(10_000));
        }
    }

    {
        let value_log = ValueLog::open(
            vl_path,
            Config::<_, _, NoCompressor>::new(NoCacher, NoCacher),
        )?;

        value_log.scan_for_stats(index.read().unwrap().values().cloned().map(Ok))?;

        {
            assert_eq!(1, value_log.segment_count());

            let segments = value_log.manifest.list_segments();

            assert_eq!(items.len() as u64, segments.first().unwrap().len());
            assert_eq!(0, segments.first().unwrap().gc_stats.stale_items());
        }

        for (key, (vhandle, _)) in index.read().unwrap().iter() {
            let item = value_log.get(vhandle)?.unwrap();
            assert_eq!(&*item, &*key.repeat(10_000));
        }
    }

    Ok(())
}

#[test]
fn recovery_delete_unfinished() -> value_log::Result<()> {
    let folder = tempfile::tempdir()?;
    let vl_path = folder.path();

    {
        let value_log = ValueLog::open(
            vl_path,
            Config::<_, _, NoCompressor>::new(NoCacher, NoCacher),
        )?;

        let mut writer = value_log.get_writer()?;
        writer.write("a", "a")?;
        value_log.register_writer(writer)?;
    }

    let faux_segment = vl_path.join("segments").join("73");
    {
        std::fs::File::create(&faux_segment)?;
        assert!(faux_segment.try_exists()?);
    }

    {
        let value_log = ValueLog::open(
            vl_path,
            Config::<_, _, NoCompressor>::new(NoCacher, NoCacher),
        )?;
        assert_eq!(1, value_log.segment_count());
    }

    assert!(!faux_segment.try_exists()?);

    Ok(())
}
