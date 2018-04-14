use super::{N, P2, V2, Curve, FiniteCurve, RoughlyComparable, THICKNESS};
use super::primitives::Segment;
use super::intersect::{Intersect, Intersection};
use ordered_float::OrderedFloat;

type ScannerFn<'a> = fn(&mut StartOffsetState, &'a Segment) -> Option<(&'a Segment, N)>;
type ScanIter<'a> = ::std::iter::Scan<
    ::std::slice::Iter<'a, Segment>,
    StartOffsetState,
    ScannerFn<'a>,
>;

#[derive(Debug)]
pub enum PathError {
    EmptyPath,
    NotContinuous,
}

pub trait Path: Sized + Clone {
    fn segments(&self) -> &[Segment];
    fn new_unchecked(segments: Vec<Segment>) -> Self;
    fn new(segments: Vec<Segment>) -> Result<Self, PathError> {
        if segments.is_empty() {
            Result::Err(PathError::EmptyPath)
        } else {
            let continuous = segments.windows(2).all(|seg_pair| {
                seg_pair[0].end().is_roughly_within(
                    seg_pair[1].start(),
                    THICKNESS,
                )
            });

            if !continuous {
                Result::Err(PathError::NotContinuous)
            } else {
                Result::Ok(Self::new_unchecked(segments))
            }
        }

    }

    fn new_welded(mut segments: Vec<Segment>) -> Result<Self, PathError> {
        if segments.is_empty() {
            Result::Err(PathError::EmptyPath)
        } else {
            let probably_closed = segments.last().unwrap().end().is_roughly_within(
                segments
                    .first()
                    .unwrap()
                    .start(),
                THICKNESS * 3.0,
            );

            if probably_closed {
                let first_again = segments[0].clone();
                segments.push(first_again);
            }

            let mut welded_segments: Vec<Segment> = segments
                .windows(2)
                .map(|seg_pair| if seg_pair[0].is_linear() {
                    Segment::line(seg_pair[0].start(), seg_pair[1].start())
                        .expect("Welding should always work for lines")
                } else {
                    Segment::arc_with_direction(
                        seg_pair[0].start(),
                        seg_pair[0].start_direction(),
                        seg_pair[1].start(),
                    ).expect("Welding should always work for arcs")
                })
                .collect();

            if !probably_closed {
                welded_segments.push(segments.last().cloned().unwrap())
            }

            Self::new(welded_segments)
        }
    }

    fn scan_segments<'a>(
        start_offset: &mut StartOffsetState,
        segment: &'a Segment,
    ) -> Option<(&'a Segment, N)> {
        let pair = (segment, start_offset.0);
        start_offset.0 += segment.length;
        Some(pair)
    }

    fn segments_with_start_offsets(&self) -> ScanIter {
        self.segments().into_iter().scan(
            StartOffsetState(0.0),
            Self::scan_segments,
        )
    }

    fn find_on_segment(&self, distance: N) -> Option<(&Segment, N)> {
        let mut distance_covered = 0.0;
        for segment in self.segments().iter() {
            let new_distance_covered = distance_covered + segment.length();
            if new_distance_covered > distance {
                return Some((segment, distance - distance_covered));
            }
            distance_covered = new_distance_covered;
        }
        None
    }

    // TODO: move this to shape
    fn contains(&self, point: P2) -> bool {
        let ray = Segment::line(point, P2::new(point.x + 10000000000.0, point.y))
            .expect("Ray should be valid");
        (self, &Self::new_unchecked(vec![ray])).intersect().len() % 2 == 1
    }

    fn self_intersections(&self) -> Vec<Intersection> {
        self.segments_with_start_offsets()
            .enumerate()
            .flat_map(|(i, (segment_a, offset_a))| {
                self.segments_with_start_offsets()
                    .skip(i + 1)
                    .flat_map(|(segment_b, offset_b)| {
                        (segment_a, segment_b)
                            .intersect()
                            .into_iter()
                            .filter_map(|intersection| if intersection.along_a.is_roughly_within(
                                0.0,
                                THICKNESS,
                            ) ||
                                intersection.along_a.is_roughly_within(
                                    segment_a.length(),
                                    THICKNESS,
                                ) ||
                                intersection.along_b.is_roughly_within(0.0, THICKNESS) ||
                                intersection.along_b.is_roughly_within(
                                    segment_b.length(),
                                    THICKNESS,
                                )
                            {
                                None
                            } else {
                                Some(Intersection {
                                    position: intersection.position,
                                    along_a: offset_a + intersection.along_a,
                                    along_b: offset_b + intersection.along_b,
                                })
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn is_closed(&self) -> bool {
        self.segments().last().unwrap().end().is_roughly_within(
            self.segments()
                .first()
                .unwrap()
                .start(),
            THICKNESS,
        )
    }


    fn concat(&self, other: &Self) -> Result<Self, PathError> {
        // TODO: somehow change this to move self and other into here
        // but then segments would have to return [Segment], possible?
        if self.end().is_roughly_within(other.start(), THICKNESS) {
            Ok(Self::new_unchecked(
                self.segments()
                    .iter()
                    .chain(other.segments())
                    .cloned()
                    .collect(),
            ))
        } else {
            Err(PathError::NotContinuous)
        }
    }

    fn to_svg(&self) -> String {
        self.segments().iter().map(Segment::to_svg).collect()
    }
}

pub struct StartOffsetState(N);

impl<T: Path> FiniteCurve for T {
    fn length(&self) -> N {
        self.segments()
            .into_iter()
            .map(|segment| segment.length())
            .fold(0.0, ::std::ops::Add::add)
    }

    fn along(&self, distance: N) -> P2 {
        match self.find_on_segment(distance) {
            Some((segment, distance_on_segment)) => segment.along(distance_on_segment),
            None => {
                if distance < 0.0 {
                    self.segments()[0].start
                } else {
                    self.segments().last().unwrap().end
                }
            }
        }
    }

    fn direction_along(&self, distance: N) -> V2 {
        match self.find_on_segment(distance) {
            Some((segment, distance_on_segment)) => segment.direction_along(distance_on_segment),
            None => {
                if distance < 0.0 {
                    self.segments()[0].start_direction()
                } else {
                    self.segments().last().unwrap().end_direction()
                }
            }
        }
    }

    fn start(&self) -> P2 {
        self.segments()[0].start()
    }

    fn start_direction(&self) -> V2 {
        self.segments()[0].start_direction()
    }

    fn end(&self) -> P2 {
        self.segments().last().unwrap().end()
    }

    fn end_direction(&self) -> V2 {
        self.segments().last().unwrap().end_direction()
    }

    fn reverse(&self) -> Self {
        Self::new_unchecked(self.segments().iter().rev().map(Segment::reverse).collect())
    }

    fn subsection(&self, start: N, end: N) -> Option<T> {
        if start > end + THICKNESS && self.is_closed() {
            let maybe_first_half = self.subsection(start, self.length());
            let maybe_second_half = self.subsection(0.0, end);

            match (maybe_first_half, maybe_second_half) {
                (Some(first_half), Some(second_half)) => {
                    Some(first_half.concat(&second_half).expect(
                        "Closed path, should always be continous",
                    ))
                }
                (Some(first_half), None) => Some(first_half),
                (None, Some(second_half)) => Some(second_half),
                _ => None,
            }
        } else {
            let segments = self.segments_with_start_offsets()
                .filter_map(|pair: (&Segment, N)| {
                    let (segment, start_offset) = pair;
                    let end_offset = start_offset + segment.length;
                    if start_offset > end || end_offset < start {
                        None
                    } else {
                        segment.subsection(start - start_offset, end - start_offset)
                    }
                })
                .collect::<Vec<_>>();
            T::new(segments).ok()
        }

    }

    fn shift_orthogonally(&self, shift_to_right: N) -> Option<Self> {
        let segments = self.segments()
            .iter()
            .filter_map(|segment| segment.shift_orthogonally(shift_to_right))
            .collect::<Vec<_>>();
        let mut glued_segments = Vec::new();
        let mut window_segments_iter = segments.iter().peekable();
        while let Some(segment) = window_segments_iter.next() {
            glued_segments.push(*segment);
            match window_segments_iter.peek() {
                Some(next_segment) => {
                    if !segment.end().is_roughly_within(
                        next_segment.start(),
                        THICKNESS,
                    )
                    {
                        glued_segments.push(Segment::line(segment.end(), next_segment.start())?);
                    }
                }
                None => break,
            }
        }
        if glued_segments.is_empty() {
            None
        } else {
            let was_closed = self.end().is_roughly_within(self.start(), THICKNESS);
            let new_end = glued_segments.last().unwrap().end();
            let new_start = glued_segments[0].start();
            if was_closed && !new_end.is_roughly_within(new_start, THICKNESS) {
                glued_segments.push(Segment::line(new_end, new_start)?);
            }
            Some(Self::new(glued_segments).unwrap())
        }
    }
}

impl<T: Path> Curve for T {
    // TODO: this can be really buggy/unexpected
    fn project_with_tolerance(&self, point: P2, tolerance: N) -> Option<N> {
        self.segments_with_start_offsets()
            .filter_map(|pair: (&Segment, N)| {
                let (segment, start_offset) = pair;
                segment.project_with_tolerance(point, tolerance).map(
                    |offset| {
                        offset + start_offset
                    },
                )
            })
            .min_by_key(|offset| OrderedFloat((self.along(*offset) - point).norm()))
    }

    fn includes(&self, point: P2) -> bool {
        self.segments().into_iter().any(
            |segment| segment.includes(point),
        )
    }

    fn distance_to(&self, point: P2) -> N {
        if let Some(offset) = self.project(point) {
            (point - self.along(offset)).norm()
        } else {
            *::std::cmp::min(
                OrderedFloat((point - self.start()).norm()),
                OrderedFloat((point - self.end()).norm()),
            )
        }
    }
}

impl<'a, T: Path> RoughlyComparable for &'a T {
    fn is_roughly_within(&self, other: &T, tolerance: N) -> bool {
        self.segments().len() == other.segments().len() &&
            self.segments().iter().zip(other.segments().iter()).all(
                |(segment_1, segment_2)| segment_1.is_roughly_within(segment_2, tolerance),
            )
    }
}

use ncollide_transformation::convex_hull2_idx;

pub fn convex_hull<P: Path>(points: &[P2]) -> P {
    let mut hull_indices = convex_hull2_idx(points);
    let first_index = hull_indices[0];
    hull_indices.push(first_index);
    P::new(
        hull_indices
            .windows(2)
            .filter_map(|idx_window| {
                let (point_1, point_2) = (points[idx_window[0]], points[idx_window[1]]);
                if point_1.is_roughly_within(point_2, ::primitives::MIN_START_TO_END) {
                    None
                } else {
                    Segment::line(point_1, point_2)
                }
            })
            .collect(),
    ).unwrap()
}

#[derive(Clone)]
pub struct VecPath(Vec<Segment>);

impl Path for VecPath {
    fn segments(&self) -> &[Segment] {
        &self.0
    }

    fn new_unchecked(vec: Vec<Segment>) -> Self {
        VecPath(vec)
    }
}