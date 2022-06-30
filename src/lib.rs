extern crate wasm_bindgen;
extern crate pem;

use base64::encode;
use pem::parse;
use ring::rand;
use ring::signature;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rsa2_sign(message: &str) -> String {
    const PRIVATE_KEY: &'static str = "-----BEGIN RSA PRIVATE KEY-----
MIIEogIBAAKCAQEAwkQQSRoP42Mrc9mP1tg7j2sKhtXAlyMWnTD8uX39y0dnS62U
PzI99JJcVmlzjQqt8ZyLhNXiOECVGa9TLngRgWwQ08JOiIWfqDGgV95GwB14au8J
eoU3ekWoBeX6MaeIWTgVL8lmAzSLnf46sbIATSievM3jGvqtRF++2k3mvUiU8RIi
nOzwWHurVSUCBckffRa7sXZzFZ84DXB0EjedVeiCIqXOohvrPCPE3G/HA1MKpdjo
nENm2FRoh9Mezphkge/1765JrOndPi65asQmp1qr0hHDykFmWNpqpLdF3G1HQWJ+
ZDfivD8Hjd1Ir4+7w3ObTvUXYGKhjWm5GvMPBQIDAQABAoIBAGDJoIOLZJcZ9bBG
mQ+ltCcEYb5ECSxD4yR2H7QbPbNl4+UkmlPSKGQN98tlxphi1b8nC6wnFAqKwr6F
R6BDWhVtTgTzLkV+rckS0Jx2tjFJfZFpfwN4E+frhIdLyZerrJcnBgdFBHpcV6Nj
4sJduSC4OD5BJgFvG5yRR3lkjq2G+dZghphzrLw+tTOTL4iQzfuHSgr6ZL0C7Pxf
KWzlePtaYh958yM0NDgJ+ZEP/4hpPTJty2k0Hir7+CVqt3QD8AaNprYhEbCODXgc
FTWeBVHNnr9Uq1bOeklddi2ksY/kwbVcc3FZnS+afw9tntGFBTLszbLXgw2g4cik
44IadwECgYEA30Dl8LWuglsmvsG2Qni/ehtryvPuLGKYy23YgOPsJzW2Qx3G/YK0
/g0IQD1LEktEfVjsajLYc42NQz/EN9XDtPUFZM9+miuHa/apEXeTko5gbTkTvtw6
0Teew4toIZxqrdf7NyucanlbXT9u1AT0LfNKKaGJAKg8+/novTFW7xECgYEA3sKw
P6G65GjPcLad3AdXzx7T0qW51CxtHbF32n/PihCdZSu54mzGXNPMYbFqInKuIrT8
0Ya+eD0Ob6CpeU/84+dhMhqC+1rgDxIyObPgvd8v/jS8VA7Ios3wrA+YmGgaS665
yGIO69EtP6nwBZeqthzK4ryXajGDxAwQ5gfLiLUCgYBzbT51gbnENBSIM+dj3sJB
A8RrnkraL/AH3bc+jJkgfKjLcq3bjKS/ZUWK0SXrQ3cZx8wT7E31EI6k4mc05xTU
jopHYPB4DWFq9JDdMxHdiDdawVPwLjVKzpMLTxx7pD+TukNsduPcOPp/77gBbr5A
s7T4aUhISrOdI8ytZMEpwQKBgCvVY5kh2gTYGvmy9Z02VLEM/+GTet2lYFNXeJ4m
SFCCtHYUBJ/ph578jlsq7heZOWra8mTIfvJqEU0l8DNO6hRuk1Gopd2QJEbS7c8s
9ixmnqsG9YgU4KYooBM7fmt/EK2T1Dr6ELWWk7DTX8e58H2lbMbxFVpwI92QAkNB
hWuBAoGAfxfSlp7UR7x2gjiQVn2yaI+cIfUfXYDgoo0CLepfVDFPyjPglMXNPuZ3
OXPNfdNso4SxJHAnlLqcPyf5lWdsuNe1SUFT7wDYdUuUAwk5y80f41VOA79J8YLr
arv0Mgxaip1YuKrWTS7YkcxVn3JHE99Adst1T3MvHkSM7qnpk3Q=
-----END RSA PRIVATE KEY-----
";
    let pem = parse(PRIVATE_KEY).unwrap();
    let rng = rand::SystemRandom::new();
    let key_pair = signature::RsaKeyPair::from_der(&pem.contents).unwrap();
    let mut signature = vec![0; key_pair.public_modulus_len()];
    key_pair
        .sign(
            &signature::RSA_PKCS1_SHA256,
            &rng,
            message.as_bytes(),
            &mut signature,
        )
        .unwrap();

    return encode(&signature);
}

#[cfg(test)]
mod tests {
    use super::rsa2_sign;
    #[test]
    fn it_works() {
        let message = "lbb";
        let signature: String = rsa2_sign(message);
        assert_eq!(signature, "Wy382uw+d/PnnTzMZ7e+cVW7PVvz5GM0bG34TXQLRIPGqxBZzojDoaVroZba94NfVcFRWQ4bxAAlA8JbaZ5rAISB+rtShUidrY91rI1GhPj+yTgk2JJYI7DttNmBQckyHXIyIWnuXFiV3f3hZtfzOBqVP+lJH1Po33n9T7rG+foQEtR03DPggKrNKCn8mdJpHLXhy5uqZ30ZJTCs+GXrMITS48faJpObNqZ6zbL/ixN/DUQ1mIty+w6d4ccYGc6vEusbe93176Wc6wI1W7Uc9nWEA/PMCgVwZaRcQ6XiASxmn+a4wDMW4poC29D2wBb2uCBz+7UuRdkKowsJ5hjmRA==");
    }
}
