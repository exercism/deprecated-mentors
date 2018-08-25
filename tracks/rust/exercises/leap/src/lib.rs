#![feature(test)]
extern crate test;

pub fn is_leap_year1(year: i32) -> bool {
    let div_4 = year % 4 == 0;
    let div_100 = year % 100 == 0;
    let div_400 = year % 400 == 0;
    div_4 && !(div_100 && !div_400)
}

pub fn is_leap_year2(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub fn is_leap_year3(year: i32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}

pub fn is_leap_year4(year: i32) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn leap1_bench(b: &mut Bencher) {
        b.iter( || {
            (0..1200).fold(0, |cnt, year| if is_leap_year1(year) {cnt + 1} else {cnt})
        })
    }

    #[bench]
    fn leap2_bench(b: &mut Bencher) {
        b.iter( || {
            (0..1200).fold(0, |cnt, year| if is_leap_year2(year) {cnt + 1} else {cnt})
        })
    }

    #[bench]
    fn leap3_bench(b: &mut Bencher) {
        b.iter( || {
            (0..1200).fold(0, |cnt, year| if is_leap_year3(year) {cnt + 1} else {cnt})
        })
    }

    #[bench]
    fn leap4_bench(b: &mut Bencher) {
        b.iter( || {
            (0..1200).fold(0, |cnt, year| if is_leap_year4(year) {cnt + 1} else {cnt})
        })
    }

}
