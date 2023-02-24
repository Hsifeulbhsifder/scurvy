//Copyright (c) 2020 Marco Boneberger
use gnuplot::Coordinate::Graph;
use gnuplot::{AxesCommon, Caption, Figure};
use s_curve::*;

fn main() {
    let constraints = SCurveConstraints {
        max_jerk: 3.,
        max_acceleration: 2.0,
        max_velocity: 3.,
    };
    let start_conditions = SCurveStartConditions {
        _q0: 0.,
        _q1: 10.,
        _v0: 0.,
        _v1: 0.,
        dir: true,
    };
    let input = SCurveInput {
        constraints,
        start_conditions,
    };
    let (params, s_curve) = s_curve_generator(&input, Derivative::Velocity);
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    for i in 0..1001 {
        x.push(i as f64 * params.time_intervals.total_duration() / 1000.);
        y.push(s_curve(
            i as f64 * params.time_intervals.total_duration() / 1000.,
        ));
    }
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("S-Curve Velocity Motion Profile", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label("time in seconds", &[])
        .set_y_label("velocity in m/s", &[])
        .lines(x.clone(), y.clone(), &[Caption("Velocity")]);
    fg.show().unwrap();
}
