pub mod p1;
pub mod p2;
pub mod parser;

pub fn quadratic_roots(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let x1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let x2 = (-b + discriminant.sqrt()) / (2.0 * a);

    Some((x1, x2))
}
