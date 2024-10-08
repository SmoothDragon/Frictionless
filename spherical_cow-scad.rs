
use flowscad::*;
use anyhow::Result;


fn spherical_cow<T: Into<X>>(radius: T) -> D3 {
    let r = radius.into();
    let body = D3::sphere(r)
        .translate_z(r*(2.0_f64.powf(-0.5)))
        .intersection(D3::cube(4*r).center().translate_z(2*r))
        ;
    let leg = D3::sphere(r/4)
        .translate(0.85*r*v3(0.5, 0.5*3.0_f64.powf(0.5), 2.0_f64.powf(-0.5)))
        .add_map(|x| x.translate_z(-r))
        .hull()
        .add_map(|x| x.mirror( (1,0,0) ))
        .add_map(|x| x.mirror( (0,1,0) ))
        .intersection(D3::cube(4*r).center().translate_z(2*r))
        ; 
    let head = D3::sphere(r/2)
        .translate(r*v3(0, 0.8, 0.75))
        ;
    body + leg + head
}

fn main() -> Result<()> {
    let cow = spherical_cow(12);
    println!("$fn=64;\n{}", &cow);
    Ok(())
}
