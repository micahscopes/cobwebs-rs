use crate::geometry::EdgeGeo;
use crate::layout::GraphLayout;
use geo::algorithm::intersects::Intersects;
use petgraph::graph::EdgeIndex;
use rstar::RTreeObject;

pub fn edge_intersects_edges<'a>(
    edge: EdgeGeo,
    edges: impl Iterator<Item = (&'a EdgeIndex, &'a EdgeGeo)>,
) -> impl Iterator<Item = (&'a EdgeIndex, &'a EdgeGeo)> {
    return edges.into_iter().filter(move |(_, other_edge)| {
        edge.intersects(other_edge) && edge != **other_edge
    });
}

impl GraphLayout {
    pub fn count_edge_intersections(&self, edge_index: EdgeIndex) -> usize {
        match self.edge_geo(edge_index) {
            Some(edge) => edge_intersects_edges(
                edge,
                self.graph_geo_tree.edges_in_envelope(&edge.envelope()),
            )
            .count(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_intersection() {
        use geo::algorithm::intersects::Intersects;
        use geo::{Coordinate, LineString};

        let p = |x, y| Coordinate { x: x, y: y };
        let linestring = LineString(vec![p(3., 2.), p(7., 6.)]);

        assert!(linestring.intersects(&LineString(vec![p(3., 4.), p(8., 4.)])));
        assert!(
            !linestring.intersects(&LineString(vec![p(9., 2.), p(11., 5.)]))
        );
    }

    #[test]
    fn test_rstar() {
        use rstar::RTree;
        let mut tree = RTree::new();
        tree.insert([0.1, 0.0f32]);
        tree.insert([0.2, 0.1]);
        tree.insert([0.3, 0.0]);

        assert_eq!(tree.nearest_neighbor(&[0.4, -0.1]), Some(&[0.3, 0.0]));
        tree.remove(&[0.3, 0.0]);
        assert_eq!(tree.nearest_neighbor(&[0.4, 0.3]), Some(&[0.2, 0.1]));

        assert_eq!(tree.size(), 2);
    }

    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }
}
