# habitable_zone
Program to estimate the habitable zone range for given star. Program is based on the [Kopparapu et al.(2014)](http://dx.doi.org/10.1088/2041-8205/787/2/L29). Compile it using command "cargo build --release".

    Usage: habzone <-teff=float> [-lstar=float] [--verb]

         option  -teff  : effective temperature of the star.
                 -lstar : luminosity of star (in solar luminosities).
                 --verb : program prints publication of origin.
