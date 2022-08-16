//! Contains implementation of graph leveling
//! Contains the implementation of Leveling
use crate::algorithm::spantree::Span;
use crate::data_structure::link_graph::LinkVertex;
use std::collections::HashSet;

/// The structure containing the levels of a graph
pub struct Leveling<T> {
    pub levels: Vec<HashSet<T>>,
}

impl Leveling<LinkVertex> {
    /// Returns a new Leveling of a graph along its span tree
    pub fn compute(span: Span<LinkVertex>) -> Self {
        let mut result = vec![];
        let mut level = HashSet::new();
        level.insert(span.root);
        while !level.is_empty() {
            result.push(level.clone());
            let mut new_level = HashSet::new();
            for v in &level {
                let children = span.downwards.get(v);
                if span.downwards.get(v).is_some() {
                    children.unwrap().iter().for_each(|v| {
                        new_level.insert(v.clone());
                    });
                }
            }
            level = new_level;
        }
        Leveling { levels: result }
    }

    /// Returns the number of levels
    pub fn size(&self) -> usize {
        self.levels.len()
    }

    /// Returns rings consisting of k levels.
    /// The last ring can contain less than k levels if the number of levels mod k is not 0.
    pub fn rings(&self, k: usize) -> Vec<HashSet<LinkVertex>> {
        let mut result = vec![];
        for c in self
            .levels
            .chunks(k)
            .collect::<Vec<&[HashSet<LinkVertex>]>>()
        {
            let mut union: HashSet<LinkVertex> = HashSet::new();
            for set in c {
                union.extend(set.clone());
            }
            result.push(union);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::leveling::Leveling;
    use crate::algorithm::spantree::Span;
    use crate::data_structure::link_graph::{LinkGraph, LinkVertex};
    use std::collections::HashSet;

    #[test]
    fn triangle() {
        let mut lg = LinkGraph::new();
        let lv0 = lg.new_vertex();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();

        let lt0 = lg.new_dart(lv1.clone(), lv0.clone(), None, None, None, None);
        let lof = lg.new_face(lt0.clone()); // Outer Face first

        let ld0 = lg.new_dart(
            lv0.clone(),
            lv1.clone(),
            None,
            None,
            Some(lt0.clone()),
            None,
        );
        let lf = lg.new_face(ld0.clone());

        let ld1 = lg.new_dart(
            lv1.clone(),
            lv2.clone(),
            Some(ld0.clone()),
            None,
            None,
            Some(lf.clone()),
        );
        let ld2 = lg.new_dart(
            lv2.clone(),
            lv0.clone(),
            Some(ld1.clone()),
            Some(ld0),
            None,
            Some(lf),
        );

        let lt2 = lg.new_dart(
            lv0.clone(),
            lv2.clone(),
            Some(lt0.clone()),
            None,
            Some(ld2),
            Some(lof.clone()),
        );
        lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(lt2),
            Some(lt0),
            Some(ld1),
            Some(lof),
        );

        let span = Span::compute(&lg, lv1.clone());

        let leveling = Leveling::compute(span);
        let cs = 2;
        println!("[RESULT]: {:?}", leveling.levels);
        assert_eq!(leveling.size(), cs);
        assert_eq!(
            leveling.levels,
            vec![HashSet::from([lv1]), HashSet::from([lv0, lv2])]
        );

        test_rings(leveling, cs);
    }

    #[test]
    fn two_triangle() {
        let mut lg = LinkGraph::new();
        let lv0 = lg.new_vertex();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();
        let lv3 = lg.new_vertex();

        let lt0 = lg.new_dart(lv1.clone(), lv0.clone(), None, None, None, None);
        let lof = lg.new_face(lt0.clone()); // Outer Face first

        let ld0 = lg.new_dart(
            lv0.clone(),
            lv1.clone(),
            None,
            None,
            Some(lt0.clone()),
            None,
        );
        let lf = lg.new_face(ld0.clone());

        let ld1 = lg.new_dart(
            lv1.clone(),
            lv2.clone(),
            Some(ld0.clone()),
            None,
            None,
            Some(lf.clone()),
        );
        let ld2 = lg.new_dart(
            lv2.clone(),
            lv0.clone(),
            Some(ld1.clone()),
            Some(ld0),
            None,
            Some(lf),
        );

        let lt2 = lg.new_dart(
            lv0.clone(),
            lv2.clone(),
            Some(lt0.clone()),
            None,
            Some(ld2.clone()),
            None,
        );

        lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(lt2.clone()),
            Some(lt0),
            Some(ld1),
            Some(lof.clone()),
        );

        let lf2 = lg.new_face(lt2.clone());
        let ld3 = lg.new_dart(
            lv2.clone(),
            lv3.clone(),
            Some(lt2.clone()),
            None,
            None,
            Some(lf2.clone()),
        );
        let ld4 = lg.new_dart(
            lv3.clone(),
            lv0.clone(),
            Some(ld3.clone()),
            Some(lt2),
            None,
            Some(lf2),
        );
        let lt3 = lg.new_dart(
            lv3.clone(),
            lv2.clone(),
            None,
            Some(ld2.clone()),
            Some(ld3),
            Some(lof.clone()),
        );
        lg.new_dart(
            lv0.clone(),
            lv3.clone(),
            Some(ld2),
            Some(lt3),
            Some(ld4),
            Some(lof),
        );

        let span = Span::compute(&lg, lv1.clone());

        let leveling = Leveling::compute(span);
        let cs = 3;
        println!("[RESULT]: {:?}", leveling.levels);
        assert_eq!(leveling.size(), cs);
        assert_eq!(
            leveling.levels,
            vec![
                HashSet::from([lv1]),
                HashSet::from([lv0, lv2]),
                HashSet::from([lv3]),
            ]
        );

        test_rings(leveling, cs);
    }

    #[test]
    fn three_triangle() {
        let mut lg = LinkGraph::new();
        let lv0 = lg.new_vertex();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();
        let lv3 = lg.new_vertex();
        let lv4 = lg.new_vertex();

        let lt0 = lg.new_dart(lv1.clone(), lv0.clone(), None, None, None, None);
        let lof = lg.new_face(lt0.clone()); // Outer Face first

        let ld0 = lg.new_dart(
            lv0.clone(),
            lv1.clone(),
            None,
            None,
            Some(lt0.clone()),
            None,
        );
        let lf = lg.new_face(ld0.clone());

        let ld1 = lg.new_dart(
            lv1.clone(),
            lv2.clone(),
            Some(ld0.clone()),
            None,
            None,
            Some(lf.clone()),
        );
        let ld2 = lg.new_dart(
            lv2.clone(),
            lv0.clone(),
            Some(ld1.clone()),
            Some(ld0),
            None,
            Some(lf),
        );

        let lt2 = lg.new_dart(
            lv0.clone(),
            lv2.clone(),
            Some(lt0.clone()),
            None,
            Some(ld2.clone()),
            None,
        );

        lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(lt2.clone()),
            Some(lt0),
            Some(ld1),
            Some(lof.clone()),
        );

        let lf2 = lg.new_face(lt2.clone());
        let ld3 = lg.new_dart(
            lv2.clone(),
            lv3.clone(),
            Some(lt2.clone()),
            None,
            None,
            Some(lf2.clone()),
        );
        let ld4 = lg.new_dart(
            lv3.clone(),
            lv0.clone(),
            Some(ld3.clone()),
            Some(lt2),
            None,
            Some(lf2),
        );
        let lt3 = lg.new_dart(
            lv3.clone(),
            lv2.clone(),
            None,
            Some(ld2.clone()),
            Some(ld3),
            Some(lof.clone()),
        );
        let lt4 = lg.new_dart(
            lv0.clone(),
            lv3.clone(),
            Some(ld2),
            Some(lt3),
            Some(ld4.clone()),
            None,
        );

        let lf3 = lg.new_face(lt4.clone());
        let ld5 = lg.new_dart(
            lv3.clone(),
            lv4.clone(),
            Some(lt4.clone()),
            None,
            None,
            Some(lf3.clone()),
        );
        let ld6 = lg.new_dart(
            lv4.clone(),
            lv0.clone(),
            Some(ld5.clone()),
            Some(lt4),
            None,
            Some(lf3),
        );
        let lt6 = lg.new_dart(
            lv0.clone(),
            lv4.clone(),
            Some(ld4.clone()),
            None,
            Some(ld6),
            Some(lof.clone()),
        );
        lg.new_dart(
            lv4.clone(),
            lv3.clone(),
            Some(lt6),
            Some(ld4),
            Some(ld5),
            Some(lof),
        );

        let span = Span::compute(&lg, lv1.clone());

        let leveling = Leveling::compute(span);
        let cs = 3;
        println!("[RESULT]: {:?}", leveling.levels);
        assert_eq!(leveling.size(), cs);
        assert_eq!(
            leveling.levels,
            vec![
                HashSet::from([lv1]),
                HashSet::from([lv0, lv2]),
                HashSet::from([lv3, lv4]),
            ]
        );

        test_rings(leveling, cs);
    }

    #[test]
    fn test() {
        let mut lg = LinkGraph::new();
        let lv0 = lg.new_vertex();
        let lv1 = lg.new_vertex();
        let lv2 = lg.new_vertex();
        let lv3 = lg.new_vertex();
        let lv4 = lg.new_vertex();

        let ld0 = lg.new_dart(lv0.clone(), lv1.clone(), None, None, None, None);
        let lof = lg.new_face(ld0.clone());
        let ld1 = lg.new_dart(
            lv1.clone(),
            lv2.clone(),
            Some(ld0.clone()),
            None,
            None,
            Some(lof.clone()),
        );
        let ld2 = lg.new_dart(
            lv2.clone(),
            lv3.clone(),
            Some(ld1.clone()),
            None,
            None,
            Some(lof.clone()),
        );
        let ld3 = lg.new_dart(
            lv3.clone(),
            lv4.clone(),
            Some(ld2.clone()),
            None,
            None,
            Some(lof.clone()),
        );
        let lt3 = lg.new_dart(
            lv4.clone(),
            lv3.clone(),
            Some(ld3.clone()),
            None,
            Some(ld3),
            Some(lof.clone()),
        );
        let lt2 = lg.new_dart(
            lv3.clone(),
            lv2.clone(),
            Some(lt3),
            None,
            Some(ld2),
            Some(lof.clone()),
        );
        let lt1 = lg.new_dart(
            lv2.clone(),
            lv1.clone(),
            Some(lt2),
            None,
            Some(ld1),
            Some(lof.clone()),
        );
        lg.new_dart(
            lv1.clone(),
            lv0.clone(),
            Some(lt1),
            Some(ld0.clone()),
            Some(ld0),
            Some(lof),
        );

        let span = Span::compute(&lg, lv1.clone());

        let leveling = Leveling::compute(span);
        let cs = 4;
        println!("[RESULT]: {:?}", leveling.levels);
        assert_eq!(leveling.size(), cs);
        assert_eq!(
            leveling.levels,
            vec![
                HashSet::from([lv1]),
                HashSet::from([lv0, lv2]),
                HashSet::from([lv3]),
                HashSet::from([lv4]),
            ]
        );

        test_rings(leveling, cs);
    }

    #[test]
    fn rings() {
        let mut lg = LinkGraph::new();
        let leveling: Vec<HashSet<LinkVertex>> = vec![
            HashSet::from([lg.new_vertex(), lg.new_vertex(), lg.new_vertex()]),
            HashSet::from([lg.new_vertex()]),
            HashSet::from([lg.new_vertex(), lg.new_vertex()]),
        ];
        test_rings(Leveling { levels: leveling }, 3);
    }

    fn test_rings(leveling: Leveling<LinkVertex>, size: usize) {
        for k in 1..size + 1 {
            let rings = leveling.rings(k);
            assert_eq!(rings.len(), (size as f64 / k as f64).ceil() as usize);
        }
    }
}
