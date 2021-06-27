use serde::ser;

/// Workaround for https://github.com/serde-rs/serde/issues/1937
pub fn serialize_vec_array<S, T, const N: usize>(
    value: &[[T; N]],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
    T: ser::Serialize,
{
    use ser::SerializeTuple;
    let mut tup = serializer.serialize_tuple(N)?;
    for row in 0..value.len() {
        for col in 0..N {
            tup.serialize_element(&value[row][col])?;
        }
    }
    tup.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Serialize;

    #[derive(Debug, Default, PartialEq, Eq, Serialize)]
    struct ComplexObj {
        field1: u8,
        field2: String,
        field3: bool,
    }

    #[derive(Debug, PartialEq, Eq, Serialize)]
    struct TestVecArray<T: Default, const N: usize>(
        #[serde(
            bound(serialize = "T: Serialize"),
            serialize_with = "serialize_vec_array"
        )]
        Vec<[T; N]>,
    );

    #[test]
    fn serialize_vec_array_should_correctly_serialize() {
        let arr = TestVecArray(vec![[1, 2, 3, 4], [5, 6, 7, 8]]);
        let s = serde_json::to_string(&arr).unwrap();
        assert_eq!(s, "[1,2,3,4,5,6,7,8]");
    }

    #[test]
    fn serialize_vec_array_should_support_complex_generic_types() {
        let arr = TestVecArray(vec![
            [ComplexObj {
                field1: 1,
                field2: "hello".to_string(),
                field3: false,
            }],
            [ComplexObj {
                field1: 2,
                field2: "world".to_string(),
                field3: true,
            }],
        ]);
        let s = serde_json::to_string(&arr).unwrap();
        assert_eq!(
            s,
            concat!(
                "[",
                r#"{"field1":1,"field2":"hello","field3":false}"#,
                ",",
                r#"{"field1":2,"field2":"world","field3":true}"#,
                "]",
            )
        );
    }
}
