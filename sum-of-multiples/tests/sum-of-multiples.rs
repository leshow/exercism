#![feature(test)]
extern crate test;
extern crate sum_of_multiples;

use sum_of_multiples::*;

use test::Bencher;

#[test]
fn multiples_one() {
    assert_eq!(0, sum_of_multiples(1, &mut vec![3, 5]))
}

#[test]
fn multiples_two() {
    assert_eq!(3, sum_of_multiples(4, &mut vec![3, 5]))
}

#[test]
fn multiples_three() {
    assert_eq!(23, sum_of_multiples(10, &mut vec![3, 5]))
}

#[test]
fn multiples_four() {
    assert_eq!(2318, sum_of_multiples(100, &mut vec![3, 5]))
}

#[test]
fn multiples_five() {
    assert_eq!(233168, sum_of_multiples(1000, &mut vec![3, 5]))
}

#[test]
fn multiples_six() {
    assert_eq!(51, sum_of_multiples(20, &mut vec![7, 13, 17]))
}

#[test]
fn multiples_seven() {
    assert_eq!(30, sum_of_multiples(15, &mut vec![4, 6]))
}

#[test]
fn multiples_eight() {
    assert_eq!(4419, sum_of_multiples(150, &mut vec![5, 6, 8]))
}

#[test]
fn multiples_nine() {
    assert_eq!(275, sum_of_multiples(51, &mut vec![5, 25]))
}

#[test]
fn multiples_ten() {
    assert_eq!(2203160, sum_of_multiples(10000, &mut vec![43, 47]))
}

#[test]
fn multiples_eleven() {
    assert_eq!(4950, sum_of_multiples(100, &mut vec![1]))
}


#[bench]
fn bench_mine_100_3_5(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(100);
               sum_of_filter(n, &mut vec![3, 5])
           });
}

#[bench]
fn bench_mine_15_4_6(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(15);
               sum_of_filter(n, &mut vec![4, 6])
           });
}

#[bench]
fn bench_mine_20_7_13_17(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(20);
               sum_of_filter(n, &mut vec![7, 13, 17])
           });
}
#[bench]
fn bench_mine_1000_3_5(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(1000);
               sum_of_filter(n, &mut vec![3, 5])
           });
}
#[bench]
fn bench_mine_10000_43_47(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(10000);
               sum_of_filter(n, &mut vec![43, 47])
           });
}

// dan
#[bench]
fn bench_dan_100_3_5(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(100);
               sum_of_multiples(n, &mut vec![3, 5])
           });
}

#[bench]
fn bench_dan_15_4_6(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(15);
               sum_of_multiples(n, &mut vec![4, 6])
           });
}
#[bench]
fn bench_dan_1000_3_5(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(1000);
               sum_of_multiples(n, &mut vec![3, 5])
           });
}
#[bench]
fn bench_dan_20_7_13_17(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(20);
               sum_of_multiples(n, &mut vec![7, 13, 17])
           });
}
#[bench]
fn bench_dan_10000_43_47(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(10000);
               sum_of_multiples(n, &mut vec![43, 47])
           });
}


// flat_map
#[bench]
fn bench_flat_100_3_5(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(100);
               sum_of_flat(n, &mut vec![3, 5])
           });
}

#[bench]
fn bench_flat_15_4_6(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(15);
               sum_of_flat(n, &mut vec![4, 6])
           });
}
#[bench]
fn bench_flat_1000_3_5(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(1000);
               sum_of_flat(n, &mut vec![3, 5])
           });
}
#[bench]
fn bench_flat_20_7_13_17(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(20);
               sum_of_flat(n, &mut vec![7, 13, 17])
           });
}

#[bench]
fn bench_flat_10000_43_47(b: &mut Bencher) {
    b.iter(|| {
               let n = test::black_box(10000);
               sum_of_flat(n, &mut vec![43, 47])
           });
}