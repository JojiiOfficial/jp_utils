use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jp_utils::{
    furi::{
        cformat::CodeFormatter, compare::FuriComparator, parse::FuriParser,
        segment::encode::FuriEncoder, seq::FuriSequence, Furigana,
    },
    reading::Reading,
};
use std::str::FromStr;

fn index_item_decode(c: &mut Criterion) {
    let example = "[水|みず]、ガス、[電気|でん|き]が[遠|とお]くから[運|はこ]ばれて[我々|われわれ]の[要求|よう|きゅう]を[満|み]たすためになんなく[供給|きょう|きゅう]されているように、いつか[画像|が|ぞう]と[音楽|おん|がく]はちょっとした[合図|あい|ず]みたいなシンプルな[手|て]の[仕草|し|ぐさ]によって[提供|てい|きょう]されることにもなります。";

    let example2 = "[水|みず]、ガス、[電気|でん|き]が[遠|とお]くから[運|はこ]ばれて[我々|われわれ]の[要求|よう|きゅう]を[満|み]たすためになんなく[供給|きょう|きゅう]されているように、いつか[画像|が|ぞう]と[音楽|おん|がく]はちょっとした[合図|あい|ず]みたいなシンプルな[手|て]の[仕草|し|ぐさ]によって[提供|ていきょう]されることにもなります。";

    let example3 = "[水|みず]、ガス、[電気|でん|き]が[遠|とお]くから[運|はこ]ばれて[我々|われわれ]の[要求|よう|きゅう]を[満|み]たすためになんなく[供|きょう][給|きゅう]されているように、いつか[画像|が|ぞう]と[音楽|おん|がく]はちょっとした[合|あい][図|ず]みたいなシンプルな[手|て]の[仕|し][草|ぐさ]によって[提供|ていきょう]されることにもなります。";

    let parts = Furigana("[水|みず]、ガス、[電気|でん|き]が[遠|とお]くから[運|はこ]ばれて[我々|われわれ]の[要求|よう|きゅう]を[満|み]たすためになんなく[供給|きょう|きゅう]されているように、いつか[画像|が|ぞう]と[音楽|おん|がく]はちょっとした[合図|あい|ず]みたいなシンプルな[手|て]の[仕草|し|ぐさ]によって[提供|てい|きょう]されることにもなります。").as_segments_ref();

    c.bench_function("encode new", |b| {
        let _ = b.iter(|| {
            let mut buf = String::new();
            let mut enc = FuriEncoder::new(&mut buf);
            for part in parts.iter() {
                enc.write_seg(part);
            }
        });
    });

    c.bench_function("encode new iter", |b| {
        let _ = b.iter(|| {
            let mut buf = String::new();
            let mut enc = FuriEncoder::new(&mut buf);
            enc.extend(parts.iter());
        });
    });

    c.bench_function("parse to kanji and kana", |b| {
        let furigana = Furigana::new_unchecked(black_box(example));
        b.iter(|| {
            let _ = furigana.kana_str();
            let _ = furigana.kanji_str();
        });
    });

    c.bench_function("furi merge kanji readings2 ", |b| {
        b.iter(|| {
            let _ = CodeFormatter::new(&black_box(Furigana(
                "それは[音|おん][楽|がく][大学|だい|がく]です",
            )))
            .merge_kanji_parts()
            .finish();
        });
    });

    c.bench_function("furi merge kanji readings", |b| {
        let furigana = Furigana(black_box(example3));
        b.iter(|| {
            let _ = CodeFormatter::new(&black_box(furigana))
                .merge_kanji_parts()
                .finish();
        });
    });

    c.bench_function("furi replace", |b| {
        let furigana = Furigana::new_unchecked(black_box(example));
        b.iter(|| {
            let _ = furigana.replace_seg(
                black_box(("ていきょう", "提供")),
                black_box(("おんがく", "音楽")),
            );
        });
    });

    c.bench_function("parse to reading", |b| {
        let furigana = Furigana::new_unchecked(black_box(example));
        b.iter(|| {
            let _ = furigana.to_reading();
        });
    });

    c.bench_function("parse to reading checked", |b| {
        b.iter(|| {
            // let _ = furigana.to_reading();
            let _ = FuriParser::new(black_box(example)).to_reading();
        });
    });

    c.bench_function("has kanji", |b| {
        let furigana = Furigana::new_unchecked(black_box(example));
        b.iter(|| {
            let _ = furigana.has_kanji();
        });
    });

    c.bench_function("get segment at", |b| {
        let furigana = Furigana::new_unchecked(example);
        b.iter(|| {
            let _ = furigana.segment_at(20);
        });
    });

    c.bench_function("get segment count", |b| {
        let furigana = Furigana::new_unchecked(example);
        b.iter(|| {
            let _ = furigana.segment_at(20);
        });
    });

    c.bench_function("bench furi kanji len", |b| {
        let furigana = Furigana::new_unchecked(example);
        b.iter(|| {
            let _ = furigana.kanji().len();
        });
    });

    c.bench_function("bench parse", |b| {
        b.iter(|| {
            let _ = FuriSequence::parse_ref(black_box(example));
        });
    });

    c.bench_function("bench parse ref", |b| {
        b.iter(|| {
            let _ = FuriParser::new(black_box(example)).count();
        });
    });

    c.bench_function("bench parse ref unchecked new", |b| {
        b.iter(|| {
            let _ = FuriParser::new(black_box(example)).unchecked().count();
        });
    });

    let example_seq = FuriSequence::parse_ref(example).unwrap();
    let example2_seq = FuriSequence::parse_ref(example2).unwrap();
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
            let _: Reading = (&seq).into();
        });
    });

    c.bench_function("Furigana to kanji", |b| {
        let furi = Furigana::new_unchecked(example);

        b.iter(|| {
            let _ = furi.kanji().to_string();
        });
    });

    c.bench_function("Furigana to kana", |b| {
        let furi = Furigana::new_unchecked(example);

        b.iter(|| {
            let _ = furi.kana().to_string();
        });
    });
}

criterion_group!(benches, index_item_decode);
criterion_main!(benches);
