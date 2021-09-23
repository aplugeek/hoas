#[macro_export]
macro_rules! mongo_syntax {
    ($q:tt,$filter:tt,$pv:tt) => {
        match $q.operator {
            Operator::EQ => match $q.value_type {
                QueryParamKind::Number => {
                    let v = parse_num($pv);
                    $filter.insert($q.name.as_str(), v);
                }
                _ => {
                    $filter.insert($q.name.as_str(), $pv);
                }
            },
            Operator::NE => {
                let ne = match $q.value_type {
                    QueryParamKind::Number => {
                        let v = parse_num($pv);
                        doc! {
                            $q.name.as_str(): { "$ne": v }
                        }
                    }
                    _ => doc! {
                        $q.name.as_str(): { "$ne": $pv }
                    },
                };
                $filter.extend(ne);
            }
            Operator::LIKE => {
                let ne = doc! {$q.name.as_str():mongodb::bson::Regex{
                    pattern:format!(".*{:?}.*",$pv),
                    options:String::from("i")
                }};
                $filter.extend(ne);
            }
            Operator::GT => {
                let v = parse_num($pv);
                let gt = doc! {
                    $q.name.as_str(): { "$gt": v }
                };
                $filter.extend(gt);
            }
            Operator::LT => {
                let v = parse_num($pv);
                let gt = doc! {
                    $q.name.as_str(): { "$lt": v }
                };
                $filter.extend(gt);
            }
            Operator::GTE => {
                let v = parse_num($pv);
                let gt = doc! {
                    $q.name.as_str(): { "$gte": v }
                };
                $filter.extend(gt);
            }
            Operator::LTE => {
                let v = parse_num($pv);
                let gt = doc! {
                    $q.name.as_str(): { "$lte": v }
                };
                $filter.extend(gt);
            }
        }
    };
}

pub fn parse_num(s: &str) -> i64 {
    s.parse::<i64>().expect("query value type invalid")
}
