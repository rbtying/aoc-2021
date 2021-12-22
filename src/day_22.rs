// --- Day 22: Reactor Reboot ---

// Operating at these extreme ocean depths has overloaded the submarine's
// reactor; it needs to be rebooted.

// The reactor core is made up of a large 3-dimensional grid made up entirely of
// cubes, one cube per integer 3-dimensional coordinate (x,y,z). Each cube can
// be either on or off; at the start of the reboot process, they are all off.
// (Could it be an old model of a reactor you've seen before?)

// To reboot the reactor, you just need to set all of the cubes to either on or
// off by following a list of reboot steps (your puzzle input). Each step
// specifies a cuboid (the set of all cubes that have coordinates which fall
// within ranges for x, y, and z) and whether to turn all of the cubes in that
// cuboid on or off.

// For example, given these reboot steps:

// on x=10..12,y=10..12,z=10..12
// on x=11..13,y=11..13,z=11..13
// off x=9..11,y=9..11,z=9..11
// on x=10..10,y=10..10,z=10..10

// The first step (on x=10..12,y=10..12,z=10..12) turns on a 3x3x3 cuboid
// consisting of 27 cubes:

//     10,10,10
//     10,10,11
//     10,10,12
//     10,11,10
//     10,11,11
//     10,11,12
//     10,12,10
//     10,12,11
//     10,12,12
//     11,10,10
//     11,10,11
//     11,10,12
//     11,11,10
//     11,11,11
//     11,11,12
//     11,12,10
//     11,12,11
//     11,12,12
//     12,10,10
//     12,10,11
//     12,10,12
//     12,11,10
//     12,11,11
//     12,11,12
//     12,12,10
//     12,12,11
//     12,12,12

// The second step (on x=11..13,y=11..13,z=11..13) turns on a 3x3x3 cuboid that
// overlaps with the first. As a result, only 19 additional cubes turn on; the
// rest are already on from the previous step:

//     11,11,13
//     11,12,13
//     11,13,11
//     11,13,12
//     11,13,13
//     12,11,13
//     12,12,13
//     12,13,11
//     12,13,12
//     12,13,13
//     13,11,11
//     13,11,12
//     13,11,13
//     13,12,11
//     13,12,12
//     13,12,13
//     13,13,11
//     13,13,12
//     13,13,13

// The third step (off x=9..11,y=9..11,z=9..11) turns off a 3x3x3 cuboid that
// overlaps partially with some cubes that are on, ultimately turning off 8
// cubes:

//     10,10,10
//     10,10,11
//     10,11,10
//     10,11,11
//     11,10,10
//     11,10,11
//     11,11,10
//     11,11,11

// The final step (on x=10..10,y=10..10,z=10..10) turns on a single cube,
// 10,10,10. After this last step, 39 cubes are on.

// The initialization procedure only uses cubes that have x, y, and z positions
// of at least -50 and at most 50. For now, ignore cubes outside this region.

// Here is a larger example:

// on x=-20..26,y=-36..17,z=-47..7
// on x=-20..33,y=-21..23,z=-26..28
// on x=-22..28,y=-29..23,z=-38..16
// on x=-46..7,y=-6..46,z=-50..-1
// on x=-49..1,y=-3..46,z=-24..28
// on x=2..47,y=-22..22,z=-23..27
// on x=-27..23,y=-28..26,z=-21..29
// on x=-39..5,y=-6..47,z=-3..44
// on x=-30..21,y=-8..43,z=-13..34
// on x=-22..26,y=-27..20,z=-29..19
// off x=-48..-32,y=26..41,z=-47..-37
// on x=-12..35,y=6..50,z=-50..-2
// off x=-48..-32,y=-32..-16,z=-15..-5
// on x=-18..26,y=-33..15,z=-7..46
// off x=-40..-22,y=-38..-28,z=23..41
// on x=-16..35,y=-41..10,z=-47..6
// off x=-32..-23,y=11..30,z=-14..3
// on x=-49..-5,y=-3..45,z=-29..18
// off x=18..30,y=-20..-8,z=-3..13
// on x=-41..9,y=-7..43,z=-33..15
// on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
// on x=967..23432,y=45373..81175,z=27513..53682

// The last two steps are fully outside the initialization procedure area; all
// other steps are fully within it. After executing these steps in the
// initialization procedure region, 590784 cubes are on.

// Execute the reboot steps. Afterward, considering only cubes in the region
// x=-50..50,y=-50..50,z=-50..50, how many cubes are on?

// --- Part Two ---

// Now that the initialization procedure is complete, you can reboot the
// reactor.

// Starting with all cubes off, run all of the reboot steps for all cubes in the
// reactor.

// Consider the following reboot steps:

// on x=-5..47,y=-31..22,z=-19..33
// on x=-44..5,y=-27..21,z=-14..35
// on x=-49..-1,y=-11..42,z=-10..38
// on x=-20..34,y=-40..6,z=-44..1
// off x=26..39,y=40..50,z=-2..11
// on x=-41..5,y=-41..6,z=-36..8
// off x=-43..-33,y=-45..-28,z=7..25
// on x=-33..15,y=-32..19,z=-34..11
// off x=35..47,y=-46..-34,z=-11..5
// on x=-14..36,y=-6..44,z=-16..29
// on x=-57795..-6158,y=29564..72030,z=20435..90618
// on x=36731..105352,y=-21140..28532,z=16094..90401
// on x=30999..107136,y=-53464..15513,z=8553..71215
// on x=13528..83982,y=-99403..-27377,z=-24141..23996
// on x=-72682..-12347,y=18159..111354,z=7391..80950
// on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
// on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
// on x=-52752..22273,y=-49450..9096,z=54442..119054
// on x=-29982..40483,y=-108474..-28371,z=-24328..38471
// on x=-4958..62750,y=40422..118853,z=-7672..65583
// on x=55694..108686,y=-43367..46958,z=-26781..48729
// on x=-98497..-18186,y=-63569..3412,z=1232..88485
// on x=-726..56291,y=-62629..13224,z=18033..85226
// on x=-110886..-34664,y=-81338..-8658,z=8914..63723
// on x=-55829..24974,y=-16897..54165,z=-121762..-28058
// on x=-65152..-11147,y=22489..91432,z=-58782..1780
// on x=-120100..-32970,y=-46592..27473,z=-11695..61039
// on x=-18631..37533,y=-124565..-50804,z=-35667..28308
// on x=-57817..18248,y=49321..117703,z=5745..55881
// on x=14781..98692,y=-1341..70827,z=15753..70151
// on x=-34419..55919,y=-19626..40991,z=39015..114138
// on x=-60785..11593,y=-56135..2999,z=-95368..-26915
// on x=-32178..58085,y=17647..101866,z=-91405..-8878
// on x=-53655..12091,y=50097..105568,z=-75335..-4862
// on x=-111166..-40997,y=-71714..2688,z=5609..50954
// on x=-16602..70118,y=-98693..-44401,z=5197..76897
// on x=16383..101554,y=4615..83635,z=-44907..18747
// off x=-95822..-15171,y=-19987..48940,z=10804..104439
// on x=-89813..-14614,y=16069..88491,z=-3297..45228
// on x=41075..99376,y=-20427..49978,z=-52012..13762
// on x=-21330..50085,y=-17944..62733,z=-112280..-30197
// on x=-16478..35915,y=36008..118594,z=-7885..47086
// off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
// off x=2032..69770,y=-71013..4824,z=7471..94418
// on x=43670..120875,y=-42068..12382,z=-24787..38892
// off x=37514..111226,y=-45862..25743,z=-16714..54663
// off x=25699..97951,y=-30668..59918,z=-15349..69697
// off x=-44271..17935,y=-9516..60759,z=49131..112598
// on x=-61695..-5813,y=40978..94975,z=8655..80240
// off x=-101086..-9439,y=-7088..67543,z=33935..83858
// off x=18020..114017,y=-48931..32606,z=21474..89843
// off x=-77139..10506,y=-89994..-18797,z=-80..59318
// off x=8476..79288,y=-75520..11602,z=-96624..-24783
// on x=-47488..-1262,y=24338..100707,z=16292..72967
// off x=-84341..13987,y=2429..92914,z=-90671..-1318
// off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
// off x=-27365..46395,y=31009..98017,z=15428..76570
// off x=-70369..-16548,y=22648..78696,z=-1892..86821
// on x=-53470..21291,y=-120233..-33476,z=-44150..38147
// off x=-93533..-4276,y=-16170..68771,z=-104985..-24507

// After running the above reboot steps, 2758514936282235 cubes are on. (Just
// for fun, 474140 of those are also in the initialization procedure region.)

// Starting again with all cubes off, execute all reboot steps. Afterward,
// considering all cubes, how many cubes are on?
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Ranges {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

impl Ranges {
    fn _overlap(s1: isize, e1: isize, s2: isize, e2: isize) -> Option<(isize, isize)> {
        if !(e1 < s2 || e2 < s1) {
            Some((s1.max(s2), e1.min(e2)))
        } else {
            None
        }
    }

    fn volume(&self) -> isize {
        (self.x_max + 1 - self.x_min)
            * (self.y_max + 1 - self.y_min)
            * (self.z_max + 1 - self.z_min)
    }

    fn overlap(&self, other: Ranges) -> Option<Ranges> {
        let x_overlap = Self::_overlap(self.x_min, self.x_max, other.x_min, other.x_max);
        let y_overlap = Self::_overlap(self.y_min, self.y_max, other.y_min, other.y_max);
        let z_overlap = Self::_overlap(self.z_min, self.z_max, other.z_min, other.z_max);

        if let (Some((x_min, x_max)), Some((y_min, y_max)), Some((z_min, z_max))) =
            (x_overlap, y_overlap, z_overlap)
        {
            let r = Ranges {
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
            };
            if r.volume() > 0 {
                Some(r)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn subtract(&self, other: Ranges) -> Vec<Ranges> {
        let overlap = self.overlap(other);
        if let Some(v) = overlap {
            // If there's an overlapping component, we need to construct a
            // set of regions around those bounds. Worst case: overlap is
            // fully contained inside of the current range.

            let segments_x = [
                (self.x_min, v.x_min - 1),
                (v.x_min, v.x_max),
                (v.x_max + 1, self.x_max),
            ];
            let segments_y = [
                (self.y_min, v.y_min - 1),
                (v.y_min, v.y_max),
                (v.y_max + 1, self.y_max),
            ];
            let segments_z = [
                (self.z_min, v.z_min - 1),
                (v.z_min, v.z_max),
                (v.z_max + 1, self.z_max),
            ];

            let mut regions = vec![];

            for (x_min, x_max) in segments_x {
                for (y_min, y_max) in segments_y {
                    for (z_min, z_max) in segments_z {
                        let r = Ranges {
                            x_min,
                            x_max,
                            y_min,
                            y_max,
                            z_min,
                            z_max,
                        };
                        if r != v && r.volume() > 0 {
                            regions.push(r);
                        }
                    }
                }
            }

            regions
        } else {
            vec![*self]
        }
    }
}

struct IndexedRanges {
    max_id: usize,
    on: HashMap<usize, Ranges>,
    sorted_by_x_min: BTreeMap<isize, Vec<usize>>,
    sorted_by_y_min: BTreeMap<isize, Vec<usize>>,
    sorted_by_z_min: BTreeMap<isize, Vec<usize>>,
    sorted_by_x_max: BTreeMap<isize, Vec<usize>>,
    sorted_by_y_max: BTreeMap<isize, Vec<usize>>,
    sorted_by_z_max: BTreeMap<isize, Vec<usize>>,
}

impl IndexedRanges {
    pub fn new() -> Self {
        Self {
            max_id: 0,
            on: HashMap::new(),
            sorted_by_x_min: BTreeMap::new(),
            sorted_by_y_min: BTreeMap::new(),
            sorted_by_z_min: BTreeMap::new(),
            sorted_by_x_max: BTreeMap::new(),
            sorted_by_y_max: BTreeMap::new(),
            sorted_by_z_max: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, r: Ranges) {
        let id = self.max_id;
        self.max_id += 1;

        self.on.insert(id, r);
        self.sorted_by_x_min.entry(r.x_min).or_default().push(id);
        self.sorted_by_y_min.entry(r.y_min).or_default().push(id);
        self.sorted_by_z_min.entry(r.z_min).or_default().push(id);
        self.sorted_by_x_max.entry(r.x_max).or_default().push(id);
        self.sorted_by_y_max.entry(r.y_max).or_default().push(id);
        self.sorted_by_z_max.entry(r.z_max).or_default().push(id);
    }

    pub fn find_overlaps(&mut self, r: Ranges) -> Vec<(usize, Ranges)> {
        let mut viable: HashSet<usize> = HashSet::new();

        viable.extend(
            self.sorted_by_x_min
                .range(&r.x_min..=&r.x_max)
                .flat_map(|(_, ids)| ids),
        );
        viable.extend(
            self.sorted_by_y_min
                .range(&r.y_min..=&r.y_max)
                .flat_map(|(_, ids)| ids),
        );
        viable.extend(
            self.sorted_by_z_min
                .range(&r.z_min..=&r.z_max)
                .flat_map(|(_, ids)| ids),
        );
        viable.extend(
            self.sorted_by_x_max
                .range(&r.x_min..=&r.x_max)
                .flat_map(|(_, ids)| ids),
        );
        viable.extend(
            self.sorted_by_y_max
                .range(&r.y_min..=&r.y_max)
                .flat_map(|(_, ids)| ids),
        );
        viable.extend(
            self.sorted_by_z_max
                .range(&r.z_min..=&r.z_max)
                .flat_map(|(_, ids)| ids),
        );

        viable
            .into_iter()
            .flat_map(|i| self.on[&i].overlap(r).map(|o| (i, o)))
            .collect()
    }

    pub fn remove(&mut self, id: usize) -> Ranges {
        let v = self.on.remove(&id).unwrap();
        self.sorted_by_x_min
            .get_mut(&v.x_min)
            .unwrap()
            .retain(|i| *i != id);
        self.sorted_by_y_min
            .get_mut(&v.y_min)
            .unwrap()
            .retain(|i| *i != id);
        self.sorted_by_z_min
            .get_mut(&v.z_min)
            .unwrap()
            .retain(|i| *i != id);
        self.sorted_by_x_max
            .get_mut(&v.x_max)
            .unwrap()
            .retain(|i| *i != id);
        self.sorted_by_y_max
            .get_mut(&v.y_max)
            .unwrap()
            .retain(|i| *i != id);
        self.sorted_by_z_max
            .get_mut(&v.z_max)
            .unwrap()
            .retain(|i| *i != id);
        v
    }

    pub fn contents(&self) -> impl Iterator<Item = Ranges> + '_ {
        self.on.values().copied()
    }
}

fn common(s: &str) -> Vec<Ranges> {
    let mut on_ranges = IndexedRanges::new();

    for line in s.lines() {
        let (onoff, ranges) = line.split_once(' ').unwrap();
        let on = match onoff {
            "on" => true,
            "off" => false,
            _ => unreachable!(),
        };

        let mut r = Ranges {
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
            z_min: 0,
            z_max: 0,
        };

        for coord_range in ranges.split(',') {
            let (axis, range_str) = coord_range.split_once('=').unwrap();
            let (min, max) = range_str.split_once("..").unwrap();

            match axis {
                "x" => {
                    r.x_min = min.parse().unwrap();
                    r.x_max = max.parse().unwrap();
                }
                "y" => {
                    r.y_min = min.parse().unwrap();
                    r.y_max = max.parse().unwrap();
                }
                "z" => {
                    r.z_min = min.parse().unwrap();
                    r.z_max = max.parse().unwrap();
                }
                _ => unreachable!(),
            }
        }

        let to_subtract = on_ranges.find_overlaps(r);

        if on {
            let mut to_turn_on = vec![r];

            for (_, sub) in to_subtract {
                to_turn_on = to_turn_on
                    .into_iter()
                    .flat_map(|r_| r_.subtract(sub))
                    .collect();
            }

            for v in to_turn_on {
                on_ranges.insert(v);
            }
        } else {
            for (idx, sub) in to_subtract {
                let v = on_ranges.remove(idx);
                for x in v.subtract(sub) {
                    on_ranges.insert(x);
                }
            }
        }
    }
    on_ranges.contents().collect()
}

pub fn part_1(s: &str) -> isize {
    let mut on_ranges = common(s);

    let limit = Ranges {
        x_min: -50,
        x_max: 50,
        y_min: -50,
        y_max: 50,
        z_min: -50,
        z_max: 50,
    };

    on_ranges = on_ranges
        .into_iter()
        .flat_map(|r| r.overlap(limit))
        .collect();

    on_ranges.iter().map(|r| r.volume()).sum::<isize>()
}

pub fn part_2(s: &str) -> isize {
    let on_ranges = common(s);
    on_ranges.iter().map(|r| r.volume()).sum::<isize>()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn test_day_22_example_part_1() {
        assert_eq!(part_1(EXAMPLE2), 39);
        assert_eq!(part_1(EXAMPLE), 590784);
    }

    #[test]
    fn test_day_22_part_1() {
        assert_eq!(part_1(include_str!("input/day_22.txt")), 545118);
    }

    #[test]
    fn test_day_22_example_part_2() {
        assert_eq!(part_2(EXAMPLE3), 2758514936282235);
    }

    #[test]
    fn test_day_22_part_2() {
        let answer = part_2(include_str!("input/day_22.txt"));
        assert_eq!(answer, 1227298136842375);
    }

    const EXAMPLE2: &str = r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#;

    const EXAMPLE: &str = r#"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"#;
    const EXAMPLE3: &str = r#"on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507"#;
}
