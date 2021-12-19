// --- Day 19: Beacon Scanner ---

// As your probe drifted down through this area, it released an assortment of
// beacons and scanners into the water. It's difficult to navigate in the pitch
// black open waters of the ocean trench, but if you can build a map of the
// trench using data from the scanners, you should be able to safely reach the
// bottom.

// The beacons and scanners float motionless in the water; they're designed to
// maintain the same position for long periods of time. Each scanner is capable
// of detecting all beacons in a large cube centered on the scanner; beacons
// that are at most 1000 units away from the scanner in each of the three axes
// (x, y, and z) have their precise position determined relative to the scanner.
// However, scanners cannot detect other scanners. The submarine has
// automatically summarized the relative positions of beacons detected by each
// scanner (your puzzle input).

// For example, if a scanner is at x,y,z coordinates 500,0,-500 and there are
// beacons at -500,1000,-1500 and 1501,0,-500, the scanner could report that the
// first beacon is at -1000,1000,-1000 (relative to the scanner) but would not
// detect the second beacon at all.

// Unfortunately, while each scanner can report the positions of all detected
// beacons relative to itself, the scanners do not know their own position.
// You'll need to determine the positions of the beacons and scanners yourself.

// The scanners and beacons map a single contiguous 3d region. This region can
// be reconstructed by finding pairs of scanners that have overlapping detection
// regions such that there are at least 12 beacons that both scanners detect
// within the overlap. By establishing 12 common beacons, you can precisely
// determine where the scanners are relative to each other, allowing you to
// reconstruct the beacon map one scanner at a time.

// For a moment, consider only two dimensions. Suppose you have the following
// scanner reports:

// --- scanner 0 ---
// 0,2
// 4,1
// 3,3

// --- scanner 1 ---
// -1,-1
// -5,0
// -2,1

// Drawing x increasing rightward, y increasing upward, scanners as S, and
// beacons as B, scanner 0 detects this:

// ...B.
// B....
// ....B
// S....

// Scanner 1 detects this:

// ...B..
// B....S
// ....B.

// For this example, assume scanners only need 3 overlapping beacons. Then, the
// beacons visible to both scanners overlap to produce the following complete
// map:

// ...B..
// B....S
// ....B.
// S.....

// Unfortunately, there's a second problem: the scanners also don't know their
// rotation or facing direction. Due to magnetic alignment, each scanner is
// rotated some integer number of 90-degree turns around all of the x, y, and z
// axes. That is, one scanner might call a direction positive x, while another
// scanner might call that direction negative y. Or, two scanners might agree on
// which direction is positive x, but one scanner might be upside-down from the
// perspective of the other scanner. In total, each scanner could be in any of
// 24 different orientations: facing positive or negative x, y, or z, and
// considering any of four directions "up" from that facing.

// For example, here is an arrangement of beacons as seen from a scanner in the
// same position but in different orientations:

// --- scanner 0 ---
// -1,-1,1
// -2,-2,2
// -3,-3,3
// -2,-3,1
// 5,6,-4
// 8,0,7

// --- scanner 0 ---
// 1,-1,1
// 2,-2,2
// 3,-3,3
// 2,-1,3
// -5,4,-6
// -8,-7,0

// --- scanner 0 ---
// -1,-1,-1
// -2,-2,-2
// -3,-3,-3
// -1,-3,-2
// 4,6,5
// -7,0,8

// --- scanner 0 ---
// 1,1,-1
// 2,2,-2
// 3,3,-3
// 1,3,-2
// -4,-6,5
// 7,0,8

// --- scanner 0 ---
// 1,1,1
// 2,2,2
// 3,3,3
// 3,1,2
// -6,-4,-5
// 0,7,-8

// By finding pairs of scanners that both see at least 12 of the same beacons,
// you can assemble the entire map. For example, consider the following report:

// --- scanner 0 ---
// 404,-588,-901
// 528,-643,409
// -838,591,734
// 390,-675,-793
// -537,-823,-458
// -485,-357,347
// -345,-311,381
// -661,-816,-575
// -876,649,763
// -618,-824,-621
// 553,345,-567
// 474,580,667
// -447,-329,318
// -584,868,-557
// 544,-627,-890
// 564,392,-477
// 455,729,728
// -892,524,684
// -689,845,-530
// 423,-701,434
// 7,-33,-71
// 630,319,-379
// 443,580,662
// -789,900,-551
// 459,-707,401

// --- scanner 1 ---
// 686,422,578
// 605,423,415
// 515,917,-361
// -336,658,858
// 95,138,22
// -476,619,847
// -340,-569,-846
// 567,-361,727
// -460,603,-452
// 669,-402,600
// 729,430,532
// -500,-761,534
// -322,571,750
// -466,-666,-811
// -429,-592,574
// -355,545,-477
// 703,-491,-529
// -328,-685,520
// 413,935,-424
// -391,539,-444
// 586,-435,557
// -364,-763,-893
// 807,-499,-711
// 755,-354,-619
// 553,889,-390

// --- scanner 2 ---
// 649,640,665
// 682,-795,504
// -784,533,-524
// -644,584,-595
// -588,-843,648
// -30,6,44
// -674,560,763
// 500,723,-460
// 609,671,-379
// -555,-800,653
// -675,-892,-343
// 697,-426,-610
// 578,704,681
// 493,664,-388
// -671,-858,530
// -667,343,800
// 571,-461,-707
// -138,-166,112
// -889,563,-600
// 646,-828,498
// 640,759,510
// -630,509,768
// -681,-892,-333
// 673,-379,-804
// -742,-814,-386
// 577,-820,562

// --- scanner 3 ---
// -589,542,597
// 605,-692,669
// -500,565,-823
// -660,373,557
// -458,-679,-417
// -488,449,543
// -626,468,-788
// 338,-750,-386
// 528,-832,-391
// 562,-778,733
// -938,-730,414
// 543,643,-506
// -524,371,-870
// 407,773,750
// -104,29,83
// 378,-903,-323
// -778,-728,485
// 426,699,580
// -438,-605,-362
// -469,-447,-387
// 509,732,623
// 647,635,-688
// -868,-804,481
// 614,-800,639
// 595,780,-596

// --- scanner 4 ---
// 727,592,562
// -293,-554,779
// 441,611,-461
// -714,465,-776
// -743,427,-804
// -660,-479,-426
// 832,-632,460
// 927,-485,-438
// 408,393,-506
// 466,436,-512
// 110,16,151
// -258,-428,682
// -393,719,612
// -211,-452,876
// 808,-476,-593
// -575,615,604
// -485,667,467
// -680,325,-822
// -627,-443,-432
// 872,-547,-609
// 833,512,582
// 807,604,487
// 839,-516,451
// 891,-625,532
// -652,-548,-490
// 30,-46,-14

// Because all coordinates are relative, in this example, all "absolute"
// positions will be expressed relative to scanner 0 (using the orientation of
// scanner 0 and as if scanner 0 is at coordinates 0,0,0).

// Scanners 0 and 1 have overlapping detection cubes; the 12 beacons they both
// detect (relative to scanner 0) are at the following coordinates:

// -618,-824,-621
// -537,-823,-458
// -447,-329,318
// 404,-588,-901
// 544,-627,-890
// 528,-643,409
// -661,-816,-575
// 390,-675,-793
// 423,-701,434
// -345,-311,381
// 459,-707,401
// -485,-357,347

// These same 12 beacons (in the same order) but from the perspective of scanner
// 1 are:

// 686,422,578
// 605,423,415
// 515,917,-361
// -336,658,858
// -476,619,847
// -460,603,-452
// 729,430,532
// -322,571,750
// -355,545,-477
// 413,935,-424
// -391,539,-444
// 553,889,-390

// Because of this, scanner 1 must be at 68,-1246,-43 (relative to scanner 0).

// Scanner 4 overlaps with scanner 1; the 12 beacons they both detect (relative
// to scanner 0) are:

// 459,-707,401
// -739,-1745,668
// -485,-357,347
// 432,-2009,850
// 528,-643,409
// 423,-701,434
// -345,-311,381
// 408,-1815,803
// 534,-1912,768
// -687,-1600,576
// -447,-329,318
// -635,-1737,486

// So, scanner 4 is at -20,-1133,1061 (relative to scanner 0).

// Following this process, scanner 2 must be at 1105,-1205,1229 (relative to
// scanner 0) and scanner 3 must be at -92,-2380,-20 (relative to scanner 0).

// The full list of beacons (relative to scanner 0) is:

// -892,524,684
// -876,649,763
// -838,591,734
// -789,900,-551
// -739,-1745,668
// -706,-3180,-659
// -697,-3072,-689
// -689,845,-530
// -687,-1600,576
// -661,-816,-575
// -654,-3158,-753
// -635,-1737,486
// -631,-672,1502
// -624,-1620,1868
// -620,-3212,371
// -618,-824,-621
// -612,-1695,1788
// -601,-1648,-643
// -584,868,-557
// -537,-823,-458
// -532,-1715,1894
// -518,-1681,-600
// -499,-1607,-770
// -485,-357,347
// -470,-3283,303
// -456,-621,1527
// -447,-329,318
// -430,-3130,366
// -413,-627,1469
// -345,-311,381
// -36,-1284,1171
// -27,-1108,-65
// 7,-33,-71
// 12,-2351,-103
// 26,-1119,1091
// 346,-2985,342
// 366,-3059,397
// 377,-2827,367
// 390,-675,-793
// 396,-1931,-563
// 404,-588,-901
// 408,-1815,803
// 423,-701,434
// 432,-2009,850
// 443,580,662
// 455,729,728
// 456,-540,1869
// 459,-707,401
// 465,-695,1988
// 474,580,667
// 496,-1584,1900
// 497,-1838,-617
// 527,-524,1933
// 528,-643,409
// 534,-1912,768
// 544,-627,-890
// 553,345,-567
// 564,392,-477
// 568,-2007,-577
// 605,-1665,1952
// 612,-1593,1893
// 630,319,-379
// 686,-3108,-505
// 776,-3184,-501
// 846,-3110,-434
// 1135,-1161,1235
// 1243,-1093,1063
// 1660,-552,429
// 1693,-557,386
// 1735,-437,1738
// 1749,-1800,1813
// 1772,-405,1572
// 1776,-675,371
// 1779,-442,1789
// 1780,-1548,337
// 1786,-1538,337
// 1847,-1591,415
// 1889,-1729,1762
// 1994,-1805,1792

// In total, there are 79 beacons.

// Assemble the full map of beacons. How many beacons are there?

// --- Part Two ---

// Sometimes, it's a good idea to appreciate just how big the ocean is. Using
// the Manhattan distance, how far apart do the scanners get?

// In the above example, scanners 2 (1105,-1205,1229) and 3 (-92,-2380,-20) are
// the largest Manhattan distance apart. In total, they are 1197 + 1175 + 1249 =
// 3621 units apart.

// What is the largest Manhattan distance between any two scanners?

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub fn relative_to(
    beacons: &[(isize, isize, isize)],
    beacon: (isize, isize, isize),
) -> impl Iterator<Item = (isize, isize, isize)> + '_ {
    let (x_, y_, z_) = beacon;
    beacons
        .iter()
        .map(move |&(x, y, z)| (x - x_, y - y_, z - z_))
}

type Mat3 = [[isize; 3]; 3];
type Vec3 = [isize; 3];

#[allow(clippy::needless_range_loop)]
pub fn mat_mul(a: Mat3, b: Mat3) -> Mat3 {
    let mut res = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                res[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    res
}

#[allow(clippy::needless_range_loop)]
pub fn change_basis(point: Vec3, rotation: Mat3) -> Vec3 {
    let mut res = [0, 0, 0];
    for i in 0..3 {
        for k in 0..3 {
            res[i] += rotation[i][k] * point[k]
        }
    }

    res
}

pub fn negate_vec(v: Vec3) -> Vec3 {
    [-v[0], -v[1], -v[2]]
}

pub fn add_vec(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn rotations() -> impl Iterator<Item = Mat3> {
    const ROT_X: Mat3 = [[1, 0, 0], [0, 0, -1], [0, 1, 0]];
    const ROT_Y: Mat3 = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]];
    const ROT_Z: Mat3 = [[0, -1, 0], [1, 0, 0], [0, 0, 1]];
    const I: Mat3 = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    const FLIP_X: Mat3 = [[-1, 0, 0], [0, 1, 0], [0, 0, 1]];
    const FLIP_Y: Mat3 = [[1, 0, 0], [0, -1, 0], [0, 0, 1]];
    const FLIP_Z: Mat3 = [[1, 0, 0], [0, 1, 0], [0, 0, -1]];

    let mut rotations = vec![];

    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                for fx in 0..2 {
                    for fy in 0..2 {
                        for fz in 0..2 {
                            let mut m = I;
                            for _ in 0..x {
                                m = mat_mul(m, ROT_X);
                            }
                            for _ in 0..y {
                                m = mat_mul(m, ROT_Y);
                            }
                            for _ in 0..z {
                                m = mat_mul(m, ROT_Z);
                            }
                            for _ in 0..fx {
                                m = mat_mul(m, FLIP_X);
                            }
                            for _ in 0..fy {
                                m = mat_mul(m, FLIP_Y);
                            }
                            for _ in 0..fz {
                                m = mat_mul(m, FLIP_Z);
                            }
                            rotations.push(m);
                        }
                    }
                }
            }
        }
    }

    rotations.into_iter()
}

fn common_enough(a: &[Vec3], b: &[Vec3]) -> Option<(Vec<Vec3>, Vec3)> {
    let points_in_a = a.iter().copied().collect::<HashSet<Vec3>>();

    for rot in rotations() {
        let new_b = b.iter().map(|v| change_basis(*v, rot)).collect::<Vec<_>>();

        for a_ in a {
            for b in &new_b {
                // Try every possible basis point :(
                let delta = add_vec(*a_, negate_vec(*b));
                let b2_iter = new_b.iter().map(move |v| add_vec(*v, delta));

                if b2_iter.clone().filter(|v| points_in_a.contains(v)).count() >= 12 {
                    return Some((b2_iter.collect(), delta));
                }
            }
        }
    }
    None
}

fn shared(s: &str) -> (HashMap<String, Arc<Vec<Vec3>>>, Vec<Vec3>) {
    let mut scanners = HashMap::new();
    let mut iter = s.lines();
    let mut cur_scanner: (String, Vec<Vec3>) = (iter.next().unwrap().to_owned(), vec![]);

    let mut done_scanners = HashSet::new();
    let mut remapped_scanners = HashMap::new();

    for line in iter {
        if line.is_empty() {
            continue;
        } else if line.starts_with("---") {
            scanners.insert(cur_scanner.0, Arc::new(cur_scanner.1));
            cur_scanner = (line.to_owned(), vec![]);
        } else {
            let mut seq = line.split(',').map(|v| v.parse().unwrap());
            cur_scanner.1.push([
                seq.next().unwrap(),
                seq.next().unwrap(),
                seq.next().unwrap(),
            ]);
        }
    }
    scanners.insert(cur_scanner.0, Arc::new(cur_scanner.1));

    remapped_scanners.insert(
        "--- scanner 0 ---".to_owned(),
        Arc::clone(&scanners["--- scanner 0 ---"]),
    );

    let mut deltas = vec![];

    let scanner_keys = scanners.keys().cloned().collect::<Vec<_>>();

    while done_scanners.len() < scanners.len() {
        for scanner in &scanner_keys {
            if remapped_scanners.contains_key(scanner) && !done_scanners.contains(scanner) {
                eprintln!("{}", scanner);

                let mut work = vec![];
                for scanner2 in &scanner_keys {
                    if scanner != scanner2 && !remapped_scanners.contains_key(scanner2) {
                        let scanner2 = scanner2.clone();
                        work.push((scanner, scanner2));
                    }
                }

                // This is not the idiomatic way to parallelize things in Rust,
                // but this is a no-libraries solution so no threadpool library
                // and no mpmc. Using a condvar seems like overkill.
                for group in work.chunks(8) {
                    let mut threads = vec![];
                    for (scanner, scanner2) in group {
                        let a = Arc::clone(&remapped_scanners[*scanner]);
                        let b = Arc::clone(&scanners[scanner2]);
                        let scanner2 = scanner2.clone();

                        threads.push(std::thread::spawn(move || {
                            common_enough(&a, &b).map(|(a, b)| (scanner2, a, b))
                        }));
                    }
                    for thread in threads {
                        if let Some((scanner2, v, delta)) = thread.join().unwrap() {
                            remapped_scanners.insert(scanner2.to_owned(), Arc::new(v));
                            deltas.push(delta);
                        }
                    }
                }

                done_scanners.insert(scanner);
                eprintln!("done: {}/{}", done_scanners.len(), scanners.len());
            }
        }
    }

    (remapped_scanners, deltas)
}

pub fn part_1(s: &str) -> usize {
    let (remapped, _) = shared(s);
    let mut all_points = HashSet::new();

    for points in remapped.values() {
        all_points.extend(points.iter().copied());
    }

    all_points.len()
}

pub fn part_2(s: &str) -> isize {
    let (_, deltas) = shared(s);

    let mut max_dist = 0;
    for a in &deltas {
        for b in &deltas {
            let mut dist = 0;
            for i in 0..3 {
                dist += a[i].max(b[i]) - a[i].min(b[i]);
            }
            max_dist = dist.max(max_dist);
        }
    }

    max_dist
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn test_day_19_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 79);
    }

    #[test]
    fn test_day_19_part_1() {
        assert_eq!(part_1(include_str!("input/day_19.txt")), 378);
    }

    #[test]
    fn test_day_19_example_part_2() {
        assert_eq!(part_2(EXAMPLE), 3621);
    }

    #[test]
    fn test_day_19_part_2() {
        let answer = part_2(include_str!("input/day_19.txt"));
        assert_eq!(answer, 13148);
    }

    const EXAMPLE: &str = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#;
}
