/// Calculates Dr. Nathan's spreadsheet
/// 

use core::f64::consts::*;

pub (crate) struct Data {
    pub (crate) x0: f64,
    pub (crate) y0: f64,
    pub (crate) z0: f64,
    pub (crate) vx0: f64,
    pub (crate) vy0: f64,
    pub (crate) vz0: f64,
    pub (crate) ax: f64,
    pub (crate) ay: f64,
    pub (crate) az: f64,
    pub (crate) extension: f64,
    pub (crate) plate_x: f64,
    pub (crate) plate_z: f64,
}

#[derive(Debug)]
pub (crate) struct Nathan {
    pub (crate) xr: Option<f64>,
    pub (crate) yr: Option<f64>,
    pub (crate) zr: Option<f64>,
    pub (crate) tr: Option<f64>,
    pub (crate) vxr: Option<f64>,
    pub (crate) vyr: Option<f64>,
    pub (crate) vzr: Option<f64>,
    pub (crate) tf: Option<f64>,
    pub (crate) vxbar: Option<f64>,
    pub (crate) vybar: Option<f64>,
    pub (crate) vzbar: Option<f64>,
    pub (crate) vbar: Option<f64>,
    pub (crate) vxhat: Option<f64>,
    pub (crate) vyhat: Option<f64>,
    pub (crate) vzhat: Option<f64>,
    pub (crate) ad: Option<f64>,
    pub (crate) atx: Option<f64>,
    pub (crate) aty: Option<f64>,
    pub (crate) atz: Option<f64>,
    pub (crate) atx_hat: Option<f64>,
    pub (crate) aty_hat: Option<f64>,
    pub (crate) atz_hat: Option<f64>,
    pub (crate) at: Option<f64>,
    pub (crate) phi_t: Option<f64>,
    pub (crate) ivb: Option<f64>,
    pub (crate) hb: Option<f64>,
    pub (crate) cd: Option<f64>,
}

impl Default for Nathan {
    fn default() -> Self {
        Self {
            xr: None,
            yr: None,
            zr: None,
            tr: None,
            vxr: None,
            vyr: None,
            vzr: None,
            tf: None,
            vxbar: None,
            vybar: None,
            vzbar: None,
            vbar: None,
            vxhat: None,
            vyhat: None,
            vzhat: None,
            ad: None,
            atx: None,
            aty: None,
            atz: None,
            atx_hat: None,
            aty_hat: None,
            atz_hat: None,
            at: None,
            phi_t: None,
            ivb: None,
            hb: None,
            cd: None,
        }
    }
}

impl From<Data> for Nathan {
    fn from (data: Data) -> Self {
        let extension = data.extension;
        let vx0 = data.vx0;
        let vy0 = data.vy0;
        let vz0 = data.vz0;
        let x0 = data.x0;
        let y0 = data.y0;
        let z0 = data.z0;
        let ax = data.ax;
        let ay = data.ay;
        let az = data.az;
        let plate_x = data.plate_x;
        let plate_z = data.plate_z;
        
        const PLATE: f64 = 17.0/12.0;
        const K: f64 =  0.00538310;
        const G: f64 = 32.174;

        let yr = 60.5 - extension;
    
        let tr = (-vy0 - (vy0.powf(2.0) - 2. * ay * (y0 - yr)).sqrt())/ay;
        
        let vxr = vx0 + ax * tr;
        let vyr = vy0 + ay * tr;
        let vzr = vz0 + az * tr;
        
        let xr = x0 + (vx0 + vxr)/2. * tr;
        let zr = z0 + (vz0 + vzr)/2. * tr;
                       
        let tf = (-vyr -(vyr.powf(2.) - 2.*ay*(yr - PLATE)).sqrt())/ay;
        let vxbar = (2. * vxr + ax * tf)/2.;
        let vybar = (2. * vyr + ay * tf)/2.;
        let vzbar = (2. * vzr + az * tf)/2.;

        let vbar = (vxbar.powf(2.) + vybar.powf(2.) + vzbar.powf(2.)).sqrt();

        let vxhat = vxbar / vbar;
        let vyhat = vybar / vbar;
        let vzhat = vzbar / vbar;

        let ad = -(vxhat * ax + vyhat * ay + vzhat * (az+G));

        let atx = ax + ad * vxhat;
        let aty = ay + ad * vyhat;
        let atz = az + ad * vzhat + G;
        let at = (atx.powf(2.0) + aty.powf(2.0) + atz.powf(2.0)).sqrt();

        let atx_hat = atx / at;
        let aty_hat = aty / at;
        let atz_hat = atz / at;

        let phi_t = atz_hat.atan2(atx_hat)* 180.0 / PI + if atz < 0. {360.0} else {0.0};

        
        let hb = (plate_x - xr - (vxr/vyr)*(PLATE-yr))*12.;
        let ivb = (plate_z - zr - (vzr/vyr)*(PLATE-yr)+0.5*G*tf.powf(2.))*12.;
        let cd = ad / (K * vbar.powf(2.0));

        Nathan {
            xr: Some(xr),
            yr: Some(yr),
            zr: Some(zr),
            tr: Some(tr),
            vxr: Some(vxr),
            vyr: Some(vyr),
            vzr: Some(vzr),
            tf: Some(tf),
            vxbar: Some(vxbar),
            vybar: Some(vybar),
            vzbar: Some(vzbar),
            vbar: Some(vbar),
            vxhat: Some(vxhat),
            vyhat: Some(vyhat),
            vzhat: Some(vzhat),
            ad: Some(ad),
            atx: Some(atx),
            aty: Some(aty),
            atz: Some(atz),
            atx_hat: Some(atx_hat),
            aty_hat: Some(aty_hat),
            atz_hat: Some(atz_hat),
            at: Some(at),
            phi_t: Some(phi_t),
            ivb: Some(ivb),
            hb: Some(hb),
            cd: Some(cd),
        }
    }
}