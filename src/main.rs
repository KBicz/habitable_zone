use std::env;
use std::process::exit;

fn helpf()
{
    println!("\n  Program habzone for MacOS, Linux and Windows written by K. Bicz, ver. of Jan 21, 2022.");
    println!("  Estimate habitable zone parameters.");
    println!("\n  Usage: habzone <-teff=float> [-lstar=float] [--verb]\n");
    println!("         option  -teff  : effective temperature of the star.");
    println!("                 -lstar : luminosity of star (in solar luminosities).");
    println!("                 --verb : program prints publication of origin.\n");
    exit(5);
}

fn habzone(teff: f64, lstar: f64, verb: bool, lstarctrl: bool)
{
    let (tstar, mut seff) = (teff - 5780.0f64, vec![0.0f64;6]);
    let seffsun = vec![1.776f64,1.107f64,0.356f64,0.320f64,1.188f64,0.99f64];
    let a = vec![2.136e-4_f64,1.332e-4f64,6.171e-5f64,5.547e-5f64,1.433e-4f64,1.209e-4f64];
    let b = vec![2.533e-8f64,1.580e-8f64,1.698e-9f64,1.526e-9f64,1.707e-8f64,1.404e-8f64];
    let c = vec![-1.332e-11f64,-8.308e-12f64,-3.198e-12f64,-2.874e-12f64,-8.968e-12f64,-7.418e-12f64];
    let d = vec![-3.097e-15f64,-1.931e-15f64,-5.575e-16f64,-5.011e-16f64,-2.084e-15f64,-1.713e-15f64];

    for i in 0..seff.len() {seff[i] = seffsun[i] + a[i]*tstar + b[i]*tstar.powf(2.0f64) + c[i]*tstar.powf(3.0f64) + d[i]*tstar.powf(4.0f64);}

    if verb
    {
        println!("\n This code calculates habitable zone 'fluxes' using the expression given in the Kopparapu et al.(2014) paper using:");
        println!(" S_effSun = {:?}",seffsun);
        println!("    a     = [{:+e}, {:+e}, {:+e}, {:+e}, {:+e}, {:+e}] ",a[0],a[1],a[2],a[3],a[4],a[5]);
        println!("    b     = [{:+e}, {:+e}, {:+e}, {:+e}, {:+e}, {:+e}] ",b[0],b[1],b[2],b[3],b[4],b[5]);
        println!("    c     = [{:+e}, {:+e}, {:+e}, {:+e}, {:+e}, {:+e}] ",c[0],c[1],c[2],c[3],c[4],c[5]);
        println!("    d     = [{:+e}, {:+e}, {:+e}, {:+e}, {:+e}, {:+e}] ",d[0],d[1],d[2],d[3],d[4],d[5]);
    }
    
    println!("\n#  Teff [K]        Recent        Runaway       Maximum       Early        5ME Runaway   0.1ME Runaway");
    println!("#    {}           Venus     Greenhouse    Greenhouse        Mars         Greenhouse      Greenhouse",teff);
    println!("    {}        {:.5}        {:.5}       {:.5}     {:.5}            {:.5}         {:.5}","S [Sc]",seff[0],seff[1],seff[2],seff[3],seff[4],seff[5]);
    if lstarctrl {println!("  {}       {:.5}        {:.5}       {:.5}     {:.5}            {:.5}         {:.5}\n","Dist [AU]",(lstar/seff[0]).powf(0.5f64),(lstar/seff[1]).powf(0.5f64),(lstar/seff[2]).powf(0.5f64),(lstar/seff[3]).powf(0.5f64),(lstar/seff[4]).powf(0.5f64),(lstar/seff[5]).powf(0.5f64) );}

}

fn main() 
{
    let mut v: Vec<&str>;
    let argc = env::args().len();
    let argv: Vec<String> = env::args().collect();
    let (mut teff, mut lstar): (f64, f64) = (2600.0f64, 0.03f64);
    let (mut verb, mut teffctrl, mut lstarctrl) = (false, false, false);
    
    if argc < 2 {helpf();}
    else if argc == 2 && !argv[1].eq("--help") && !argv[1].eq("-h") {v = argv[1].split("=").collect(); teff = v[1].parse().unwrap(); teffctrl = true;}
    else 
    {
        for i in 0..argc
        {
            if argv[i].contains("-teff=") {v = argv[i].split("=").collect(); teff = v[1].parse().unwrap(); teffctrl = true;}
            else if argv[i].contains("-lstar=") {v = argv[i].split("=").collect(); lstar = v[1].parse().unwrap(); lstarctrl = true;}
            else if argv[i].eq("--verb") {verb = true;}
        }
    }

    if !teffctrl
    {
        println!("\n# Error! You have to give effective temperature of the star!\n");
        exit(1);
    }

    habzone(teff,lstar,verb,lstarctrl);
}
