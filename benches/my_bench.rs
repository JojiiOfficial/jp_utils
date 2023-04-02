use std::str::FromStr;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jp_utils::{
    furigana::{self, compare::FuriComparator, parse::parse_seq_ref, seq::FuriSequence},
    reading::r_owned::ReadingOwned,
};

fn index_item_decode(c: &mut Criterion) {
    let example = "[水|みず]、ガス、[電気|でん|き]が[遠|とお]くから[運|はこ]ばれて[我々|われわれ]の[要求|よう|きゅう]を[満|み]たすためになんなく[供給|きょう|きゅう]されているように、いつか[画像|が|ぞう]と[音楽|おん|がく]はちょっとした[合図|あい|ず]みたいなシンプルな[手|て]の[仕草|し|ぐさ]によって[提供|てい|きょう]されることにもなります。";

    let example2 = "[水|みず]、ガス、[電気|でん|き]が[遠|とお]くから[運|はこ]ばれて[我々|われわれ]の[要求|よう|きゅう]を[満|み]たすためになんなく[供給|きょう|きゅう]されているように、いつか[画像|が|ぞう]と[音楽|おん|がく]はちょっとした[合図|あい|ず]みたいなシンプルな[手|て]の[仕草|し|ぐさ]によって[提供|ていきょう]されることにもなります。";

    c.bench_function("bench parse", |b| {
        b.iter(|| {
            let _ = furigana::parse::parse_seq(black_box(example));
        });
    });

    c.bench_function("bench parse ref", |b| {
        b.iter(|| {
            let _ = furigana::parse::parse_seq_ref(black_box(example));
        });
    });

    c.bench_function("bench parse ref unchecked", |b| {
        b.iter(|| {
            let _ = furigana::parse::parse_seq_ref_unchecked(black_box(example));
        });
    });

    let example_seq = parse_seq_ref(example).unwrap();
    let example2_seq = parse_seq_ref(example2).unwrap();
    c.bench_function("bench compare equal literals", |b| {
        b.iter(|| {
            let _ =
                FuriComparator::new(true).eq_seq(black_box(&example_seq), black_box(&example2_seq));
        });
    });

    c.bench_function("bench compare", |b| {
        b.iter(|| {
            let _ = FuriComparator::new(false)
                .eq_seq(black_box(&example_seq), black_box(&example2_seq));
        });
    });

    c.bench_function("SeqToReading", |b| {
        let seq = FuriSequence::from_str(example).unwrap();

        b.iter(|| {
            let _: ReadingOwned = (&seq).into();
        });
    });
}

criterion_group!(benches, index_item_decode);
criterion_main!(benches);
