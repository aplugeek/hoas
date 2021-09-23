use hoas_conf::conf::Route;
use std::collections::HashMap;

pub struct RouteMap(pub HashMap<String, Route>);

impl RouteMap {
    pub fn from_conf(route: &Vec<Route>) -> Self {
        let mut route_map = HashMap::new();
        route.iter().for_each(|r| {
            route_map.insert(r.path.clone(), r.clone());
        });
        RouteMap(route_map)
    }
}
