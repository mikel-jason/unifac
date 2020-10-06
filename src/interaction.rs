pub struct InteractionParameter {
    i: u8,
    j: u8,
    a_ij: f64,
}

impl InteractionParameter {
    pub fn from(i: u8, j: u8) -> Result<InteractionParameter, &'static str> {
        let i_internal: usize = i.into();
        let j_internal: usize = j.into();
        let id_internal = i_internal * 1000 + j_internal;
        match id_internal {
            1002 => Ok(InteractionParameter {
                i: 1,
                j: 2,
                a_ij: 86.0200,
            }),
            2001 => Ok(InteractionParameter {
                i: 2,
                j: 1,
                a_ij: -35.3600,
            }),
            1003 => Ok(InteractionParameter {
                i: 1,
                j: 3,
                a_ij: 61.1300,
            }),
            3001 => Ok(InteractionParameter {
                i: 3,
                j: 1,
                a_ij: -11.1200,
            }),
            1004 => Ok(InteractionParameter {
                i: 1,
                j: 4,
                a_ij: 76.5000,
            }),
            4001 => Ok(InteractionParameter {
                i: 4,
                j: 1,
                a_ij: -69.7000,
            }),
            1005 => Ok(InteractionParameter {
                i: 1,
                j: 5,
                a_ij: 986.5000,
            }),
            5001 => Ok(InteractionParameter {
                i: 5,
                j: 1,
                a_ij: 156.4000,
            }),
            1006 => Ok(InteractionParameter {
                i: 1,
                j: 6,
                a_ij: 697.2000,
            }),
            6001 => Ok(InteractionParameter {
                i: 6,
                j: 1,
                a_ij: 16.5100,
            }),
            1007 => Ok(InteractionParameter {
                i: 1,
                j: 7,
                a_ij: 1318.0000,
            }),
            7001 => Ok(InteractionParameter {
                i: 7,
                j: 1,
                a_ij: 300.0000,
            }),
            1008 => Ok(InteractionParameter {
                i: 1,
                j: 8,
                a_ij: 1333.0000,
            }),
            8001 => Ok(InteractionParameter {
                i: 8,
                j: 1,
                a_ij: 275.8000,
            }),
            1009 => Ok(InteractionParameter {
                i: 1,
                j: 9,
                a_ij: 476.4000,
            }),
            9001 => Ok(InteractionParameter {
                i: 9,
                j: 1,
                a_ij: 26.7600,
            }),
            1010 => Ok(InteractionParameter {
                i: 1,
                j: 10,
                a_ij: 677.0000,
            }),
            10001 => Ok(InteractionParameter {
                i: 10,
                j: 1,
                a_ij: 505.7000,
            }),
            1011 => Ok(InteractionParameter {
                i: 1,
                j: 11,
                a_ij: 232.1000,
            }),
            11001 => Ok(InteractionParameter {
                i: 11,
                j: 1,
                a_ij: 114.8000,
            }),
            1012 => Ok(InteractionParameter {
                i: 1,
                j: 12,
                a_ij: 507.0000,
            }),
            12001 => Ok(InteractionParameter {
                i: 12,
                j: 1,
                a_ij: 329.3000,
            }),
            1013 => Ok(InteractionParameter {
                i: 1,
                j: 13,
                a_ij: 251.5000,
            }),
            13001 => Ok(InteractionParameter {
                i: 13,
                j: 1,
                a_ij: 83.3600,
            }),
            1014 => Ok(InteractionParameter {
                i: 1,
                j: 14,
                a_ij: 391.5000,
            }),
            14001 => Ok(InteractionParameter {
                i: 14,
                j: 1,
                a_ij: -30.4800,
            }),
            1015 => Ok(InteractionParameter {
                i: 1,
                j: 15,
                a_ij: 255.7000,
            }),
            15001 => Ok(InteractionParameter {
                i: 15,
                j: 1,
                a_ij: 65.3300,
            }),
            1016 => Ok(InteractionParameter {
                i: 1,
                j: 16,
                a_ij: 206.6000,
            }),
            16001 => Ok(InteractionParameter {
                i: 16,
                j: 1,
                a_ij: -83.9800,
            }),
            1017 => Ok(InteractionParameter {
                i: 1,
                j: 17,
                a_ij: 920.7000,
            }),
            17001 => Ok(InteractionParameter {
                i: 17,
                j: 1,
                a_ij: 1139.0000,
            }),
            1018 => Ok(InteractionParameter {
                i: 1,
                j: 18,
                a_ij: 287.7700,
            }),
            18001 => Ok(InteractionParameter {
                i: 18,
                j: 1,
                a_ij: -101.5600,
            }),
            1019 => Ok(InteractionParameter {
                i: 1,
                j: 19,
                a_ij: 597.0000,
            }),
            19001 => Ok(InteractionParameter {
                i: 19,
                j: 1,
                a_ij: 24.8200,
            }),
            1020 => Ok(InteractionParameter {
                i: 1,
                j: 20,
                a_ij: 663.5000,
            }),
            20001 => Ok(InteractionParameter {
                i: 20,
                j: 1,
                a_ij: 315.3000,
            }),
            1021 => Ok(InteractionParameter {
                i: 1,
                j: 21,
                a_ij: 35.9300,
            }),
            21001 => Ok(InteractionParameter {
                i: 21,
                j: 1,
                a_ij: 91.4600,
            }),
            1022 => Ok(InteractionParameter {
                i: 1,
                j: 22,
                a_ij: 53.7600,
            }),
            22001 => Ok(InteractionParameter {
                i: 22,
                j: 1,
                a_ij: 34.0100,
            }),
            1023 => Ok(InteractionParameter {
                i: 1,
                j: 23,
                a_ij: 24.9000,
            }),
            23001 => Ok(InteractionParameter {
                i: 23,
                j: 1,
                a_ij: 36.7000,
            }),
            1024 => Ok(InteractionParameter {
                i: 1,
                j: 24,
                a_ij: 104.3000,
            }),
            24001 => Ok(InteractionParameter {
                i: 24,
                j: 1,
                a_ij: -78.4500,
            }),
            1025 => Ok(InteractionParameter {
                i: 1,
                j: 25,
                a_ij: 11.4400,
            }),
            25001 => Ok(InteractionParameter {
                i: 25,
                j: 1,
                a_ij: 106.8000,
            }),
            1026 => Ok(InteractionParameter {
                i: 1,
                j: 26,
                a_ij: 661.5000,
            }),
            26001 => Ok(InteractionParameter {
                i: 26,
                j: 1,
                a_ij: -32.6900,
            }),
            1027 => Ok(InteractionParameter {
                i: 1,
                j: 27,
                a_ij: 543.0000,
            }),
            27001 => Ok(InteractionParameter {
                i: 27,
                j: 1,
                a_ij: 5541.0000,
            }),
            1028 => Ok(InteractionParameter {
                i: 1,
                j: 28,
                a_ij: 153.6000,
            }),
            28001 => Ok(InteractionParameter {
                i: 28,
                j: 1,
                a_ij: -52.6500,
            }),
            1029 => Ok(InteractionParameter {
                i: 1,
                j: 29,
                a_ij: 184.4000,
            }),
            29001 => Ok(InteractionParameter {
                i: 29,
                j: 1,
                a_ij: -7.4810,
            }),
            1030 => Ok(InteractionParameter {
                i: 1,
                j: 30,
                a_ij: 354.5500,
            }),
            30001 => Ok(InteractionParameter {
                i: 30,
                j: 1,
                a_ij: -25.3100,
            }),
            1031 => Ok(InteractionParameter {
                i: 1,
                j: 31,
                a_ij: 3025.0000,
            }),
            31001 => Ok(InteractionParameter {
                i: 31,
                j: 1,
                a_ij: 139.9300,
            }),
            1032 => Ok(InteractionParameter {
                i: 1,
                j: 32,
                a_ij: 335.8000,
            }),
            32001 => Ok(InteractionParameter {
                i: 32,
                j: 1,
                a_ij: 128.0000,
            }),
            1033 => Ok(InteractionParameter {
                i: 1,
                j: 33,
                a_ij: 479.5000,
            }),
            33001 => Ok(InteractionParameter {
                i: 33,
                j: 1,
                a_ij: -31.5200,
            }),
            1034 => Ok(InteractionParameter {
                i: 1,
                j: 34,
                a_ij: 298.9000,
            }),
            34001 => Ok(InteractionParameter {
                i: 34,
                j: 1,
                a_ij: -72.8800,
            }),
            1035 => Ok(InteractionParameter {
                i: 1,
                j: 35,
                a_ij: 526.5000,
            }),
            35001 => Ok(InteractionParameter {
                i: 35,
                j: 1,
                a_ij: 50.4900,
            }),
            1036 => Ok(InteractionParameter {
                i: 1,
                j: 36,
                a_ij: 689.0000,
            }),
            36001 => Ok(InteractionParameter {
                i: 36,
                j: 1,
                a_ij: -165.9000,
            }),
            1037 => Ok(InteractionParameter {
                i: 1,
                j: 37,
                a_ij: -4.1890,
            }),
            37001 => Ok(InteractionParameter {
                i: 37,
                j: 1,
                a_ij: 47.4100,
            }),
            1038 => Ok(InteractionParameter {
                i: 1,
                j: 38,
                a_ij: 125.8000,
            }),
            38001 => Ok(InteractionParameter {
                i: 38,
                j: 1,
                a_ij: -5.1320,
            }),
            1039 => Ok(InteractionParameter {
                i: 1,
                j: 39,
                a_ij: 485.3000,
            }),
            39001 => Ok(InteractionParameter {
                i: 39,
                j: 1,
                a_ij: -31.9500,
            }),
            1040 => Ok(InteractionParameter {
                i: 1,
                j: 40,
                a_ij: -2.8590,
            }),
            40001 => Ok(InteractionParameter {
                i: 40,
                j: 1,
                a_ij: 147.3000,
            }),
            1041 => Ok(InteractionParameter {
                i: 1,
                j: 41,
                a_ij: 387.1000,
            }),
            41001 => Ok(InteractionParameter {
                i: 41,
                j: 1,
                a_ij: 529.0000,
            }),
            1042 => Ok(InteractionParameter {
                i: 1,
                j: 42,
                a_ij: -450.4000,
            }),
            42001 => Ok(InteractionParameter {
                i: 42,
                j: 1,
                a_ij: -34.3600,
            }),
            1043 => Ok(InteractionParameter {
                i: 1,
                j: 43,
                a_ij: 252.7000,
            }),
            43001 => Ok(InteractionParameter {
                i: 43,
                j: 1,
                a_ij: 110.2000,
            }),
            1044 => Ok(InteractionParameter {
                i: 1,
                j: 44,
                a_ij: 220.3000,
            }),
            44001 => Ok(InteractionParameter {
                i: 44,
                j: 1,
                a_ij: 13.8900,
            }),
            1045 => Ok(InteractionParameter {
                i: 1,
                j: 45,
                a_ij: -5.8690,
            }),
            45001 => Ok(InteractionParameter {
                i: 45,
                j: 1,
                a_ij: 30.7400,
            }),
            1046 => Ok(InteractionParameter {
                i: 1,
                j: 46,
                a_ij: 390.9000,
            }),
            46001 => Ok(InteractionParameter {
                i: 46,
                j: 1,
                a_ij: 27.9700,
            }),
            1047 => Ok(InteractionParameter {
                i: 1,
                j: 47,
                a_ij: 553.3000,
            }),
            47001 => Ok(InteractionParameter {
                i: 47,
                j: 1,
                a_ij: -11.9200,
            }),
            1048 => Ok(InteractionParameter {
                i: 1,
                j: 48,
                a_ij: 187.0000,
            }),
            48001 => Ok(InteractionParameter {
                i: 48,
                j: 1,
                a_ij: 39.9300,
            }),
            1049 => Ok(InteractionParameter {
                i: 1,
                j: 49,
                a_ij: 216.1000,
            }),
            49001 => Ok(InteractionParameter {
                i: 49,
                j: 1,
                a_ij: -23.6100,
            }),
            1050 => Ok(InteractionParameter {
                i: 1,
                j: 50,
                a_ij: 92.9900,
            }),
            50001 => Ok(InteractionParameter {
                i: 50,
                j: 1,
                a_ij: -8.4790,
            }),
            1051 => Ok(InteractionParameter {
                i: 1,
                j: 51,
                a_ij: 699.1300,
            }),
            51001 => Ok(InteractionParameter {
                i: 51,
                j: 1,
                a_ij: 456.1900,
            }),
            1055 => Ok(InteractionParameter {
                i: 1,
                j: 55,
                a_ij: 808.5900,
            }),
            55001 => Ok(InteractionParameter {
                i: 55,
                j: 1,
                a_ij: 245.2100,
            }),
            1084 => Ok(InteractionParameter {
                i: 1,
                j: 84,
                a_ij: -1243.0000,
            }),
            84001 => Ok(InteractionParameter {
                i: 84,
                j: 1,
                a_ij: 125.3600,
            }),
            1085 => Ok(InteractionParameter {
                i: 1,
                j: 85,
                a_ij: 637.6500,
            }),
            85001 => Ok(InteractionParameter {
                i: 85,
                j: 1,
                a_ij: 221.5600,
            }),
            2003 => Ok(InteractionParameter {
                i: 2,
                j: 3,
                a_ij: 38.8100,
            }),
            3002 => Ok(InteractionParameter {
                i: 3,
                j: 2,
                a_ij: 3.4460,
            }),
            2004 => Ok(InteractionParameter {
                i: 2,
                j: 4,
                a_ij: 74.1500,
            }),
            4002 => Ok(InteractionParameter {
                i: 4,
                j: 2,
                a_ij: -113.6000,
            }),
            2005 => Ok(InteractionParameter {
                i: 2,
                j: 5,
                a_ij: 524.1000,
            }),
            5002 => Ok(InteractionParameter {
                i: 5,
                j: 2,
                a_ij: 457.0000,
            }),
            2006 => Ok(InteractionParameter {
                i: 2,
                j: 6,
                a_ij: 787.6000,
            }),
            6002 => Ok(InteractionParameter {
                i: 6,
                j: 2,
                a_ij: -12.5200,
            }),
            2007 => Ok(InteractionParameter {
                i: 2,
                j: 7,
                a_ij: 270.6000,
            }),
            7002 => Ok(InteractionParameter {
                i: 7,
                j: 2,
                a_ij: 496.1000,
            }),
            2008 => Ok(InteractionParameter {
                i: 2,
                j: 8,
                a_ij: 526.1000,
            }),
            8002 => Ok(InteractionParameter {
                i: 8,
                j: 2,
                a_ij: 217.5000,
            }),
            2009 => Ok(InteractionParameter {
                i: 2,
                j: 9,
                a_ij: 182.6000,
            }),
            9002 => Ok(InteractionParameter {
                i: 9,
                j: 2,
                a_ij: 42.9200,
            }),
            2010 => Ok(InteractionParameter {
                i: 2,
                j: 10,
                a_ij: 448.7500,
            }),
            10002 => Ok(InteractionParameter {
                i: 10,
                j: 2,
                a_ij: 56.3000,
            }),
            2011 => Ok(InteractionParameter {
                i: 2,
                j: 11,
                a_ij: 37.8500,
            }),
            11002 => Ok(InteractionParameter {
                i: 11,
                j: 2,
                a_ij: 132.1000,
            }),
            2012 => Ok(InteractionParameter {
                i: 2,
                j: 12,
                a_ij: 333.5000,
            }),
            12002 => Ok(InteractionParameter {
                i: 12,
                j: 2,
                a_ij: 110.4000,
            }),
            2013 => Ok(InteractionParameter {
                i: 2,
                j: 13,
                a_ij: 214.5000,
            }),
            13002 => Ok(InteractionParameter {
                i: 13,
                j: 2,
                a_ij: 26.5100,
            }),
            2014 => Ok(InteractionParameter {
                i: 2,
                j: 14,
                a_ij: 240.9000,
            }),
            14002 => Ok(InteractionParameter {
                i: 14,
                j: 2,
                a_ij: 1.1630,
            }),
            2015 => Ok(InteractionParameter {
                i: 2,
                j: 15,
                a_ij: 163.9000,
            }),
            15002 => Ok(InteractionParameter {
                i: 15,
                j: 2,
                a_ij: -28.7000,
            }),
            2016 => Ok(InteractionParameter {
                i: 2,
                j: 16,
                a_ij: 61.1100,
            }),
            16002 => Ok(InteractionParameter {
                i: 16,
                j: 2,
                a_ij: -25.3800,
            }),
            2017 => Ok(InteractionParameter {
                i: 2,
                j: 17,
                a_ij: 749.3000,
            }),
            17002 => Ok(InteractionParameter {
                i: 17,
                j: 2,
                a_ij: 2000.0000,
            }),
            2018 => Ok(InteractionParameter {
                i: 2,
                j: 18,
                a_ij: 280.5000,
            }),
            18002 => Ok(InteractionParameter {
                i: 18,
                j: 2,
                a_ij: -47.6300,
            }),
            2019 => Ok(InteractionParameter {
                i: 2,
                j: 19,
                a_ij: 336.9000,
            }),
            19002 => Ok(InteractionParameter {
                i: 19,
                j: 2,
                a_ij: -40.6200,
            }),
            2020 => Ok(InteractionParameter {
                i: 2,
                j: 20,
                a_ij: 318.9000,
            }),
            20002 => Ok(InteractionParameter {
                i: 20,
                j: 2,
                a_ij: 1264.0000,
            }),
            2021 => Ok(InteractionParameter {
                i: 2,
                j: 21,
                a_ij: -36.8700,
            }),
            21002 => Ok(InteractionParameter {
                i: 21,
                j: 2,
                a_ij: 40.2500,
            }),
            2022 => Ok(InteractionParameter {
                i: 2,
                j: 22,
                a_ij: 58.5500,
            }),
            22002 => Ok(InteractionParameter {
                i: 22,
                j: 2,
                a_ij: -23.5000,
            }),
            2023 => Ok(InteractionParameter {
                i: 2,
                j: 23,
                a_ij: -13.9900,
            }),
            23002 => Ok(InteractionParameter {
                i: 23,
                j: 2,
                a_ij: 51.0600,
            }),
            2024 => Ok(InteractionParameter {
                i: 2,
                j: 24,
                a_ij: -109.7000,
            }),
            24002 => Ok(InteractionParameter {
                i: 24,
                j: 2,
                a_ij: 160.9000,
            }),
            2025 => Ok(InteractionParameter {
                i: 2,
                j: 25,
                a_ij: 100.1000,
            }),
            25002 => Ok(InteractionParameter {
                i: 25,
                j: 2,
                a_ij: 70.3200,
            }),
            2026 => Ok(InteractionParameter {
                i: 2,
                j: 26,
                a_ij: 357.5000,
            }),
            26002 => Ok(InteractionParameter {
                i: 26,
                j: 2,
                a_ij: -1.9960,
            }),
            2028 => Ok(InteractionParameter {
                i: 2,
                j: 28,
                a_ij: 76.3000,
            }),
            28002 => Ok(InteractionParameter {
                i: 28,
                j: 2,
                a_ij: 16.6230,
            }),
            2030 => Ok(InteractionParameter {
                i: 2,
                j: 30,
                a_ij: 262.9000,
            }),
            30002 => Ok(InteractionParameter {
                i: 30,
                j: 2,
                a_ij: 82.6400,
            }),
            2033 => Ok(InteractionParameter {
                i: 2,
                j: 33,
                a_ij: 183.8000,
            }),
            33002 => Ok(InteractionParameter {
                i: 33,
                j: 2,
                a_ij: 174.6000,
            }),
            2034 => Ok(InteractionParameter {
                i: 2,
                j: 34,
                a_ij: 31.1400,
            }),
            34002 => Ok(InteractionParameter {
                i: 34,
                j: 2,
                a_ij: 41.3800,
            }),
            2035 => Ok(InteractionParameter {
                i: 2,
                j: 35,
                a_ij: 179.0000,
            }),
            35002 => Ok(InteractionParameter {
                i: 35,
                j: 2,
                a_ij: 64.0700,
            }),
            2036 => Ok(InteractionParameter {
                i: 2,
                j: 36,
                a_ij: -52.8700,
            }),
            36002 => Ok(InteractionParameter {
                i: 36,
                j: 2,
                a_ij: 573.0000,
            }),
            2037 => Ok(InteractionParameter {
                i: 2,
                j: 37,
                a_ij: -66.4600,
            }),
            37002 => Ok(InteractionParameter {
                i: 37,
                j: 2,
                a_ij: 124.2000,
            }),
            2038 => Ok(InteractionParameter {
                i: 2,
                j: 38,
                a_ij: 359.3000,
            }),
            38002 => Ok(InteractionParameter {
                i: 38,
                j: 2,
                a_ij: -131.7000,
            }),
            2039 => Ok(InteractionParameter {
                i: 2,
                j: 39,
                a_ij: -70.4500,
            }),
            39002 => Ok(InteractionParameter {
                i: 39,
                j: 2,
                a_ij: 249.0000,
            }),
            2040 => Ok(InteractionParameter {
                i: 2,
                j: 40,
                a_ij: 449.4000,
            }),
            40002 => Ok(InteractionParameter {
                i: 40,
                j: 2,
                a_ij: 62.4000,
            }),
            2041 => Ok(InteractionParameter {
                i: 2,
                j: 41,
                a_ij: 48.3300,
            }),
            41002 => Ok(InteractionParameter {
                i: 41,
                j: 2,
                a_ij: 1397.0000,
            }),
            2044 => Ok(InteractionParameter {
                i: 2,
                j: 44,
                a_ij: 86.4600,
            }),
            44002 => Ok(InteractionParameter {
                i: 44,
                j: 2,
                a_ij: -16.1100,
            }),
            2046 => Ok(InteractionParameter {
                i: 2,
                j: 46,
                a_ij: 200.2000,
            }),
            46002 => Ok(InteractionParameter {
                i: 46,
                j: 2,
                a_ij: 9.7550,
            }),
            2047 => Ok(InteractionParameter {
                i: 2,
                j: 47,
                a_ij: 268.1000,
            }),
            47002 => Ok(InteractionParameter {
                i: 47,
                j: 2,
                a_ij: 132.4000,
            }),
            2048 => Ok(InteractionParameter {
                i: 2,
                j: 48,
                a_ij: -617.0000,
            }),
            48002 => Ok(InteractionParameter {
                i: 48,
                j: 2,
                a_ij: 543.6000,
            }),
            2049 => Ok(InteractionParameter {
                i: 2,
                j: 49,
                a_ij: 62.5600,
            }),
            49002 => Ok(InteractionParameter {
                i: 49,
                j: 2,
                a_ij: 161.1000,
            }),
            2055 => Ok(InteractionParameter {
                i: 2,
                j: 55,
                a_ij: 200.9400,
            }),
            55002 => Ok(InteractionParameter {
                i: 55,
                j: 2,
                a_ij: 384.4500,
            }),
            2084 => Ok(InteractionParameter {
                i: 2,
                j: 84,
                a_ij: -861.4600,
            }),
            84002 => Ok(InteractionParameter {
                i: 84,
                j: 2,
                a_ij: -391.8100,
            }),
            2085 => Ok(InteractionParameter {
                i: 2,
                j: 85,
                a_ij: 424.9300,
            }),
            85002 => Ok(InteractionParameter {
                i: 85,
                j: 2,
                a_ij: 629.9600,
            }),
            3004 => Ok(InteractionParameter {
                i: 3,
                j: 4,
                a_ij: 167.0000,
            }),
            4003 => Ok(InteractionParameter {
                i: 4,
                j: 3,
                a_ij: -146.8000,
            }),
            3005 => Ok(InteractionParameter {
                i: 3,
                j: 5,
                a_ij: 636.1000,
            }),
            5003 => Ok(InteractionParameter {
                i: 5,
                j: 3,
                a_ij: 89.6000,
            }),
            3006 => Ok(InteractionParameter {
                i: 3,
                j: 6,
                a_ij: 637.3500,
            }),
            6003 => Ok(InteractionParameter {
                i: 6,
                j: 3,
                a_ij: -50.0000,
            }),
            3007 => Ok(InteractionParameter {
                i: 3,
                j: 7,
                a_ij: 903.8000,
            }),
            7003 => Ok(InteractionParameter {
                i: 7,
                j: 3,
                a_ij: 362.3000,
            }),
            3008 => Ok(InteractionParameter {
                i: 3,
                j: 8,
                a_ij: 1329.0000,
            }),
            8003 => Ok(InteractionParameter {
                i: 8,
                j: 3,
                a_ij: 25.3400,
            }),
            3009 => Ok(InteractionParameter {
                i: 3,
                j: 9,
                a_ij: 25.7700,
            }),
            9003 => Ok(InteractionParameter {
                i: 9,
                j: 3,
                a_ij: 140.1000,
            }),
            3010 => Ok(InteractionParameter {
                i: 3,
                j: 10,
                a_ij: 347.3000,
            }),
            10003 => Ok(InteractionParameter {
                i: 10,
                j: 3,
                a_ij: 23.3900,
            }),
            3011 => Ok(InteractionParameter {
                i: 3,
                j: 11,
                a_ij: 5.9940,
            }),
            11003 => Ok(InteractionParameter {
                i: 11,
                j: 3,
                a_ij: 85.8400,
            }),
            3012 => Ok(InteractionParameter {
                i: 3,
                j: 12,
                a_ij: 287.1000,
            }),
            12003 => Ok(InteractionParameter {
                i: 12,
                j: 3,
                a_ij: 18.1200,
            }),
            3013 => Ok(InteractionParameter {
                i: 3,
                j: 13,
                a_ij: 32.1400,
            }),
            13003 => Ok(InteractionParameter {
                i: 13,
                j: 3,
                a_ij: 52.1300,
            }),
            3014 => Ok(InteractionParameter {
                i: 3,
                j: 14,
                a_ij: 161.7000,
            }),
            14003 => Ok(InteractionParameter {
                i: 14,
                j: 3,
                a_ij: -44.8500,
            }),
            3015 => Ok(InteractionParameter {
                i: 3,
                j: 15,
                a_ij: 122.8000,
            }),
            15003 => Ok(InteractionParameter {
                i: 15,
                j: 3,
                a_ij: -22.3100,
            }),
            3016 => Ok(InteractionParameter {
                i: 3,
                j: 16,
                a_ij: 90.4900,
            }),
            16003 => Ok(InteractionParameter {
                i: 16,
                j: 3,
                a_ij: -223.9000,
            }),
            3017 => Ok(InteractionParameter {
                i: 3,
                j: 17,
                a_ij: 648.2000,
            }),
            17003 => Ok(InteractionParameter {
                i: 17,
                j: 3,
                a_ij: 247.5000,
            }),
            3018 => Ok(InteractionParameter {
                i: 3,
                j: 18,
                a_ij: -4.4490,
            }),
            18003 => Ok(InteractionParameter {
                i: 18,
                j: 3,
                a_ij: 31.8700,
            }),
            3019 => Ok(InteractionParameter {
                i: 3,
                j: 19,
                a_ij: 212.5000,
            }),
            19003 => Ok(InteractionParameter {
                i: 19,
                j: 3,
                a_ij: -22.9700,
            }),
            3020 => Ok(InteractionParameter {
                i: 3,
                j: 20,
                a_ij: 537.4000,
            }),
            20003 => Ok(InteractionParameter {
                i: 20,
                j: 3,
                a_ij: 62.3200,
            }),
            3021 => Ok(InteractionParameter {
                i: 3,
                j: 21,
                a_ij: -18.8100,
            }),
            21003 => Ok(InteractionParameter {
                i: 21,
                j: 3,
                a_ij: 4.6800,
            }),
            3022 => Ok(InteractionParameter {
                i: 3,
                j: 22,
                a_ij: -144.4000,
            }),
            22003 => Ok(InteractionParameter {
                i: 22,
                j: 3,
                a_ij: 121.3000,
            }),
            3023 => Ok(InteractionParameter {
                i: 3,
                j: 23,
                a_ij: -231.9000,
            }),
            23003 => Ok(InteractionParameter {
                i: 23,
                j: 3,
                a_ij: 288.5000,
            }),
            3024 => Ok(InteractionParameter {
                i: 3,
                j: 24,
                a_ij: 3.0000,
            }),
            24003 => Ok(InteractionParameter {
                i: 24,
                j: 3,
                a_ij: -4.7000,
            }),
            3025 => Ok(InteractionParameter {
                i: 3,
                j: 25,
                a_ij: 187.0000,
            }),
            25003 => Ok(InteractionParameter {
                i: 25,
                j: 3,
                a_ij: -97.2700,
            }),
            3026 => Ok(InteractionParameter {
                i: 3,
                j: 26,
                a_ij: 168.0400,
            }),
            26003 => Ok(InteractionParameter {
                i: 26,
                j: 3,
                a_ij: 10.3800,
            }),
            3027 => Ok(InteractionParameter {
                i: 3,
                j: 27,
                a_ij: 194.9000,
            }),
            27003 => Ok(InteractionParameter {
                i: 27,
                j: 3,
                a_ij: 1824.0000,
            }),
            3028 => Ok(InteractionParameter {
                i: 3,
                j: 28,
                a_ij: 52.0680,
            }),
            28003 => Ok(InteractionParameter {
                i: 28,
                j: 3,
                a_ij: 21.4970,
            }),
            3029 => Ok(InteractionParameter {
                i: 3,
                j: 29,
                a_ij: -10.4300,
            }),
            29003 => Ok(InteractionParameter {
                i: 29,
                j: 3,
                a_ij: 28.4100,
            }),
            3030 => Ok(InteractionParameter {
                i: 3,
                j: 30,
                a_ij: -64.6900,
            }),
            30003 => Ok(InteractionParameter {
                i: 30,
                j: 3,
                a_ij: 157.2900,
            }),
            3031 => Ok(InteractionParameter {
                i: 3,
                j: 31,
                a_ij: 210.3660,
            }),
            31003 => Ok(InteractionParameter {
                i: 31,
                j: 3,
                a_ij: 221.4000,
            }),
            3032 => Ok(InteractionParameter {
                i: 3,
                j: 32,
                a_ij: 113.3000,
            }),
            32003 => Ok(InteractionParameter {
                i: 32,
                j: 3,
                a_ij: 58.6800,
            }),
            3033 => Ok(InteractionParameter {
                i: 3,
                j: 33,
                a_ij: 261.3000,
            }),
            33003 => Ok(InteractionParameter {
                i: 33,
                j: 3,
                a_ij: -154.2000,
            }),
            3034 => Ok(InteractionParameter {
                i: 3,
                j: 34,
                a_ij: 154.2600,
            }),
            34003 => Ok(InteractionParameter {
                i: 34,
                j: 3,
                a_ij: -101.1200,
            }),
            3035 => Ok(InteractionParameter {
                i: 3,
                j: 35,
                a_ij: 169.9000,
            }),
            35003 => Ok(InteractionParameter {
                i: 35,
                j: 3,
                a_ij: -2.5040,
            }),
            3036 => Ok(InteractionParameter {
                i: 3,
                j: 36,
                a_ij: 383.9000,
            }),
            36003 => Ok(InteractionParameter {
                i: 36,
                j: 3,
                a_ij: -123.6000,
            }),
            3037 => Ok(InteractionParameter {
                i: 3,
                j: 37,
                a_ij: -259.1000,
            }),
            37003 => Ok(InteractionParameter {
                i: 37,
                j: 3,
                a_ij: 395.8000,
            }),
            3038 => Ok(InteractionParameter {
                i: 3,
                j: 38,
                a_ij: 389.3000,
            }),
            38003 => Ok(InteractionParameter {
                i: 38,
                j: 3,
                a_ij: -237.2000,
            }),
            3039 => Ok(InteractionParameter {
                i: 3,
                j: 39,
                a_ij: 245.6000,
            }),
            39003 => Ok(InteractionParameter {
                i: 39,
                j: 3,
                a_ij: -133.9000,
            }),
            3040 => Ok(InteractionParameter {
                i: 3,
                j: 40,
                a_ij: 22.6700,
            }),
            40003 => Ok(InteractionParameter {
                i: 40,
                j: 3,
                a_ij: 140.6000,
            }),
            3041 => Ok(InteractionParameter {
                i: 3,
                j: 41,
                a_ij: 103.5000,
            }),
            41003 => Ok(InteractionParameter {
                i: 41,
                j: 3,
                a_ij: 317.6000,
            }),
            3042 => Ok(InteractionParameter {
                i: 3,
                j: 42,
                a_ij: -432.3000,
            }),
            42003 => Ok(InteractionParameter {
                i: 42,
                j: 3,
                a_ij: 787.9000,
            }),
            3043 => Ok(InteractionParameter {
                i: 3,
                j: 43,
                a_ij: 238.9000,
            }),
            43003 => Ok(InteractionParameter {
                i: 43,
                j: 3,
                a_ij: 234.4000,
            }),
            3044 => Ok(InteractionParameter {
                i: 3,
                j: 44,
                a_ij: 30.0400,
            }),
            44003 => Ok(InteractionParameter {
                i: 44,
                j: 3,
                a_ij: -23.8800,
            }),
            3045 => Ok(InteractionParameter {
                i: 3,
                j: 45,
                a_ij: -88.1100,
            }),
            45003 => Ok(InteractionParameter {
                i: 45,
                j: 3,
                a_ij: 167.9000,
            }),
            3047 => Ok(InteractionParameter {
                i: 3,
                j: 47,
                a_ij: 333.3000,
            }),
            47003 => Ok(InteractionParameter {
                i: 47,
                j: 3,
                a_ij: -86.8800,
            }),
            3049 => Ok(InteractionParameter {
                i: 3,
                j: 49,
                a_ij: -59.5800,
            }),
            49003 => Ok(InteractionParameter {
                i: 49,
                j: 3,
                a_ij: 142.9000,
            }),
            3050 => Ok(InteractionParameter {
                i: 3,
                j: 50,
                a_ij: -39.1600,
            }),
            50003 => Ok(InteractionParameter {
                i: 50,
                j: 3,
                a_ij: 23.9300,
            }),
            3055 => Ok(InteractionParameter {
                i: 3,
                j: 55,
                a_ij: 360.8200,
            }),
            55003 => Ok(InteractionParameter {
                i: 55,
                j: 3,
                a_ij: 47.0500,
            }),
            4005 => Ok(InteractionParameter {
                i: 4,
                j: 5,
                a_ij: 803.2000,
            }),
            5004 => Ok(InteractionParameter {
                i: 5,
                j: 4,
                a_ij: 25.8200,
            }),
            4006 => Ok(InteractionParameter {
                i: 4,
                j: 6,
                a_ij: 603.2500,
            }),
            6004 => Ok(InteractionParameter {
                i: 6,
                j: 4,
                a_ij: -44.5000,
            }),
            4007 => Ok(InteractionParameter {
                i: 4,
                j: 7,
                a_ij: 5695.0000,
            }),
            7004 => Ok(InteractionParameter {
                i: 7,
                j: 4,
                a_ij: 377.6000,
            }),
            4008 => Ok(InteractionParameter {
                i: 4,
                j: 8,
                a_ij: 884.9000,
            }),
            8004 => Ok(InteractionParameter {
                i: 8,
                j: 4,
                a_ij: 244.2000,
            }),
            4009 => Ok(InteractionParameter {
                i: 4,
                j: 9,
                a_ij: -52.1000,
            }),
            9004 => Ok(InteractionParameter {
                i: 9,
                j: 4,
                a_ij: 365.8000,
            }),
            4010 => Ok(InteractionParameter {
                i: 4,
                j: 10,
                a_ij: 586.8000,
            }),
            10004 => Ok(InteractionParameter {
                i: 10,
                j: 4,
                a_ij: 106.0000,
            }),
            4011 => Ok(InteractionParameter {
                i: 4,
                j: 11,
                a_ij: 5688.0000,
            }),
            11004 => Ok(InteractionParameter {
                i: 11,
                j: 4,
                a_ij: -170.0000,
            }),
            4012 => Ok(InteractionParameter {
                i: 4,
                j: 12,
                a_ij: 197.8000,
            }),
            12004 => Ok(InteractionParameter {
                i: 12,
                j: 4,
                a_ij: 428.0000,
            }),
            4013 => Ok(InteractionParameter {
                i: 4,
                j: 13,
                a_ij: 213.1000,
            }),
            13004 => Ok(InteractionParameter {
                i: 13,
                j: 4,
                a_ij: 65.6900,
            }),
            4014 => Ok(InteractionParameter {
                i: 4,
                j: 14,
                a_ij: 19.0200,
            }),
            14004 => Ok(InteractionParameter {
                i: 14,
                j: 4,
                a_ij: 296.4000,
            }),
            4015 => Ok(InteractionParameter {
                i: 4,
                j: 15,
                a_ij: -49.2900,
            }),
            15004 => Ok(InteractionParameter {
                i: 15,
                j: 4,
                a_ij: 223.0000,
            }),
            4016 => Ok(InteractionParameter {
                i: 4,
                j: 16,
                a_ij: 23.5000,
            }),
            16004 => Ok(InteractionParameter {
                i: 16,
                j: 4,
                a_ij: 109.9000,
            }),
            4017 => Ok(InteractionParameter {
                i: 4,
                j: 17,
                a_ij: 664.2000,
            }),
            17004 => Ok(InteractionParameter {
                i: 17,
                j: 4,
                a_ij: 762.8000,
            }),
            4018 => Ok(InteractionParameter {
                i: 4,
                j: 18,
                a_ij: 52.8000,
            }),
            18004 => Ok(InteractionParameter {
                i: 18,
                j: 4,
                a_ij: 49.8000,
            }),
            4019 => Ok(InteractionParameter {
                i: 4,
                j: 19,
                a_ij: 6096.0000,
            }),
            19004 => Ok(InteractionParameter {
                i: 19,
                j: 4,
                a_ij: -138.4000,
            }),
            4020 => Ok(InteractionParameter {
                i: 4,
                j: 20,
                a_ij: 872.3000,
            }),
            20004 => Ok(InteractionParameter {
                i: 20,
                j: 4,
                a_ij: 89.8600,
            }),
            4021 => Ok(InteractionParameter {
                i: 4,
                j: 21,
                a_ij: -114.1400,
            }),
            21004 => Ok(InteractionParameter {
                i: 21,
                j: 4,
                a_ij: 122.9100,
            }),
            4022 => Ok(InteractionParameter {
                i: 4,
                j: 22,
                a_ij: -111.0000,
            }),
            22004 => Ok(InteractionParameter {
                i: 22,
                j: 4,
                a_ij: 140.7800,
            }),
            4023 => Ok(InteractionParameter {
                i: 4,
                j: 23,
                a_ij: -80.2500,
            }),
            23004 => Ok(InteractionParameter {
                i: 23,
                j: 4,
                a_ij: 69.9000,
            }),
            4024 => Ok(InteractionParameter {
                i: 4,
                j: 24,
                a_ij: -141.3000,
            }),
            24004 => Ok(InteractionParameter {
                i: 24,
                j: 4,
                a_ij: 134.7000,
            }),
            4025 => Ok(InteractionParameter {
                i: 4,
                j: 25,
                a_ij: -211.0000,
            }),
            25004 => Ok(InteractionParameter {
                i: 25,
                j: 4,
                a_ij: 402.5000,
            }),
            4026 => Ok(InteractionParameter {
                i: 4,
                j: 26,
                a_ij: 3629.0000,
            }),
            26004 => Ok(InteractionParameter {
                i: 26,
                j: 4,
                a_ij: -97.0500,
            }),
            4027 => Ok(InteractionParameter {
                i: 4,
                j: 27,
                a_ij: 4448.0000,
            }),
            27004 => Ok(InteractionParameter {
                i: 27,
                j: 4,
                a_ij: -127.8000,
            }),
            4028 => Ok(InteractionParameter {
                i: 4,
                j: 28,
                a_ij: -9.4510,
            }),
            28004 => Ok(InteractionParameter {
                i: 28,
                j: 4,
                a_ij: 40.6750,
            }),
            4029 => Ok(InteractionParameter {
                i: 4,
                j: 29,
                a_ij: 393.6000,
            }),
            29004 => Ok(InteractionParameter {
                i: 29,
                j: 4,
                a_ij: 19.5600,
            }),
            4030 => Ok(InteractionParameter {
                i: 4,
                j: 30,
                a_ij: 48.4900,
            }),
            30004 => Ok(InteractionParameter {
                i: 30,
                j: 4,
                a_ij: 128.8000,
            }),
            4031 => Ok(InteractionParameter {
                i: 4,
                j: 31,
                a_ij: 4975.0000,
            }),
            31004 => Ok(InteractionParameter {
                i: 31,
                j: 4,
                a_ij: 150.6400,
            }),
            4032 => Ok(InteractionParameter {
                i: 4,
                j: 32,
                a_ij: 259.0000,
            }),
            32004 => Ok(InteractionParameter {
                i: 32,
                j: 4,
                a_ij: 26.4100,
            }),
            4033 => Ok(InteractionParameter {
                i: 4,
                j: 33,
                a_ij: 210.0000,
            }),
            33004 => Ok(InteractionParameter {
                i: 33,
                j: 4,
                a_ij: 1112.0000,
            }),
            4034 => Ok(InteractionParameter {
                i: 4,
                j: 34,
                a_ij: -152.5500,
            }),
            34004 => Ok(InteractionParameter {
                i: 34,
                j: 4,
                a_ij: 614.5200,
            }),
            4035 => Ok(InteractionParameter {
                i: 4,
                j: 35,
                a_ij: 4284.0000,
            }),
            35004 => Ok(InteractionParameter {
                i: 35,
                j: 4,
                a_ij: -143.2000,
            }),
            4036 => Ok(InteractionParameter {
                i: 4,
                j: 36,
                a_ij: -119.2000,
            }),
            36004 => Ok(InteractionParameter {
                i: 36,
                j: 4,
                a_ij: 397.4000,
            }),
            4037 => Ok(InteractionParameter {
                i: 4,
                j: 37,
                a_ij: -282.5000,
            }),
            37004 => Ok(InteractionParameter {
                i: 37,
                j: 4,
                a_ij: 419.1000,
            }),
            4038 => Ok(InteractionParameter {
                i: 4,
                j: 38,
                a_ij: 101.4000,
            }),
            38004 => Ok(InteractionParameter {
                i: 38,
                j: 4,
                a_ij: -157.3000,
            }),
            4039 => Ok(InteractionParameter {
                i: 4,
                j: 39,
                a_ij: 5629.0000,
            }),
            39004 => Ok(InteractionParameter {
                i: 39,
                j: 4,
                a_ij: -240.2000,
            }),
            4040 => Ok(InteractionParameter {
                i: 4,
                j: 40,
                a_ij: -245.3900,
            }),
            40004 => Ok(InteractionParameter {
                i: 40,
                j: 4,
                a_ij: 839.8300,
            }),
            4041 => Ok(InteractionParameter {
                i: 4,
                j: 41,
                a_ij: 69.2600,
            }),
            41004 => Ok(InteractionParameter {
                i: 41,
                j: 4,
                a_ij: 615.8000,
            }),
            4044 => Ok(InteractionParameter {
                i: 4,
                j: 44,
                a_ij: 46.3800,
            }),
            44004 => Ok(InteractionParameter {
                i: 44,
                j: 4,
                a_ij: 6.2140,
            }),
            4047 => Ok(InteractionParameter {
                i: 4,
                j: 47,
                a_ij: 421.9000,
            }),
            47004 => Ok(InteractionParameter {
                i: 47,
                j: 4,
                a_ij: -19.4500,
            }),
            4049 => Ok(InteractionParameter {
                i: 4,
                j: 49,
                a_ij: -203.6000,
            }),
            49004 => Ok(InteractionParameter {
                i: 49,
                j: 4,
                a_ij: 274.1000,
            }),
            4050 => Ok(InteractionParameter {
                i: 4,
                j: 50,
                a_ij: 184.9000,
            }),
            50004 => Ok(InteractionParameter {
                i: 50,
                j: 4,
                a_ij: 2.8450,
            }),
            4055 => Ok(InteractionParameter {
                i: 4,
                j: 55,
                a_ij: 233.5100,
            }),
            55004 => Ok(InteractionParameter {
                i: 55,
                j: 4,
                a_ij: 347.1300,
            }),
            5006 => Ok(InteractionParameter {
                i: 5,
                j: 6,
                a_ij: -137.1000,
            }),
            6005 => Ok(InteractionParameter {
                i: 6,
                j: 5,
                a_ij: 249.1000,
            }),
            5007 => Ok(InteractionParameter {
                i: 5,
                j: 7,
                a_ij: 353.5000,
            }),
            7005 => Ok(InteractionParameter {
                i: 7,
                j: 5,
                a_ij: -229.1000,
            }),
            5008 => Ok(InteractionParameter {
                i: 5,
                j: 8,
                a_ij: -259.7000,
            }),
            8005 => Ok(InteractionParameter {
                i: 8,
                j: 5,
                a_ij: -451.6000,
            }),
            5009 => Ok(InteractionParameter {
                i: 5,
                j: 9,
                a_ij: 84.0000,
            }),
            9005 => Ok(InteractionParameter {
                i: 9,
                j: 5,
                a_ij: 164.5000,
            }),
            5010 => Ok(InteractionParameter {
                i: 5,
                j: 10,
                a_ij: -203.6000,
            }),
            10005 => Ok(InteractionParameter {
                i: 10,
                j: 5,
                a_ij: 529.0000,
            }),
            5011 => Ok(InteractionParameter {
                i: 5,
                j: 11,
                a_ij: 101.1000,
            }),
            11005 => Ok(InteractionParameter {
                i: 11,
                j: 5,
                a_ij: 245.4000,
            }),
            5012 => Ok(InteractionParameter {
                i: 5,
                j: 12,
                a_ij: 267.8000,
            }),
            12005 => Ok(InteractionParameter {
                i: 12,
                j: 5,
                a_ij: 139.4000,
            }),
            5013 => Ok(InteractionParameter {
                i: 5,
                j: 13,
                a_ij: 28.0600,
            }),
            13005 => Ok(InteractionParameter {
                i: 13,
                j: 5,
                a_ij: 237.7000,
            }),
            5014 => Ok(InteractionParameter {
                i: 5,
                j: 14,
                a_ij: 8.6420,
            }),
            14005 => Ok(InteractionParameter {
                i: 14,
                j: 5,
                a_ij: -242.8000,
            }),
            5015 => Ok(InteractionParameter {
                i: 5,
                j: 15,
                a_ij: 42.7000,
            }),
            15005 => Ok(InteractionParameter {
                i: 15,
                j: 5,
                a_ij: -150.0000,
            }),
            5016 => Ok(InteractionParameter {
                i: 5,
                j: 16,
                a_ij: -323.0000,
            }),
            16005 => Ok(InteractionParameter {
                i: 16,
                j: 5,
                a_ij: 28.6000,
            }),
            5017 => Ok(InteractionParameter {
                i: 5,
                j: 17,
                a_ij: -52.3900,
            }),
            17005 => Ok(InteractionParameter {
                i: 17,
                j: 5,
                a_ij: -17.4000,
            }),
            5018 => Ok(InteractionParameter {
                i: 5,
                j: 18,
                a_ij: 170.0290,
            }),
            18005 => Ok(InteractionParameter {
                i: 18,
                j: 5,
                a_ij: -132.3000,
            }),
            5019 => Ok(InteractionParameter {
                i: 5,
                j: 19,
                a_ij: 6.7120,
            }),
            19005 => Ok(InteractionParameter {
                i: 19,
                j: 5,
                a_ij: 185.4000,
            }),
            5020 => Ok(InteractionParameter {
                i: 5,
                j: 20,
                a_ij: 199.0000,
            }),
            20005 => Ok(InteractionParameter {
                i: 20,
                j: 5,
                a_ij: -151.0000,
            }),
            5021 => Ok(InteractionParameter {
                i: 5,
                j: 21,
                a_ij: 75.6200,
            }),
            21005 => Ok(InteractionParameter {
                i: 21,
                j: 5,
                a_ij: 562.2000,
            }),
            5022 => Ok(InteractionParameter {
                i: 5,
                j: 22,
                a_ij: 65.2800,
            }),
            22005 => Ok(InteractionParameter {
                i: 22,
                j: 5,
                a_ij: 527.6000,
            }),
            5023 => Ok(InteractionParameter {
                i: 5,
                j: 23,
                a_ij: -98.1200,
            }),
            23005 => Ok(InteractionParameter {
                i: 23,
                j: 5,
                a_ij: 742.1000,
            }),
            5024 => Ok(InteractionParameter {
                i: 5,
                j: 24,
                a_ij: 143.1000,
            }),
            24005 => Ok(InteractionParameter {
                i: 24,
                j: 5,
                a_ij: 856.3000,
            }),
            5025 => Ok(InteractionParameter {
                i: 5,
                j: 25,
                a_ij: 123.5000,
            }),
            25005 => Ok(InteractionParameter {
                i: 25,
                j: 5,
                a_ij: 325.7000,
            }),
            5026 => Ok(InteractionParameter {
                i: 5,
                j: 26,
                a_ij: 256.5000,
            }),
            26005 => Ok(InteractionParameter {
                i: 26,
                j: 5,
                a_ij: 261.6000,
            }),
            5027 => Ok(InteractionParameter {
                i: 5,
                j: 27,
                a_ij: 157.1000,
            }),
            27005 => Ok(InteractionParameter {
                i: 27,
                j: 5,
                a_ij: 561.6000,
            }),
            5028 => Ok(InteractionParameter {
                i: 5,
                j: 28,
                a_ij: 488.9000,
            }),
            28005 => Ok(InteractionParameter {
                i: 28,
                j: 5,
                a_ij: 609.8000,
            }),
            5029 => Ok(InteractionParameter {
                i: 5,
                j: 29,
                a_ij: 147.5000,
            }),
            29005 => Ok(InteractionParameter {
                i: 29,
                j: 5,
                a_ij: 461.6000,
            }),
            5030 => Ok(InteractionParameter {
                i: 5,
                j: 30,
                a_ij: -120.4600,
            }),
            30005 => Ok(InteractionParameter {
                i: 30,
                j: 5,
                a_ij: 521.6300,
            }),
            5031 => Ok(InteractionParameter {
                i: 5,
                j: 31,
                a_ij: -318.9300,
            }),
            31005 => Ok(InteractionParameter {
                i: 31,
                j: 5,
                a_ij: 267.6000,
            }),
            5032 => Ok(InteractionParameter {
                i: 5,
                j: 32,
                a_ij: 313.5000,
            }),
            32005 => Ok(InteractionParameter {
                i: 32,
                j: 5,
                a_ij: 501.3000,
            }),
            5033 => Ok(InteractionParameter {
                i: 5,
                j: 33,
                a_ij: 202.1000,
            }),
            33005 => Ok(InteractionParameter {
                i: 33,
                j: 5,
                a_ij: 524.9000,
            }),
            5034 => Ok(InteractionParameter {
                i: 5,
                j: 34,
                a_ij: 727.8000,
            }),
            34005 => Ok(InteractionParameter {
                i: 34,
                j: 5,
                a_ij: 68.9500,
            }),
            5035 => Ok(InteractionParameter {
                i: 5,
                j: 35,
                a_ij: -202.1000,
            }),
            35005 => Ok(InteractionParameter {
                i: 35,
                j: 5,
                a_ij: -25.8700,
            }),
            5036 => Ok(InteractionParameter {
                i: 5,
                j: 36,
                a_ij: 74.2700,
            }),
            36005 => Ok(InteractionParameter {
                i: 36,
                j: 5,
                a_ij: 389.3000,
            }),
            5037 => Ok(InteractionParameter {
                i: 5,
                j: 37,
                a_ij: 225.8000,
            }),
            37005 => Ok(InteractionParameter {
                i: 37,
                j: 5,
                a_ij: 738.9000,
            }),
            5038 => Ok(InteractionParameter {
                i: 5,
                j: 38,
                a_ij: 44.7800,
            }),
            38005 => Ok(InteractionParameter {
                i: 38,
                j: 5,
                a_ij: 649.7000,
            }),
            5039 => Ok(InteractionParameter {
                i: 5,
                j: 39,
                a_ij: -143.9000,
            }),
            39005 => Ok(InteractionParameter {
                i: 39,
                j: 5,
                a_ij: 64.1600,
            }),
            5041 => Ok(InteractionParameter {
                i: 5,
                j: 41,
                a_ij: 190.3000,
            }),
            41005 => Ok(InteractionParameter {
                i: 41,
                j: 5,
                a_ij: 88.6300,
            }),
            5042 => Ok(InteractionParameter {
                i: 5,
                j: 42,
                a_ij: -817.7000,
            }),
            42005 => Ok(InteractionParameter {
                i: 42,
                j: 5,
                a_ij: 1913.0000,
            }),
            5043 => Ok(InteractionParameter {
                i: 5,
                j: 43,
                a_ij: -1712.8000,
            }),
            43005 => Ok(InteractionParameter {
                i: 43,
                j: 5,
                a_ij: 430.0600,
            }),
            5044 => Ok(InteractionParameter {
                i: 5,
                j: 44,
                a_ij: -504.2000,
            }),
            44005 => Ok(InteractionParameter {
                i: 44,
                j: 5,
                a_ij: 796.9000,
            }),
            5045 => Ok(InteractionParameter {
                i: 5,
                j: 45,
                a_ij: 72.9600,
            }),
            45005 => Ok(InteractionParameter {
                i: 45,
                j: 5,
                a_ij: 794.4000,
            }),
            5046 => Ok(InteractionParameter {
                i: 5,
                j: 46,
                a_ij: -382.7000,
            }),
            46005 => Ok(InteractionParameter {
                i: 46,
                j: 5,
                a_ij: 394.8000,
            }),
            5047 => Ok(InteractionParameter {
                i: 5,
                j: 47,
                a_ij: -248.3000,
            }),
            47005 => Ok(InteractionParameter {
                i: 47,
                j: 5,
                a_ij: 517.5000,
            }),
            5049 => Ok(InteractionParameter {
                i: 5,
                j: 49,
                a_ij: 104.7000,
            }),
            49005 => Ok(InteractionParameter {
                i: 49,
                j: 5,
                a_ij: -61.2000,
            }),
            5050 => Ok(InteractionParameter {
                i: 5,
                j: 50,
                a_ij: 57.6500,
            }),
            50005 => Ok(InteractionParameter {
                i: 50,
                j: 5,
                a_ij: 682.5000,
            }),
            5055 => Ok(InteractionParameter {
                i: 5,
                j: 55,
                a_ij: 215.8100,
            }),
            55005 => Ok(InteractionParameter {
                i: 55,
                j: 5,
                a_ij: 72.1900,
            }),
            5084 => Ok(InteractionParameter {
                i: 5,
                j: 84,
                a_ij: -1840.8000,
            }),
            84005 => Ok(InteractionParameter {
                i: 84,
                j: 5,
                a_ij: 111.6500,
            }),
            5085 => Ok(InteractionParameter {
                i: 5,
                j: 85,
                a_ij: 56.2980,
            }),
            85005 => Ok(InteractionParameter {
                i: 85,
                j: 5,
                a_ij: 122.1900,
            }),
            6007 => Ok(InteractionParameter {
                i: 6,
                j: 7,
                a_ij: -180.9500,
            }),
            7006 => Ok(InteractionParameter {
                i: 7,
                j: 6,
                a_ij: 289.6000,
            }),
            6008 => Ok(InteractionParameter {
                i: 6,
                j: 8,
                a_ij: -101.7000,
            }),
            8006 => Ok(InteractionParameter {
                i: 8,
                j: 6,
                a_ij: -265.2000,
            }),
            6009 => Ok(InteractionParameter {
                i: 6,
                j: 9,
                a_ij: 23.3900,
            }),
            9006 => Ok(InteractionParameter {
                i: 9,
                j: 6,
                a_ij: 108.6500,
            }),
            6010 => Ok(InteractionParameter {
                i: 6,
                j: 10,
                a_ij: 306.4200,
            }),
            10006 => Ok(InteractionParameter {
                i: 10,
                j: 6,
                a_ij: -340.1800,
            }),
            6011 => Ok(InteractionParameter {
                i: 6,
                j: 11,
                a_ij: -10.7200,
            }),
            11006 => Ok(InteractionParameter {
                i: 11,
                j: 6,
                a_ij: 249.6300,
            }),
            6012 => Ok(InteractionParameter {
                i: 6,
                j: 12,
                a_ij: 179.7000,
            }),
            12006 => Ok(InteractionParameter {
                i: 12,
                j: 6,
                a_ij: 227.8000,
            }),
            6013 => Ok(InteractionParameter {
                i: 6,
                j: 13,
                a_ij: -128.6000,
            }),
            13006 => Ok(InteractionParameter {
                i: 13,
                j: 6,
                a_ij: 238.4000,
            }),
            6014 => Ok(InteractionParameter {
                i: 6,
                j: 14,
                a_ij: 359.3000,
            }),
            14006 => Ok(InteractionParameter {
                i: 14,
                j: 6,
                a_ij: -481.6500,
            }),
            6015 => Ok(InteractionParameter {
                i: 6,
                j: 15,
                a_ij: -20.9800,
            }),
            15006 => Ok(InteractionParameter {
                i: 15,
                j: 6,
                a_ij: -370.3000,
            }),
            6016 => Ok(InteractionParameter {
                i: 6,
                j: 16,
                a_ij: 53.9000,
            }),
            16006 => Ok(InteractionParameter {
                i: 16,
                j: 6,
                a_ij: -406.8000,
            }),
            6017 => Ok(InteractionParameter {
                i: 6,
                j: 17,
                a_ij: 489.7000,
            }),
            17006 => Ok(InteractionParameter {
                i: 17,
                j: 6,
                a_ij: -118.1000,
            }),
            6018 => Ok(InteractionParameter {
                i: 6,
                j: 18,
                a_ij: 580.4800,
            }),
            18006 => Ok(InteractionParameter {
                i: 18,
                j: 6,
                a_ij: -378.2400,
            }),
            6019 => Ok(InteractionParameter {
                i: 6,
                j: 19,
                a_ij: 53.2800,
            }),
            19006 => Ok(InteractionParameter {
                i: 19,
                j: 6,
                a_ij: 162.6000,
            }),
            6020 => Ok(InteractionParameter {
                i: 6,
                j: 20,
                a_ij: -202.0000,
            }),
            20006 => Ok(InteractionParameter {
                i: 20,
                j: 6,
                a_ij: 339.8000,
            }),
            6021 => Ok(InteractionParameter {
                i: 6,
                j: 21,
                a_ij: -38.3200,
            }),
            21006 => Ok(InteractionParameter {
                i: 21,
                j: 6,
                a_ij: 529.0000,
            }),
            6022 => Ok(InteractionParameter {
                i: 6,
                j: 22,
                a_ij: -102.5400,
            }),
            22006 => Ok(InteractionParameter {
                i: 22,
                j: 6,
                a_ij: 669.9000,
            }),
            6023 => Ok(InteractionParameter {
                i: 6,
                j: 23,
                a_ij: -139.3500,
            }),
            23006 => Ok(InteractionParameter {
                i: 23,
                j: 6,
                a_ij: 649.1000,
            }),
            6024 => Ok(InteractionParameter {
                i: 6,
                j: 24,
                a_ij: -44.7600,
            }),
            24006 => Ok(InteractionParameter {
                i: 24,
                j: 6,
                a_ij: 709.6000,
            }),
            6025 => Ok(InteractionParameter {
                i: 6,
                j: 25,
                a_ij: -28.2500,
            }),
            25006 => Ok(InteractionParameter {
                i: 25,
                j: 6,
                a_ij: 612.8000,
            }),
            6026 => Ok(InteractionParameter {
                i: 6,
                j: 26,
                a_ij: 75.1400,
            }),
            26006 => Ok(InteractionParameter {
                i: 26,
                j: 6,
                a_ij: 252.5600,
            }),
            6027 => Ok(InteractionParameter {
                i: 6,
                j: 27,
                a_ij: 457.8800,
            }),
            27006 => Ok(InteractionParameter {
                i: 27,
                j: 6,
                a_ij: 511.2900,
            }),
            6028 => Ok(InteractionParameter {
                i: 6,
                j: 28,
                a_ij: -31.0900,
            }),
            28006 => Ok(InteractionParameter {
                i: 28,
                j: 6,
                a_ij: 914.2000,
            }),
            6029 => Ok(InteractionParameter {
                i: 6,
                j: 29,
                a_ij: 17.5000,
            }),
            29006 => Ok(InteractionParameter {
                i: 29,
                j: 6,
                a_ij: 448.6000,
            }),
            6030 => Ok(InteractionParameter {
                i: 6,
                j: 30,
                a_ij: -61.7600,
            }),
            30006 => Ok(InteractionParameter {
                i: 30,
                j: 6,
                a_ij: 287.0000,
            }),
            6031 => Ok(InteractionParameter {
                i: 6,
                j: 31,
                a_ij: -119.2000,
            }),
            31006 => Ok(InteractionParameter {
                i: 31,
                j: 6,
                a_ij: 240.8000,
            }),
            6032 => Ok(InteractionParameter {
                i: 6,
                j: 32,
                a_ij: 212.1000,
            }),
            32006 => Ok(InteractionParameter {
                i: 32,
                j: 6,
                a_ij: 431.3000,
            }),
            6033 => Ok(InteractionParameter {
                i: 6,
                j: 33,
                a_ij: 106.3000,
            }),
            33006 => Ok(InteractionParameter {
                i: 33,
                j: 6,
                a_ij: 494.7000,
            }),
            6034 => Ok(InteractionParameter {
                i: 6,
                j: 34,
                a_ij: -119.1000,
            }),
            34006 => Ok(InteractionParameter {
                i: 34,
                j: 6,
                a_ij: 967.7100,
            }),
            6035 => Ok(InteractionParameter {
                i: 6,
                j: 35,
                a_ij: -399.3000,
            }),
            35006 => Ok(InteractionParameter {
                i: 35,
                j: 6,
                a_ij: 695.0000,
            }),
            6036 => Ok(InteractionParameter {
                i: 6,
                j: 36,
                a_ij: -5.2240,
            }),
            36006 => Ok(InteractionParameter {
                i: 36,
                j: 6,
                a_ij: 218.8000,
            }),
            6037 => Ok(InteractionParameter {
                i: 6,
                j: 37,
                a_ij: 33.4700,
            }),
            37006 => Ok(InteractionParameter {
                i: 37,
                j: 6,
                a_ij: 528.0000,
            }),
            6038 => Ok(InteractionParameter {
                i: 6,
                j: 38,
                a_ij: -48.2500,
            }),
            38006 => Ok(InteractionParameter {
                i: 38,
                j: 6,
                a_ij: 645.9000,
            }),
            6039 => Ok(InteractionParameter {
                i: 6,
                j: 39,
                a_ij: -172.4000,
            }),
            39006 => Ok(InteractionParameter {
                i: 39,
                j: 6,
                a_ij: 172.2000,
            }),
            6041 => Ok(InteractionParameter {
                i: 6,
                j: 41,
                a_ij: 165.7000,
            }),
            41006 => Ok(InteractionParameter {
                i: 41,
                j: 6,
                a_ij: 171.0000,
            }),
            6045 => Ok(InteractionParameter {
                i: 6,
                j: 45,
                a_ij: -52.1000,
            }),
            45006 => Ok(InteractionParameter {
                i: 45,
                j: 6,
                a_ij: 762.7000,
            }),
            6048 => Ok(InteractionParameter {
                i: 6,
                j: 48,
                a_ij: 37.6300,
            }),
            48006 => Ok(InteractionParameter {
                i: 48,
                j: 6,
                a_ij: 420.0000,
            }),
            6049 => Ok(InteractionParameter {
                i: 6,
                j: 49,
                a_ij: -59.4000,
            }),
            49006 => Ok(InteractionParameter {
                i: 49,
                j: 6,
                a_ij: -89.2400,
            }),
            6050 => Ok(InteractionParameter {
                i: 6,
                j: 50,
                a_ij: -46.0100,
            }),
            50006 => Ok(InteractionParameter {
                i: 50,
                j: 6,
                a_ij: 597.8000,
            }),
            6055 => Ok(InteractionParameter {
                i: 6,
                j: 55,
                a_ij: 150.0200,
            }),
            55006 => Ok(InteractionParameter {
                i: 55,
                j: 6,
                a_ij: 265.7500,
            }),
            7008 => Ok(InteractionParameter {
                i: 7,
                j: 8,
                a_ij: 324.5000,
            }),
            8007 => Ok(InteractionParameter {
                i: 8,
                j: 7,
                a_ij: -601.8000,
            }),
            7009 => Ok(InteractionParameter {
                i: 7,
                j: 9,
                a_ij: -195.4000,
            }),
            9007 => Ok(InteractionParameter {
                i: 9,
                j: 7,
                a_ij: 472.5000,
            }),
            7010 => Ok(InteractionParameter {
                i: 7,
                j: 10,
                a_ij: -116.0000,
            }),
            10007 => Ok(InteractionParameter {
                i: 10,
                j: 7,
                a_ij: 480.8000,
            }),
            7011 => Ok(InteractionParameter {
                i: 7,
                j: 11,
                a_ij: 72.8700,
            }),
            11007 => Ok(InteractionParameter {
                i: 11,
                j: 7,
                a_ij: 200.8000,
            }),
            7012 => Ok(InteractionParameter {
                i: 7,
                j: 12,
                a_ij: 233.8700,
            }),
            12007 => Ok(InteractionParameter {
                i: 12,
                j: 7,
                a_ij: 124.6300,
            }),
            7013 => Ok(InteractionParameter {
                i: 7,
                j: 13,
                a_ij: 540.5000,
            }),
            13007 => Ok(InteractionParameter {
                i: 13,
                j: 7,
                a_ij: -314.7000,
            }),
            7014 => Ok(InteractionParameter {
                i: 7,
                j: 14,
                a_ij: 48.8900,
            }),
            14007 => Ok(InteractionParameter {
                i: 14,
                j: 7,
                a_ij: -330.4000,
            }),
            7015 => Ok(InteractionParameter {
                i: 7,
                j: 15,
                a_ij: 168.0000,
            }),
            15007 => Ok(InteractionParameter {
                i: 15,
                j: 7,
                a_ij: -448.2000,
            }),
            7016 => Ok(InteractionParameter {
                i: 7,
                j: 16,
                a_ij: 304.0000,
            }),
            16007 => Ok(InteractionParameter {
                i: 16,
                j: 7,
                a_ij: -598.8000,
            }),
            7017 => Ok(InteractionParameter {
                i: 7,
                j: 17,
                a_ij: 243.2000,
            }),
            17007 => Ok(InteractionParameter {
                i: 17,
                j: 7,
                a_ij: -341.6000,
            }),
            7018 => Ok(InteractionParameter {
                i: 7,
                j: 18,
                a_ij: 459.0000,
            }),
            18007 => Ok(InteractionParameter {
                i: 18,
                j: 7,
                a_ij: -332.9000,
            }),
            7019 => Ok(InteractionParameter {
                i: 7,
                j: 19,
                a_ij: 112.6000,
            }),
            19007 => Ok(InteractionParameter {
                i: 19,
                j: 7,
                a_ij: 242.8000,
            }),
            7020 => Ok(InteractionParameter {
                i: 7,
                j: 20,
                a_ij: -14.0900,
            }),
            20007 => Ok(InteractionParameter {
                i: 20,
                j: 7,
                a_ij: -66.1700,
            }),
            7021 => Ok(InteractionParameter {
                i: 7,
                j: 21,
                a_ij: 325.4400,
            }),
            21007 => Ok(InteractionParameter {
                i: 21,
                j: 7,
                a_ij: 698.2400,
            }),
            7022 => Ok(InteractionParameter {
                i: 7,
                j: 22,
                a_ij: 370.4000,
            }),
            22007 => Ok(InteractionParameter {
                i: 22,
                j: 7,
                a_ij: 708.6900,
            }),
            7023 => Ok(InteractionParameter {
                i: 7,
                j: 23,
                a_ij: 353.6800,
            }),
            23007 => Ok(InteractionParameter {
                i: 23,
                j: 7,
                a_ij: 826.7600,
            }),
            7024 => Ok(InteractionParameter {
                i: 7,
                j: 24,
                a_ij: 497.5400,
            }),
            24007 => Ok(InteractionParameter {
                i: 24,
                j: 7,
                a_ij: 1201.0000,
            }),
            7025 => Ok(InteractionParameter {
                i: 7,
                j: 25,
                a_ij: 133.9000,
            }),
            25007 => Ok(InteractionParameter {
                i: 25,
                j: 7,
                a_ij: -274.5000,
            }),
            7026 => Ok(InteractionParameter {
                i: 7,
                j: 26,
                a_ij: 220.6000,
            }),
            26007 => Ok(InteractionParameter {
                i: 26,
                j: 7,
                a_ij: 417.9000,
            }),
            7027 => Ok(InteractionParameter {
                i: 7,
                j: 27,
                a_ij: 399.5000,
            }),
            27007 => Ok(InteractionParameter {
                i: 27,
                j: 7,
                a_ij: 360.7000,
            }),
            7028 => Ok(InteractionParameter {
                i: 7,
                j: 28,
                a_ij: 887.1000,
            }),
            28007 => Ok(InteractionParameter {
                i: 28,
                j: 7,
                a_ij: 1081.0000,
            }),
            7030 => Ok(InteractionParameter {
                i: 7,
                j: 30,
                a_ij: 188.0260,
            }),
            30007 => Ok(InteractionParameter {
                i: 30,
                j: 7,
                a_ij: 23.4840,
            }),
            7031 => Ok(InteractionParameter {
                i: 7,
                j: 31,
                a_ij: 12.7200,
            }),
            31007 => Ok(InteractionParameter {
                i: 31,
                j: 7,
                a_ij: -137.4000,
            }),
            7033 => Ok(InteractionParameter {
                i: 7,
                j: 33,
                a_ij: 777.1000,
            }),
            33007 => Ok(InteractionParameter {
                i: 33,
                j: 7,
                a_ij: 79.1800,
            }),
            7035 => Ok(InteractionParameter {
                i: 7,
                j: 35,
                a_ij: -139.0000,
            }),
            35007 => Ok(InteractionParameter {
                i: 35,
                j: 7,
                a_ij: -240.0000,
            }),
            7036 => Ok(InteractionParameter {
                i: 7,
                j: 36,
                a_ij: 160.8000,
            }),
            36007 => Ok(InteractionParameter {
                i: 36,
                j: 7,
                a_ij: 386.6000,
            }),
            7039 => Ok(InteractionParameter {
                i: 7,
                j: 39,
                a_ij: 319.0000,
            }),
            39007 => Ok(InteractionParameter {
                i: 39,
                j: 7,
                a_ij: -287.1000,
            }),
            7041 => Ok(InteractionParameter {
                i: 7,
                j: 41,
                a_ij: -197.5000,
            }),
            41007 => Ok(InteractionParameter {
                i: 41,
                j: 7,
                a_ij: 284.4000,
            }),
            7042 => Ok(InteractionParameter {
                i: 7,
                j: 42,
                a_ij: -363.8000,
            }),
            42007 => Ok(InteractionParameter {
                i: 42,
                j: 7,
                a_ij: 180.2000,
            }),
            7044 => Ok(InteractionParameter {
                i: 7,
                j: 44,
                a_ij: -452.2000,
            }),
            44007 => Ok(InteractionParameter {
                i: 44,
                j: 7,
                a_ij: 832.2000,
            }),
            7046 => Ok(InteractionParameter {
                i: 7,
                j: 46,
                a_ij: 835.6000,
            }),
            46007 => Ok(InteractionParameter {
                i: 46,
                j: 7,
                a_ij: -509.3000,
            }),
            7047 => Ok(InteractionParameter {
                i: 7,
                j: 47,
                a_ij: 139.6000,
            }),
            47007 => Ok(InteractionParameter {
                i: 47,
                j: 7,
                a_ij: -205.7000,
            }),
            7049 => Ok(InteractionParameter {
                i: 7,
                j: 49,
                a_ij: 407.9000,
            }),
            49007 => Ok(InteractionParameter {
                i: 49,
                j: 7,
                a_ij: -384.3000,
            }),
            7055 => Ok(InteractionParameter {
                i: 7,
                j: 55,
                a_ij: -255.6300,
            }),
            55007 => Ok(InteractionParameter {
                i: 55,
                j: 7,
                a_ij: 627.3900,
            }),
            8009 => Ok(InteractionParameter {
                i: 8,
                j: 9,
                a_ij: -356.1000,
            }),
            9008 => Ok(InteractionParameter {
                i: 9,
                j: 8,
                a_ij: -133.1000,
            }),
            8010 => Ok(InteractionParameter {
                i: 8,
                j: 10,
                a_ij: -271.1000,
            }),
            10008 => Ok(InteractionParameter {
                i: 10,
                j: 8,
                a_ij: -155.6000,
            }),
            8011 => Ok(InteractionParameter {
                i: 8,
                j: 11,
                a_ij: -449.4000,
            }),
            11008 => Ok(InteractionParameter {
                i: 11,
                j: 8,
                a_ij: -36.7200,
            }),
            8012 => Ok(InteractionParameter {
                i: 8,
                j: 12,
                a_ij: -32.5200,
            }),
            12008 => Ok(InteractionParameter {
                i: 12,
                j: 8,
                a_ij: -234.2500,
            }),
            8013 => Ok(InteractionParameter {
                i: 8,
                j: 13,
                a_ij: -162.8742,
            }),
            13008 => Ok(InteractionParameter {
                i: 13,
                j: 8,
                a_ij: -178.5461,
            }),
            8014 => Ok(InteractionParameter {
                i: 8,
                j: 14,
                a_ij: -832.9700,
            }),
            14008 => Ok(InteractionParameter {
                i: 14,
                j: 8,
                a_ij: -870.8000,
            }),
            8017 => Ok(InteractionParameter {
                i: 8,
                j: 17,
                a_ij: 119.9000,
            }),
            17008 => Ok(InteractionParameter {
                i: 17,
                j: 8,
                a_ij: -253.1000,
            }),
            8018 => Ok(InteractionParameter {
                i: 8,
                j: 18,
                a_ij: -305.5000,
            }),
            18008 => Ok(InteractionParameter {
                i: 18,
                j: 8,
                a_ij: -341.6000,
            }),
            8020 => Ok(InteractionParameter {
                i: 8,
                j: 20,
                a_ij: 408.9000,
            }),
            20008 => Ok(InteractionParameter {
                i: 20,
                j: 8,
                a_ij: -11.0000,
            }),
            8022 => Ok(InteractionParameter {
                i: 8,
                j: 22,
                a_ij: 517.2700,
            }),
            22008 => Ok(InteractionParameter {
                i: 22,
                j: 8,
                a_ij: 1633.5000,
            }),
            8024 => Ok(InteractionParameter {
                i: 8,
                j: 24,
                a_ij: 1827.0000,
            }),
            24008 => Ok(InteractionParameter {
                i: 24,
                j: 8,
                a_ij: 10000.0000,
            }),
            8025 => Ok(InteractionParameter {
                i: 8,
                j: 25,
                a_ij: 6915.0000,
            }),
            25008 => Ok(InteractionParameter {
                i: 25,
                j: 8,
                a_ij: 622.3000,
            }),
            8027 => Ok(InteractionParameter {
                i: 8,
                j: 27,
                a_ij: -413.4800,
            }),
            27008 => Ok(InteractionParameter {
                i: 27,
                j: 8,
                a_ij: 815.1200,
            }),
            8028 => Ok(InteractionParameter {
                i: 8,
                j: 28,
                a_ij: 8483.5000,
            }),
            28008 => Ok(InteractionParameter {
                i: 28,
                j: 8,
                a_ij: 1421.3000,
            }),
            8031 => Ok(InteractionParameter {
                i: 8,
                j: 31,
                a_ij: -687.1000,
            }),
            31008 => Ok(InteractionParameter {
                i: 31,
                j: 8,
                a_ij: 838.4000,
            }),
            8041 => Ok(InteractionParameter {
                i: 8,
                j: 41,
                a_ij: -494.2000,
            }),
            41008 => Ok(InteractionParameter {
                i: 41,
                j: 8,
                a_ij: -167.3000,
            }),
            8044 => Ok(InteractionParameter {
                i: 8,
                j: 44,
                a_ij: -659.0000,
            }),
            44008 => Ok(InteractionParameter {
                i: 44,
                j: 8,
                a_ij: -234.7000,
            }),
            8050 => Ok(InteractionParameter {
                i: 8,
                j: 50,
                a_ij: 1005.0000,
            }),
            50008 => Ok(InteractionParameter {
                i: 50,
                j: 8,
                a_ij: 810.5000,
            }),
            9010 => Ok(InteractionParameter {
                i: 9,
                j: 10,
                a_ij: -37.3600,
            }),
            10009 => Ok(InteractionParameter {
                i: 10,
                j: 9,
                a_ij: 128.0000,
            }),
            9011 => Ok(InteractionParameter {
                i: 9,
                j: 11,
                a_ij: -213.7000,
            }),
            11009 => Ok(InteractionParameter {
                i: 11,
                j: 9,
                a_ij: 372.2000,
            }),
            9012 => Ok(InteractionParameter {
                i: 9,
                j: 12,
                a_ij: -190.4000,
            }),
            12009 => Ok(InteractionParameter {
                i: 12,
                j: 9,
                a_ij: 385.4000,
            }),
            9013 => Ok(InteractionParameter {
                i: 9,
                j: 13,
                a_ij: -103.6000,
            }),
            13009 => Ok(InteractionParameter {
                i: 13,
                j: 9,
                a_ij: 191.1000,
            }),
            9015 => Ok(InteractionParameter {
                i: 9,
                j: 15,
                a_ij: -174.2000,
            }),
            15009 => Ok(InteractionParameter {
                i: 15,
                j: 9,
                a_ij: 394.6000,
            }),
            9016 => Ok(InteractionParameter {
                i: 9,
                j: 16,
                a_ij: -169.0000,
            }),
            16009 => Ok(InteractionParameter {
                i: 16,
                j: 9,
                a_ij: 225.3000,
            }),
            9017 => Ok(InteractionParameter {
                i: 9,
                j: 17,
                a_ij: 6201.0000,
            }),
            17009 => Ok(InteractionParameter {
                i: 17,
                j: 9,
                a_ij: -450.3000,
            }),
            9018 => Ok(InteractionParameter {
                i: 9,
                j: 18,
                a_ij: 7.3410,
            }),
            18009 => Ok(InteractionParameter {
                i: 18,
                j: 9,
                a_ij: 29.1000,
            }),
            9019 => Ok(InteractionParameter {
                i: 9,
                j: 19,
                a_ij: 481.7000,
            }),
            19009 => Ok(InteractionParameter {
                i: 19,
                j: 9,
                a_ij: -287.5000,
            }),
            9020 => Ok(InteractionParameter {
                i: 9,
                j: 20,
                a_ij: 669.4000,
            }),
            20009 => Ok(InteractionParameter {
                i: 20,
                j: 9,
                a_ij: -297.8000,
            }),
            9021 => Ok(InteractionParameter {
                i: 9,
                j: 21,
                a_ij: -191.6900,
            }),
            21009 => Ok(InteractionParameter {
                i: 21,
                j: 9,
                a_ij: 286.2800,
            }),
            9022 => Ok(InteractionParameter {
                i: 9,
                j: 22,
                a_ij: -130.3000,
            }),
            22009 => Ok(InteractionParameter {
                i: 22,
                j: 9,
                a_ij: 82.8600,
            }),
            9023 => Ok(InteractionParameter {
                i: 9,
                j: 23,
                a_ij: -354.5500,
            }),
            23009 => Ok(InteractionParameter {
                i: 23,
                j: 9,
                a_ij: 552.1000,
            }),
            9024 => Ok(InteractionParameter {
                i: 9,
                j: 24,
                a_ij: -39.2000,
            }),
            24009 => Ok(InteractionParameter {
                i: 24,
                j: 9,
                a_ij: 372.0000,
            }),
            9025 => Ok(InteractionParameter {
                i: 9,
                j: 25,
                a_ij: -119.8000,
            }),
            25009 => Ok(InteractionParameter {
                i: 25,
                j: 9,
                a_ij: 518.4000,
            }),
            9026 => Ok(InteractionParameter {
                i: 9,
                j: 26,
                a_ij: 137.5000,
            }),
            26009 => Ok(InteractionParameter {
                i: 26,
                j: 9,
                a_ij: -142.6100,
            }),
            9027 => Ok(InteractionParameter {
                i: 9,
                j: 27,
                a_ij: 548.5000,
            }),
            27009 => Ok(InteractionParameter {
                i: 27,
                j: 9,
                a_ij: -101.5000,
            }),
            9028 => Ok(InteractionParameter {
                i: 9,
                j: 28,
                a_ij: 216.1380,
            }),
            28009 => Ok(InteractionParameter {
                i: 28,
                j: 9,
                a_ij: 303.6570,
            }),
            9029 => Ok(InteractionParameter {
                i: 9,
                j: 29,
                a_ij: -46.2800,
            }),
            29009 => Ok(InteractionParameter {
                i: 29,
                j: 9,
                a_ij: 160.6000,
            }),
            9030 => Ok(InteractionParameter {
                i: 9,
                j: 30,
                a_ij: -163.7000,
            }),
            30009 => Ok(InteractionParameter {
                i: 30,
                j: 9,
                a_ij: 317.5000,
            }),
            9031 => Ok(InteractionParameter {
                i: 9,
                j: 31,
                a_ij: 71.4600,
            }),
            31009 => Ok(InteractionParameter {
                i: 31,
                j: 9,
                a_ij: 135.4000,
            }),
            9032 => Ok(InteractionParameter {
                i: 9,
                j: 32,
                a_ij: 53.5900,
            }),
            32009 => Ok(InteractionParameter {
                i: 32,
                j: 9,
                a_ij: 138.0000,
            }),
            9033 => Ok(InteractionParameter {
                i: 9,
                j: 33,
                a_ij: 245.2000,
            }),
            33009 => Ok(InteractionParameter {
                i: 33,
                j: 9,
                a_ij: -142.6000,
            }),
            9034 => Ok(InteractionParameter {
                i: 9,
                j: 34,
                a_ij: -246.6000,
            }),
            34009 => Ok(InteractionParameter {
                i: 34,
                j: 9,
                a_ij: 443.6150,
            }),
            9035 => Ok(InteractionParameter {
                i: 9,
                j: 35,
                a_ij: -44.5800,
            }),
            35009 => Ok(InteractionParameter {
                i: 35,
                j: 9,
                a_ij: 110.4000,
            }),
            9036 => Ok(InteractionParameter {
                i: 9,
                j: 36,
                a_ij: -63.5000,
            }),
            36009 => Ok(InteractionParameter {
                i: 36,
                j: 9,
                a_ij: 114.5500,
            }),
            9037 => Ok(InteractionParameter {
                i: 9,
                j: 37,
                a_ij: -34.5700,
            }),
            37009 => Ok(InteractionParameter {
                i: 37,
                j: 9,
                a_ij: -40.9000,
            }),
            9039 => Ok(InteractionParameter {
                i: 9,
                j: 39,
                a_ij: -61.7000,
            }),
            39009 => Ok(InteractionParameter {
                i: 39,
                j: 9,
                a_ij: 97.0400,
            }),
            9041 => Ok(InteractionParameter {
                i: 9,
                j: 41,
                a_ij: -18.8000,
            }),
            41009 => Ok(InteractionParameter {
                i: 41,
                j: 9,
                a_ij: 123.4000,
            }),
            9042 => Ok(InteractionParameter {
                i: 9,
                j: 42,
                a_ij: -588.9000,
            }),
            42009 => Ok(InteractionParameter {
                i: 42,
                j: 9,
                a_ij: 992.4000,
            }),
            9047 => Ok(InteractionParameter {
                i: 9,
                j: 47,
                a_ij: 37.5400,
            }),
            47009 => Ok(InteractionParameter {
                i: 47,
                j: 9,
                a_ij: 156.4000,
            }),
            9050 => Ok(InteractionParameter {
                i: 9,
                j: 50,
                a_ij: -162.6000,
            }),
            50009 => Ok(InteractionParameter {
                i: 50,
                j: 9,
                a_ij: 278.8000,
            }),
            10011 => Ok(InteractionParameter {
                i: 10,
                j: 11,
                a_ij: -110.3000,
            }),
            11010 => Ok(InteractionParameter {
                i: 11,
                j: 10,
                a_ij: 185.1000,
            }),
            10012 => Ok(InteractionParameter {
                i: 10,
                j: 12,
                a_ij: 766.0000,
            }),
            12010 => Ok(InteractionParameter {
                i: 12,
                j: 10,
                a_ij: -236.5000,
            }),
            10013 => Ok(InteractionParameter {
                i: 10,
                j: 13,
                a_ij: 304.1000,
            }),
            13010 => Ok(InteractionParameter {
                i: 13,
                j: 10,
                a_ij: -7.8380,
            }),
            10019 => Ok(InteractionParameter {
                i: 10,
                j: 19,
                a_ij: -106.4000,
            }),
            19010 => Ok(InteractionParameter {
                i: 19,
                j: 10,
                a_ij: 224.6600,
            }),
            10020 => Ok(InteractionParameter {
                i: 10,
                j: 20,
                a_ij: 497.5000,
            }),
            20010 => Ok(InteractionParameter {
                i: 20,
                j: 10,
                a_ij: -165.5000,
            }),
            10021 => Ok(InteractionParameter {
                i: 10,
                j: 21,
                a_ij: 751.9000,
            }),
            21010 => Ok(InteractionParameter {
                i: 21,
                j: 10,
                a_ij: -47.5100,
            }),
            10022 => Ok(InteractionParameter {
                i: 10,
                j: 22,
                a_ij: 67.5200,
            }),
            22010 => Ok(InteractionParameter {
                i: 22,
                j: 10,
                a_ij: 190.6000,
            }),
            10023 => Ok(InteractionParameter {
                i: 10,
                j: 23,
                a_ij: -483.7000,
            }),
            23010 => Ok(InteractionParameter {
                i: 23,
                j: 10,
                a_ij: 242.8000,
            }),
            10032 => Ok(InteractionParameter {
                i: 10,
                j: 32,
                a_ij: 117.0000,
            }),
            32010 => Ok(InteractionParameter {
                i: 32,
                j: 10,
                a_ij: 245.9000,
            }),
            10034 => Ok(InteractionParameter {
                i: 10,
                j: 34,
                a_ij: 2.2100,
            }),
            34010 => Ok(InteractionParameter {
                i: 34,
                j: 10,
                a_ij: -55.8700,
            }),
            10036 => Ok(InteractionParameter {
                i: 10,
                j: 36,
                a_ij: -339.2000,
            }),
            36010 => Ok(InteractionParameter {
                i: 36,
                j: 10,
                a_ij: 354.0000,
            }),
            10037 => Ok(InteractionParameter {
                i: 10,
                j: 37,
                a_ij: 172.4000,
            }),
            37010 => Ok(InteractionParameter {
                i: 37,
                j: 10,
                a_ij: 183.8000,
            }),
            10039 => Ok(InteractionParameter {
                i: 10,
                j: 39,
                a_ij: -268.8000,
            }),
            39010 => Ok(InteractionParameter {
                i: 39,
                j: 10,
                a_ij: 13.8900,
            }),
            10041 => Ok(InteractionParameter {
                i: 10,
                j: 41,
                a_ij: -275.5000,
            }),
            41010 => Ok(InteractionParameter {
                i: 41,
                j: 10,
                a_ij: 577.5000,
            }),
            11012 => Ok(InteractionParameter {
                i: 11,
                j: 12,
                a_ij: -241.8000,
            }),
            12011 => Ok(InteractionParameter {
                i: 12,
                j: 11,
                a_ij: 1167.0000,
            }),
            11013 => Ok(InteractionParameter {
                i: 11,
                j: 13,
                a_ij: -235.7000,
            }),
            13011 => Ok(InteractionParameter {
                i: 13,
                j: 11,
                a_ij: 461.3000,
            }),
            11015 => Ok(InteractionParameter {
                i: 11,
                j: 15,
                a_ij: -73.5000,
            }),
            15011 => Ok(InteractionParameter {
                i: 15,
                j: 11,
                a_ij: 136.0000,
            }),
            11016 => Ok(InteractionParameter {
                i: 11,
                j: 16,
                a_ij: -196.7000,
            }),
            16011 => Ok(InteractionParameter {
                i: 16,
                j: 11,
                a_ij: 2888.6001,
            }),
            11017 => Ok(InteractionParameter {
                i: 11,
                j: 17,
                a_ij: 475.5000,
            }),
            17011 => Ok(InteractionParameter {
                i: 17,
                j: 11,
                a_ij: -294.8000,
            }),
            11018 => Ok(InteractionParameter {
                i: 11,
                j: 18,
                a_ij: -0.1300,
            }),
            18011 => Ok(InteractionParameter {
                i: 18,
                j: 11,
                a_ij: 8.8700,
            }),
            11019 => Ok(InteractionParameter {
                i: 11,
                j: 19,
                a_ij: 494.6000,
            }),
            19011 => Ok(InteractionParameter {
                i: 19,
                j: 11,
                a_ij: -266.6000,
            }),
            11020 => Ok(InteractionParameter {
                i: 11,
                j: 20,
                a_ij: 660.2000,
            }),
            20011 => Ok(InteractionParameter {
                i: 20,
                j: 11,
                a_ij: -256.3000,
            }),
            11021 => Ok(InteractionParameter {
                i: 11,
                j: 21,
                a_ij: -34.7400,
            }),
            21011 => Ok(InteractionParameter {
                i: 21,
                j: 11,
                a_ij: 35.3800,
            }),
            11022 => Ok(InteractionParameter {
                i: 11,
                j: 22,
                a_ij: 108.8500,
            }),
            22011 => Ok(InteractionParameter {
                i: 22,
                j: 11,
                a_ij: -132.9500,
            }),
            11023 => Ok(InteractionParameter {
                i: 11,
                j: 23,
                a_ij: -209.6600,
            }),
            23011 => Ok(InteractionParameter {
                i: 23,
                j: 11,
                a_ij: 176.4500,
            }),
            11024 => Ok(InteractionParameter {
                i: 11,
                j: 24,
                a_ij: 54.5700,
            }),
            24011 => Ok(InteractionParameter {
                i: 24,
                j: 11,
                a_ij: 129.4900,
            }),
            11025 => Ok(InteractionParameter {
                i: 11,
                j: 25,
                a_ij: 442.4000,
            }),
            25011 => Ok(InteractionParameter {
                i: 25,
                j: 11,
                a_ij: -171.1000,
            }),
            11026 => Ok(InteractionParameter {
                i: 11,
                j: 26,
                a_ij: -81.1300,
            }),
            26011 => Ok(InteractionParameter {
                i: 26,
                j: 11,
                a_ij: 129.3000,
            }),
            11028 => Ok(InteractionParameter {
                i: 11,
                j: 28,
                a_ij: 183.0460,
            }),
            28011 => Ok(InteractionParameter {
                i: 28,
                j: 11,
                a_ij: 243.7750,
            }),
            11030 => Ok(InteractionParameter {
                i: 11,
                j: 30,
                a_ij: 202.2500,
            }),
            30011 => Ok(InteractionParameter {
                i: 30,
                j: 11,
                a_ij: -146.3100,
            }),
            11031 => Ok(InteractionParameter {
                i: 11,
                j: 31,
                a_ij: -101.7000,
            }),
            31011 => Ok(InteractionParameter {
                i: 31,
                j: 11,
                a_ij: 152.0000,
            }),
            11032 => Ok(InteractionParameter {
                i: 11,
                j: 32,
                a_ij: 148.3000,
            }),
            32011 => Ok(InteractionParameter {
                i: 32,
                j: 11,
                a_ij: 21.9200,
            }),
            11033 => Ok(InteractionParameter {
                i: 11,
                j: 33,
                a_ij: 18.8800,
            }),
            33011 => Ok(InteractionParameter {
                i: 33,
                j: 11,
                a_ij: 24.3700,
            }),
            11034 => Ok(InteractionParameter {
                i: 11,
                j: 34,
                a_ij: 71.4800,
            }),
            34011 => Ok(InteractionParameter {
                i: 34,
                j: 11,
                a_ij: -111.4500,
            }),
            11035 => Ok(InteractionParameter {
                i: 11,
                j: 35,
                a_ij: 52.0800,
            }),
            35011 => Ok(InteractionParameter {
                i: 35,
                j: 11,
                a_ij: 41.5700,
            }),
            11036 => Ok(InteractionParameter {
                i: 11,
                j: 36,
                a_ij: -28.6100,
            }),
            36011 => Ok(InteractionParameter {
                i: 36,
                j: 11,
                a_ij: 175.5300,
            }),
            11037 => Ok(InteractionParameter {
                i: 11,
                j: 37,
                a_ij: -275.2000,
            }),
            37011 => Ok(InteractionParameter {
                i: 37,
                j: 11,
                a_ij: 611.3000,
            }),
            11039 => Ok(InteractionParameter {
                i: 11,
                j: 39,
                a_ij: 85.3300,
            }),
            39011 => Ok(InteractionParameter {
                i: 39,
                j: 11,
                a_ij: -82.1200,
            }),
            11041 => Ok(InteractionParameter {
                i: 11,
                j: 41,
                a_ij: 560.2000,
            }),
            41011 => Ok(InteractionParameter {
                i: 41,
                j: 11,
                a_ij: -234.9000,
            }),
            11047 => Ok(InteractionParameter {
                i: 11,
                j: 47,
                a_ij: 151.8000,
            }),
            47011 => Ok(InteractionParameter {
                i: 47,
                j: 11,
                a_ij: -3.4440,
            }),
            12013 => Ok(InteractionParameter {
                i: 12,
                j: 13,
                a_ij: -234.0000,
            }),
            13012 => Ok(InteractionParameter {
                i: 13,
                j: 12,
                a_ij: 457.3000,
            }),
            12018 => Ok(InteractionParameter {
                i: 12,
                j: 18,
                a_ij: -233.4000,
            }),
            18012 => Ok(InteractionParameter {
                i: 18,
                j: 12,
                a_ij: 554.4000,
            }),
            12019 => Ok(InteractionParameter {
                i: 12,
                j: 19,
                a_ij: -47.2500,
            }),
            19012 => Ok(InteractionParameter {
                i: 19,
                j: 12,
                a_ij: 99.3700,
            }),
            12020 => Ok(InteractionParameter {
                i: 12,
                j: 20,
                a_ij: -268.1000,
            }),
            20012 => Ok(InteractionParameter {
                i: 20,
                j: 12,
                a_ij: 193.9000,
            }),
            12022 => Ok(InteractionParameter {
                i: 12,
                j: 22,
                a_ij: 31.0000,
            }),
            22012 => Ok(InteractionParameter {
                i: 22,
                j: 12,
                a_ij: 80.9900,
            }),
            12023 => Ok(InteractionParameter {
                i: 12,
                j: 23,
                a_ij: -126.2000,
            }),
            23012 => Ok(InteractionParameter {
                i: 23,
                j: 12,
                a_ij: 235.6000,
            }),
            12024 => Ok(InteractionParameter {
                i: 12,
                j: 24,
                a_ij: 179.7000,
            }),
            24012 => Ok(InteractionParameter {
                i: 24,
                j: 12,
                a_ij: 351.9000,
            }),
            12025 => Ok(InteractionParameter {
                i: 12,
                j: 25,
                a_ij: 24.2800,
            }),
            25012 => Ok(InteractionParameter {
                i: 25,
                j: 12,
                a_ij: 383.3000,
            }),
            12029 => Ok(InteractionParameter {
                i: 12,
                j: 29,
                a_ij: 103.9000,
            }),
            29012 => Ok(InteractionParameter {
                i: 29,
                j: 12,
                a_ij: 201.5000,
            }),
            12033 => Ok(InteractionParameter {
                i: 12,
                j: 33,
                a_ij: 298.1300,
            }),
            33012 => Ok(InteractionParameter {
                i: 33,
                j: 12,
                a_ij: -92.2600,
            }),
            12037 => Ok(InteractionParameter {
                i: 12,
                j: 37,
                a_ij: -11.4000,
            }),
            37012 => Ok(InteractionParameter {
                i: 37,
                j: 12,
                a_ij: 134.5000,
            }),
            12039 => Ok(InteractionParameter {
                i: 12,
                j: 39,
                a_ij: 308.9000,
            }),
            39012 => Ok(InteractionParameter {
                i: 39,
                j: 12,
                a_ij: -116.7000,
            }),
            12041 => Ok(InteractionParameter {
                i: 12,
                j: 41,
                a_ij: -122.3000,
            }),
            41012 => Ok(InteractionParameter {
                i: 41,
                j: 12,
                a_ij: 145.4000,
            }),
            13014 => Ok(InteractionParameter {
                i: 13,
                j: 14,
                a_ij: -78.3600,
            }),
            14013 => Ok(InteractionParameter {
                i: 14,
                j: 13,
                a_ij: 222.1000,
            }),
            13015 => Ok(InteractionParameter {
                i: 13,
                j: 15,
                a_ij: 251.5000,
            }),
            15013 => Ok(InteractionParameter {
                i: 15,
                j: 13,
                a_ij: -56.0800,
            }),
            13016 => Ok(InteractionParameter {
                i: 13,
                j: 16,
                a_ij: 5422.2998,
            }),
            16013 => Ok(InteractionParameter {
                i: 16,
                j: 13,
                a_ij: -194.1000,
            }),
            13017 => Ok(InteractionParameter {
                i: 13,
                j: 17,
                a_ij: -46.3900,
            }),
            17013 => Ok(InteractionParameter {
                i: 17,
                j: 13,
                a_ij: 285.3600,
            }),
            13018 => Ok(InteractionParameter {
                i: 13,
                j: 18,
                a_ij: 213.2000,
            }),
            18013 => Ok(InteractionParameter {
                i: 18,
                j: 13,
                a_ij: -156.1000,
            }),
            13019 => Ok(InteractionParameter {
                i: 13,
                j: 19,
                a_ij: -18.5100,
            }),
            19013 => Ok(InteractionParameter {
                i: 19,
                j: 13,
                a_ij: 38.8100,
            }),
            13020 => Ok(InteractionParameter {
                i: 13,
                j: 20,
                a_ij: 664.6000,
            }),
            20013 => Ok(InteractionParameter {
                i: 20,
                j: 13,
                a_ij: -338.5000,
            }),
            13021 => Ok(InteractionParameter {
                i: 13,
                j: 21,
                a_ij: 301.1400,
            }),
            21013 => Ok(InteractionParameter {
                i: 21,
                j: 13,
                a_ij: 225.3900,
            }),
            13022 => Ok(InteractionParameter {
                i: 13,
                j: 22,
                a_ij: 137.7700,
            }),
            22013 => Ok(InteractionParameter {
                i: 22,
                j: 13,
                a_ij: -197.7100,
            }),
            13023 => Ok(InteractionParameter {
                i: 13,
                j: 23,
                a_ij: -154.3000,
            }),
            23013 => Ok(InteractionParameter {
                i: 23,
                j: 13,
                a_ij: -20.9300,
            }),
            13024 => Ok(InteractionParameter {
                i: 13,
                j: 24,
                a_ij: 47.6700,
            }),
            24013 => Ok(InteractionParameter {
                i: 24,
                j: 13,
                a_ij: 113.9000,
            }),
            13025 => Ok(InteractionParameter {
                i: 13,
                j: 25,
                a_ij: 134.8000,
            }),
            25013 => Ok(InteractionParameter {
                i: 25,
                j: 13,
                a_ij: -25.1500,
            }),
            13026 => Ok(InteractionParameter {
                i: 13,
                j: 26,
                a_ij: 95.1800,
            }),
            26013 => Ok(InteractionParameter {
                i: 26,
                j: 13,
                a_ij: -94.4900,
            }),
            13027 => Ok(InteractionParameter {
                i: 13,
                j: 27,
                a_ij: 155.1100,
            }),
            27013 => Ok(InteractionParameter {
                i: 27,
                j: 13,
                a_ij: 220.6600,
            }),
            13028 => Ok(InteractionParameter {
                i: 13,
                j: 28,
                a_ij: 140.8960,
            }),
            28013 => Ok(InteractionParameter {
                i: 28,
                j: 13,
                a_ij: 112.3820,
            }),
            13029 => Ok(InteractionParameter {
                i: 13,
                j: 29,
                a_ij: -8.5380,
            }),
            29013 => Ok(InteractionParameter {
                i: 29,
                j: 13,
                a_ij: 63.7100,
            }),
            13030 => Ok(InteractionParameter {
                i: 13,
                j: 30,
                a_ij: 170.1000,
            }),
            30013 => Ok(InteractionParameter {
                i: 30,
                j: 13,
                a_ij: -87.3100,
            }),
            13031 => Ok(InteractionParameter {
                i: 13,
                j: 31,
                a_ij: -20.1100,
            }),
            31013 => Ok(InteractionParameter {
                i: 31,
                j: 13,
                a_ij: 9.2070,
            }),
            13032 => Ok(InteractionParameter {
                i: 13,
                j: 32,
                a_ij: -149.5000,
            }),
            32013 => Ok(InteractionParameter {
                i: 32,
                j: 13,
                a_ij: 476.6000,
            }),
            13033 => Ok(InteractionParameter {
                i: 13,
                j: 33,
                a_ij: -202.3000,
            }),
            33013 => Ok(InteractionParameter {
                i: 33,
                j: 13,
                a_ij: 736.4000,
            }),
            13034 => Ok(InteractionParameter {
                i: 13,
                j: 34,
                a_ij: -156.5700,
            }),
            34013 => Ok(InteractionParameter {
                i: 34,
                j: 13,
                a_ij: 173.7700,
            }),
            13035 => Ok(InteractionParameter {
                i: 13,
                j: 35,
                a_ij: 128.8000,
            }),
            35013 => Ok(InteractionParameter {
                i: 35,
                j: 13,
                a_ij: -93.5100,
            }),
            13037 => Ok(InteractionParameter {
                i: 13,
                j: 37,
                a_ij: 240.2000,
            }),
            37013 => Ok(InteractionParameter {
                i: 37,
                j: 13,
                a_ij: -217.9000,
            }),
            13038 => Ok(InteractionParameter {
                i: 13,
                j: 38,
                a_ij: -273.9500,
            }),
            38013 => Ok(InteractionParameter {
                i: 38,
                j: 13,
                a_ij: 167.3000,
            }),
            13039 => Ok(InteractionParameter {
                i: 13,
                j: 39,
                a_ij: 254.8000,
            }),
            39013 => Ok(InteractionParameter {
                i: 39,
                j: 13,
                a_ij: -158.2000,
            }),
            13040 => Ok(InteractionParameter {
                i: 13,
                j: 40,
                a_ij: -172.5100,
            }),
            40013 => Ok(InteractionParameter {
                i: 40,
                j: 13,
                a_ij: 278.1500,
            }),
            13041 => Ok(InteractionParameter {
                i: 13,
                j: 41,
                a_ij: 417.0000,
            }),
            41013 => Ok(InteractionParameter {
                i: 41,
                j: 13,
                a_ij: -247.8000,
            }),
            13042 => Ok(InteractionParameter {
                i: 13,
                j: 42,
                a_ij: 1338.0000,
            }),
            42013 => Ok(InteractionParameter {
                i: 42,
                j: 13,
                a_ij: 448.5000,
            }),
            14015 => Ok(InteractionParameter {
                i: 14,
                j: 15,
                a_ij: -107.2000,
            }),
            15014 => Ok(InteractionParameter {
                i: 15,
                j: 14,
                a_ij: 127.4000,
            }),
            14016 => Ok(InteractionParameter {
                i: 14,
                j: 16,
                a_ij: -41.1100,
            }),
            16014 => Ok(InteractionParameter {
                i: 16,
                j: 14,
                a_ij: 38.8900,
            }),
            14017 => Ok(InteractionParameter {
                i: 14,
                j: 17,
                a_ij: -200.7000,
            }),
            17014 => Ok(InteractionParameter {
                i: 17,
                j: 14,
                a_ij: -15.0700,
            }),
            14019 => Ok(InteractionParameter {
                i: 14,
                j: 19,
                a_ij: 358.9000,
            }),
            19014 => Ok(InteractionParameter {
                i: 19,
                j: 14,
                a_ij: -157.3000,
            }),
            14021 => Ok(InteractionParameter {
                i: 14,
                j: 21,
                a_ij: -82.9200,
            }),
            21014 => Ok(InteractionParameter {
                i: 21,
                j: 14,
                a_ij: 131.2000,
            }),
            14024 => Ok(InteractionParameter {
                i: 14,
                j: 24,
                a_ij: -99.8100,
            }),
            24014 => Ok(InteractionParameter {
                i: 24,
                j: 14,
                a_ij: 261.1000,
            }),
            14025 => Ok(InteractionParameter {
                i: 14,
                j: 25,
                a_ij: 30.0500,
            }),
            25014 => Ok(InteractionParameter {
                i: 25,
                j: 14,
                a_ij: 108.5000,
            }),
            14029 => Ok(InteractionParameter {
                i: 14,
                j: 29,
                a_ij: -70.1400,
            }),
            29014 => Ok(InteractionParameter {
                i: 29,
                j: 14,
                a_ij: 106.7000,
            }),
            14035 => Ok(InteractionParameter {
                i: 14,
                j: 35,
                a_ij: 874.1900,
            }),
            35014 => Ok(InteractionParameter {
                i: 35,
                j: 14,
                a_ij: -366.5100,
            }),
            14039 => Ok(InteractionParameter {
                i: 14,
                j: 39,
                a_ij: -164.0000,
            }),
            39014 => Ok(InteractionParameter {
                i: 39,
                j: 14,
                a_ij: 49.7000,
            }),
            14042 => Ok(InteractionParameter {
                i: 14,
                j: 42,
                a_ij: -664.4000,
            }),
            42014 => Ok(InteractionParameter {
                i: 42,
                j: 14,
                a_ij: 961.8000,
            }),
            14043 => Ok(InteractionParameter {
                i: 14,
                j: 43,
                a_ij: 275.9000,
            }),
            43014 => Ok(InteractionParameter {
                i: 43,
                j: 14,
                a_ij: -125.2000,
            }),
            15016 => Ok(InteractionParameter {
                i: 15,
                j: 16,
                a_ij: -189.2000,
            }),
            16015 => Ok(InteractionParameter {
                i: 16,
                j: 15,
                a_ij: 865.9000,
            }),
            15017 => Ok(InteractionParameter {
                i: 15,
                j: 17,
                a_ij: 138.5400,
            }),
            17015 => Ok(InteractionParameter {
                i: 17,
                j: 15,
                a_ij: 64.3000,
            }),
            15018 => Ok(InteractionParameter {
                i: 15,
                j: 18,
                a_ij: 431.4900,
            }),
            18015 => Ok(InteractionParameter {
                i: 18,
                j: 15,
                a_ij: -207.6600,
            }),
            15019 => Ok(InteractionParameter {
                i: 15,
                j: 19,
                a_ij: 147.1000,
            }),
            19015 => Ok(InteractionParameter {
                i: 19,
                j: 15,
                a_ij: -108.5000,
            }),
            15024 => Ok(InteractionParameter {
                i: 15,
                j: 24,
                a_ij: 71.2300,
            }),
            24015 => Ok(InteractionParameter {
                i: 24,
                j: 15,
                a_ij: 91.1300,
            }),
            15025 => Ok(InteractionParameter {
                i: 15,
                j: 25,
                a_ij: -18.9300,
            }),
            25015 => Ok(InteractionParameter {
                i: 25,
                j: 15,
                a_ij: 102.2000,
            }),
            15031 => Ok(InteractionParameter {
                i: 15,
                j: 31,
                a_ij: 939.0700,
            }),
            31015 => Ok(InteractionParameter {
                i: 31,
                j: 15,
                a_ij: -213.7400,
            }),
            15038 => Ok(InteractionParameter {
                i: 15,
                j: 38,
                a_ij: 570.9000,
            }),
            38015 => Ok(InteractionParameter {
                i: 38,
                j: 15,
                a_ij: -198.8000,
            }),
            15039 => Ok(InteractionParameter {
                i: 15,
                j: 39,
                a_ij: -255.2200,
            }),
            39015 => Ok(InteractionParameter {
                i: 39,
                j: 15,
                a_ij: 10.0300,
            }),
            15041 => Ok(InteractionParameter {
                i: 15,
                j: 41,
                a_ij: -38.7700,
            }),
            41015 => Ok(InteractionParameter {
                i: 41,
                j: 15,
                a_ij: 284.5000,
            }),
            15042 => Ok(InteractionParameter {
                i: 15,
                j: 42,
                a_ij: 448.1000,
            }),
            42015 => Ok(InteractionParameter {
                i: 42,
                j: 15,
                a_ij: 1464.2000,
            }),
            15043 => Ok(InteractionParameter {
                i: 15,
                j: 43,
                a_ij: -1327.0000,
            }),
            43015 => Ok(InteractionParameter {
                i: 43,
                j: 15,
                a_ij: 1603.8000,
            }),
            16017 => Ok(InteractionParameter {
                i: 16,
                j: 17,
                a_ij: 287.4300,
            }),
            17016 => Ok(InteractionParameter {
                i: 17,
                j: 16,
                a_ij: -24.4600,
            }),
            16019 => Ok(InteractionParameter {
                i: 16,
                j: 19,
                a_ij: 1255.1000,
            }),
            19016 => Ok(InteractionParameter {
                i: 19,
                j: 16,
                a_ij: -446.8600,
            }),
            16021 => Ok(InteractionParameter {
                i: 16,
                j: 21,
                a_ij: -182.9100,
            }),
            21016 => Ok(InteractionParameter {
                i: 21,
                j: 16,
                a_ij: 151.3800,
            }),
            16022 => Ok(InteractionParameter {
                i: 16,
                j: 22,
                a_ij: -73.8500,
            }),
            22016 => Ok(InteractionParameter {
                i: 22,
                j: 16,
                a_ij: -141.4000,
            }),
            16023 => Ok(InteractionParameter {
                i: 16,
                j: 23,
                a_ij: -352.9000,
            }),
            23016 => Ok(InteractionParameter {
                i: 23,
                j: 16,
                a_ij: -293.7000,
            }),
            16024 => Ok(InteractionParameter {
                i: 16,
                j: 24,
                a_ij: -262.0000,
            }),
            24016 => Ok(InteractionParameter {
                i: 24,
                j: 16,
                a_ij: 316.9000,
            }),
            16025 => Ok(InteractionParameter {
                i: 16,
                j: 25,
                a_ij: -181.9000,
            }),
            25016 => Ok(InteractionParameter {
                i: 25,
                j: 16,
                a_ij: 2951.0000,
            }),
            16035 => Ok(InteractionParameter {
                i: 16,
                j: 35,
                a_ij: 243.1000,
            }),
            35016 => Ok(InteractionParameter {
                i: 35,
                j: 16,
                a_ij: -257.2000,
            }),
            16038 => Ok(InteractionParameter {
                i: 16,
                j: 38,
                a_ij: -196.3120,
            }),
            38016 => Ok(InteractionParameter {
                i: 38,
                j: 16,
                a_ij: 116.4780,
            }),
            16039 => Ok(InteractionParameter {
                i: 16,
                j: 39,
                a_ij: 22.0500,
            }),
            39016 => Ok(InteractionParameter {
                i: 39,
                j: 16,
                a_ij: -185.2000,
            }),
            17018 => Ok(InteractionParameter {
                i: 17,
                j: 18,
                a_ij: 89.7000,
            }),
            18017 => Ok(InteractionParameter {
                i: 18,
                j: 17,
                a_ij: 117.4000,
            }),
            17019 => Ok(InteractionParameter {
                i: 17,
                j: 19,
                a_ij: -281.6000,
            }),
            19017 => Ok(InteractionParameter {
                i: 19,
                j: 17,
                a_ij: 777.4000,
            }),
            17020 => Ok(InteractionParameter {
                i: 17,
                j: 20,
                a_ij: -396.0000,
            }),
            20017 => Ok(InteractionParameter {
                i: 20,
                j: 17,
                a_ij: 493.8000,
            }),
            17021 => Ok(InteractionParameter {
                i: 17,
                j: 21,
                a_ij: 287.0000,
            }),
            21017 => Ok(InteractionParameter {
                i: 21,
                j: 17,
                a_ij: 429.7000,
            }),
            17022 => Ok(InteractionParameter {
                i: 17,
                j: 22,
                a_ij: -111.0000,
            }),
            22017 => Ok(InteractionParameter {
                i: 22,
                j: 17,
                a_ij: 140.8000,
            }),
            17024 => Ok(InteractionParameter {
                i: 17,
                j: 24,
                a_ij: 882.0000,
            }),
            24017 => Ok(InteractionParameter {
                i: 24,
                j: 17,
                a_ij: 898.2000,
            }),
            17025 => Ok(InteractionParameter {
                i: 17,
                j: 25,
                a_ij: 617.5000,
            }),
            25017 => Ok(InteractionParameter {
                i: 25,
                j: 17,
                a_ij: 334.9000,
            }),
            17027 => Ok(InteractionParameter {
                i: 17,
                j: 27,
                a_ij: -139.3000,
            }),
            27017 => Ok(InteractionParameter {
                i: 27,
                j: 17,
                a_ij: 134.9000,
            }),
            17031 => Ok(InteractionParameter {
                i: 17,
                j: 31,
                a_ij: 0.1004,
            }),
            31017 => Ok(InteractionParameter {
                i: 31,
                j: 17,
                a_ij: 192.3000,
            }),
            17039 => Ok(InteractionParameter {
                i: 17,
                j: 39,
                a_ij: -334.4000,
            }),
            39017 => Ok(InteractionParameter {
                i: 39,
                j: 17,
                a_ij: 343.7000,
            }),
            17041 => Ok(InteractionParameter {
                i: 17,
                j: 41,
                a_ij: -89.4200,
            }),
            41017 => Ok(InteractionParameter {
                i: 41,
                j: 17,
                a_ij: -22.1000,
            }),
            18019 => Ok(InteractionParameter {
                i: 18,
                j: 19,
                a_ij: -169.6700,
            }),
            19018 => Ok(InteractionParameter {
                i: 19,
                j: 18,
                a_ij: 134.2800,
            }),
            18020 => Ok(InteractionParameter {
                i: 18,
                j: 20,
                a_ij: -153.7000,
            }),
            20018 => Ok(InteractionParameter {
                i: 20,
                j: 18,
                a_ij: -313.5000,
            }),
            18022 => Ok(InteractionParameter {
                i: 18,
                j: 22,
                a_ij: -351.6000,
            }),
            22018 => Ok(InteractionParameter {
                i: 22,
                j: 18,
                a_ij: 587.3000,
            }),
            18023 => Ok(InteractionParameter {
                i: 18,
                j: 23,
                a_ij: -114.7300,
            }),
            23018 => Ok(InteractionParameter {
                i: 23,
                j: 18,
                a_ij: 18.9800,
            }),
            18024 => Ok(InteractionParameter {
                i: 18,
                j: 24,
                a_ij: -205.3000,
            }),
            24018 => Ok(InteractionParameter {
                i: 24,
                j: 18,
                a_ij: 368.5000,
            }),
            18025 => Ok(InteractionParameter {
                i: 18,
                j: 25,
                a_ij: -2.1700,
            }),
            25018 => Ok(InteractionParameter {
                i: 25,
                j: 18,
                a_ij: 20.1800,
            }),
            18027 => Ok(InteractionParameter {
                i: 18,
                j: 27,
                a_ij: 2845.0000,
            }),
            27018 => Ok(InteractionParameter {
                i: 27,
                j: 18,
                a_ij: 2475.0000,
            }),
            18033 => Ok(InteractionParameter {
                i: 18,
                j: 33,
                a_ij: -60.7800,
            }),
            33018 => Ok(InteractionParameter {
                i: 33,
                j: 18,
                a_ij: -42.7100,
            }),
            18037 => Ok(InteractionParameter {
                i: 18,
                j: 37,
                a_ij: 160.7000,
            }),
            37018 => Ok(InteractionParameter {
                i: 37,
                j: 18,
                a_ij: 281.6000,
            }),
            18038 => Ok(InteractionParameter {
                i: 18,
                j: 38,
                a_ij: -158.8000,
            }),
            38018 => Ok(InteractionParameter {
                i: 38,
                j: 18,
                a_ij: 159.8000,
            }),
            18050 => Ok(InteractionParameter {
                i: 18,
                j: 50,
                a_ij: -136.6000,
            }),
            50018 => Ok(InteractionParameter {
                i: 50,
                j: 18,
                a_ij: 221.4000,
            }),
            19020 => Ok(InteractionParameter {
                i: 19,
                j: 20,
                a_ij: 205.2700,
            }),
            20019 => Ok(InteractionParameter {
                i: 20,
                j: 19,
                a_ij: 92.0700,
            }),
            19021 => Ok(InteractionParameter {
                i: 19,
                j: 21,
                a_ij: 4.9330,
            }),
            21019 => Ok(InteractionParameter {
                i: 21,
                j: 19,
                a_ij: 54.3200,
            }),
            19022 => Ok(InteractionParameter {
                i: 19,
                j: 22,
                a_ij: -152.7000,
            }),
            22019 => Ok(InteractionParameter {
                i: 22,
                j: 19,
                a_ij: 258.6000,
            }),
            19023 => Ok(InteractionParameter {
                i: 19,
                j: 23,
                a_ij: -15.6200,
            }),
            23019 => Ok(InteractionParameter {
                i: 23,
                j: 19,
                a_ij: 74.0400,
            }),
            19024 => Ok(InteractionParameter {
                i: 19,
                j: 24,
                a_ij: -54.8600,
            }),
            24019 => Ok(InteractionParameter {
                i: 24,
                j: 19,
                a_ij: 491.9500,
            }),
            19025 => Ok(InteractionParameter {
                i: 19,
                j: 25,
                a_ij: -4.6240,
            }),
            25019 => Ok(InteractionParameter {
                i: 25,
                j: 19,
                a_ij: 363.5000,
            }),
            19026 => Ok(InteractionParameter {
                i: 19,
                j: 26,
                a_ij: -0.5150,
            }),
            26019 => Ok(InteractionParameter {
                i: 26,
                j: 19,
                a_ij: 0.2830,
            }),
            19028 => Ok(InteractionParameter {
                i: 19,
                j: 28,
                a_ij: 230.8520,
            }),
            28019 => Ok(InteractionParameter {
                i: 28,
                j: 19,
                a_ij: 335.7430,
            }),
            19029 => Ok(InteractionParameter {
                i: 19,
                j: 29,
                a_ij: 0.4604,
            }),
            29019 => Ok(InteractionParameter {
                i: 29,
                j: 19,
                a_ij: 161.0000,
            }),
            19031 => Ok(InteractionParameter {
                i: 19,
                j: 31,
                a_ij: 177.5000,
            }),
            31019 => Ok(InteractionParameter {
                i: 31,
                j: 19,
                a_ij: 169.6000,
            }),
            19033 => Ok(InteractionParameter {
                i: 19,
                j: 33,
                a_ij: -62.1700,
            }),
            33019 => Ok(InteractionParameter {
                i: 33,
                j: 19,
                a_ij: 136.9000,
            }),
            19034 => Ok(InteractionParameter {
                i: 19,
                j: 34,
                a_ij: -203.0200,
            }),
            34019 => Ok(InteractionParameter {
                i: 34,
                j: 19,
                a_ij: 329.1200,
            }),
            19036 => Ok(InteractionParameter {
                i: 19,
                j: 36,
                a_ij: 81.5700,
            }),
            36019 => Ok(InteractionParameter {
                i: 36,
                j: 19,
                a_ij: -42.3100,
            }),
            19037 => Ok(InteractionParameter {
                i: 19,
                j: 37,
                a_ij: -55.7700,
            }),
            37019 => Ok(InteractionParameter {
                i: 37,
                j: 19,
                a_ij: 335.2000,
            }),
            19039 => Ok(InteractionParameter {
                i: 19,
                j: 39,
                a_ij: -151.5000,
            }),
            39019 => Ok(InteractionParameter {
                i: 39,
                j: 19,
                a_ij: 150.6000,
            }),
            19041 => Ok(InteractionParameter {
                i: 19,
                j: 41,
                a_ij: 120.3000,
            }),
            41019 => Ok(InteractionParameter {
                i: 41,
                j: 19,
                a_ij: -61.6000,
            }),
            19047 => Ok(InteractionParameter {
                i: 19,
                j: 47,
                a_ij: 16.2300,
            }),
            47019 => Ok(InteractionParameter {
                i: 47,
                j: 19,
                a_ij: 119.2000,
            }),
            20021 => Ok(InteractionParameter {
                i: 20,
                j: 21,
                a_ij: 13.4100,
            }),
            21020 => Ok(InteractionParameter {
                i: 21,
                j: 20,
                a_ij: 519.1000,
            }),
            20022 => Ok(InteractionParameter {
                i: 20,
                j: 22,
                a_ij: -44.7000,
            }),
            22020 => Ok(InteractionParameter {
                i: 22,
                j: 20,
                a_ij: 543.3000,
            }),
            20023 => Ok(InteractionParameter {
                i: 20,
                j: 23,
                a_ij: 39.6300,
            }),
            23020 => Ok(InteractionParameter {
                i: 23,
                j: 20,
                a_ij: 504.2000,
            }),
            20024 => Ok(InteractionParameter {
                i: 20,
                j: 24,
                a_ij: 183.4000,
            }),
            24020 => Ok(InteractionParameter {
                i: 24,
                j: 20,
                a_ij: 631.0000,
            }),
            20025 => Ok(InteractionParameter {
                i: 20,
                j: 25,
                a_ij: -79.0800,
            }),
            25020 => Ok(InteractionParameter {
                i: 25,
                j: 20,
                a_ij: 993.4000,
            }),
            20030 => Ok(InteractionParameter {
                i: 20,
                j: 30,
                a_ij: -208.9000,
            }),
            30020 => Ok(InteractionParameter {
                i: 30,
                j: 20,
                a_ij: 570.6000,
            }),
            20032 => Ok(InteractionParameter {
                i: 20,
                j: 32,
                a_ij: 228.4000,
            }),
            32020 => Ok(InteractionParameter {
                i: 32,
                j: 20,
                a_ij: 616.6000,
            }),
            20033 => Ok(InteractionParameter {
                i: 20,
                j: 33,
                a_ij: -95.0000,
            }),
            33020 => Ok(InteractionParameter {
                i: 33,
                j: 20,
                a_ij: 5256.0000,
            }),
            20035 => Ok(InteractionParameter {
                i: 20,
                j: 35,
                a_ij: -463.6000,
            }),
            35020 => Ok(InteractionParameter {
                i: 35,
                j: 20,
                a_ij: -180.2000,
            }),
            20037 => Ok(InteractionParameter {
                i: 20,
                j: 37,
                a_ij: -11.1600,
            }),
            37020 => Ok(InteractionParameter {
                i: 37,
                j: 20,
                a_ij: 898.2000,
            }),
            20039 => Ok(InteractionParameter {
                i: 20,
                j: 39,
                a_ij: -228.0000,
            }),
            39020 => Ok(InteractionParameter {
                i: 39,
                j: 20,
                a_ij: -97.7700,
            }),
            20041 => Ok(InteractionParameter {
                i: 20,
                j: 41,
                a_ij: -337.0000,
            }),
            41020 => Ok(InteractionParameter {
                i: 41,
                j: 20,
                a_ij: 1179.0000,
            }),
            20046 => Ok(InteractionParameter {
                i: 20,
                j: 46,
                a_ij: -322.3000,
            }),
            46020 => Ok(InteractionParameter {
                i: 46,
                j: 20,
                a_ij: -70.2500,
            }),
            21022 => Ok(InteractionParameter {
                i: 21,
                j: 22,
                a_ij: 108.3100,
            }),
            22021 => Ok(InteractionParameter {
                i: 22,
                j: 21,
                a_ij: -84.5300,
            }),
            21023 => Ok(InteractionParameter {
                i: 21,
                j: 23,
                a_ij: 249.1500,
            }),
            23021 => Ok(InteractionParameter {
                i: 23,
                j: 21,
                a_ij: -157.1000,
            }),
            21024 => Ok(InteractionParameter {
                i: 21,
                j: 24,
                a_ij: 62.4200,
            }),
            24021 => Ok(InteractionParameter {
                i: 24,
                j: 21,
                a_ij: 11.8000,
            }),
            21025 => Ok(InteractionParameter {
                i: 21,
                j: 25,
                a_ij: 153.0000,
            }),
            25021 => Ok(InteractionParameter {
                i: 25,
                j: 21,
                a_ij: -129.7000,
            }),
            21026 => Ok(InteractionParameter {
                i: 21,
                j: 26,
                a_ij: 32.7300,
            }),
            26021 => Ok(InteractionParameter {
                i: 26,
                j: 21,
                a_ij: 113.0000,
            }),
            21027 => Ok(InteractionParameter {
                i: 21,
                j: 27,
                a_ij: 86.2000,
            }),
            27021 => Ok(InteractionParameter {
                i: 27,
                j: 21,
                a_ij: 1971.0000,
            }),
            21028 => Ok(InteractionParameter {
                i: 21,
                j: 28,
                a_ij: 450.0880,
            }),
            28021 => Ok(InteractionParameter {
                i: 28,
                j: 21,
                a_ij: -73.0920,
            }),
            21029 => Ok(InteractionParameter {
                i: 21,
                j: 29,
                a_ij: 59.0200,
            }),
            29021 => Ok(InteractionParameter {
                i: 29,
                j: 21,
                a_ij: -27.9400,
            }),
            21030 => Ok(InteractionParameter {
                i: 21,
                j: 30,
                a_ij: 65.5600,
            }),
            30021 => Ok(InteractionParameter {
                i: 30,
                j: 21,
                a_ij: -39.4600,
            }),
            21032 => Ok(InteractionParameter {
                i: 21,
                j: 32,
                a_ij: 2.2200,
            }),
            32021 => Ok(InteractionParameter {
                i: 32,
                j: 21,
                a_ij: 179.2500,
            }),
            21033 => Ok(InteractionParameter {
                i: 21,
                j: 33,
                a_ij: 344.4000,
            }),
            33021 => Ok(InteractionParameter {
                i: 33,
                j: 21,
                a_ij: -262.3000,
            }),
            21037 => Ok(InteractionParameter {
                i: 21,
                j: 37,
                a_ij: -168.2000,
            }),
            37021 => Ok(InteractionParameter {
                i: 37,
                j: 21,
                a_ij: 383.2000,
            }),
            21039 => Ok(InteractionParameter {
                i: 21,
                j: 39,
                a_ij: 6.5700,
            }),
            39021 => Ok(InteractionParameter {
                i: 39,
                j: 21,
                a_ij: -55.2100,
            }),
            21041 => Ok(InteractionParameter {
                i: 21,
                j: 41,
                a_ij: 63.6700,
            }),
            41021 => Ok(InteractionParameter {
                i: 41,
                j: 21,
                a_ij: 182.2000,
            }),
            22023 => Ok(InteractionParameter {
                i: 22,
                j: 23,
                a_ij: 0.0000,
            }),
            23022 => Ok(InteractionParameter {
                i: 23,
                j: 22,
                a_ij: 0.0000,
            }),
            22024 => Ok(InteractionParameter {
                i: 22,
                j: 24,
                a_ij: 56.3300,
            }),
            24022 => Ok(InteractionParameter {
                i: 24,
                j: 22,
                a_ij: 17.9700,
            }),
            22025 => Ok(InteractionParameter {
                i: 22,
                j: 25,
                a_ij: 223.1000,
            }),
            25022 => Ok(InteractionParameter {
                i: 25,
                j: 22,
                a_ij: -8.3090,
            }),
            22026 => Ok(InteractionParameter {
                i: 22,
                j: 26,
                a_ij: 108.9000,
            }),
            26022 => Ok(InteractionParameter {
                i: 26,
                j: 22,
                a_ij: -9.6390,
            }),
            22030 => Ok(InteractionParameter {
                i: 22,
                j: 30,
                a_ij: 149.5600,
            }),
            30022 => Ok(InteractionParameter {
                i: 30,
                j: 22,
                a_ij: -116.2100,
            }),
            22032 => Ok(InteractionParameter {
                i: 22,
                j: 32,
                a_ij: 177.6000,
            }),
            32022 => Ok(InteractionParameter {
                i: 32,
                j: 22,
                a_ij: -40.8200,
            }),
            22033 => Ok(InteractionParameter {
                i: 22,
                j: 33,
                a_ij: 315.9000,
            }),
            33022 => Ok(InteractionParameter {
                i: 33,
                j: 22,
                a_ij: -174.5000,
            }),
            22035 => Ok(InteractionParameter {
                i: 22,
                j: 35,
                a_ij: 215.0000,
            }),
            35022 => Ok(InteractionParameter {
                i: 35,
                j: 22,
                a_ij: -215.0000,
            }),
            22037 => Ok(InteractionParameter {
                i: 22,
                j: 37,
                a_ij: -91.8000,
            }),
            37022 => Ok(InteractionParameter {
                i: 37,
                j: 22,
                a_ij: 301.9000,
            }),
            22039 => Ok(InteractionParameter {
                i: 22,
                j: 39,
                a_ij: -160.2800,
            }),
            39022 => Ok(InteractionParameter {
                i: 39,
                j: 22,
                a_ij: 397.2400,
            }),
            22041 => Ok(InteractionParameter {
                i: 22,
                j: 41,
                a_ij: -96.8700,
            }),
            41022 => Ok(InteractionParameter {
                i: 41,
                j: 22,
                a_ij: 305.4000,
            }),
            22047 => Ok(InteractionParameter {
                i: 22,
                j: 47,
                a_ij: 361.1000,
            }),
            47022 => Ok(InteractionParameter {
                i: 47,
                j: 22,
                a_ij: -194.7000,
            }),
            23024 => Ok(InteractionParameter {
                i: 23,
                j: 24,
                a_ij: -30.1000,
            }),
            24023 => Ok(InteractionParameter {
                i: 24,
                j: 23,
                a_ij: 51.9000,
            }),
            23025 => Ok(InteractionParameter {
                i: 23,
                j: 25,
                a_ij: 192.1000,
            }),
            25023 => Ok(InteractionParameter {
                i: 25,
                j: 23,
                a_ij: -0.2266,
            }),
            23028 => Ok(InteractionParameter {
                i: 23,
                j: 28,
                a_ij: 116.6120,
            }),
            28023 => Ok(InteractionParameter {
                i: 28,
                j: 23,
                a_ij: -26.0580,
            }),
            23030 => Ok(InteractionParameter {
                i: 23,
                j: 30,
                a_ij: -64.3800,
            }),
            30023 => Ok(InteractionParameter {
                i: 30,
                j: 23,
                a_ij: 48.4840,
            }),
            23032 => Ok(InteractionParameter {
                i: 23,
                j: 32,
                a_ij: 86.4000,
            }),
            32023 => Ok(InteractionParameter {
                i: 32,
                j: 23,
                a_ij: 21.7600,
            }),
            23033 => Ok(InteractionParameter {
                i: 23,
                j: 33,
                a_ij: 168.8000,
            }),
            33023 => Ok(InteractionParameter {
                i: 33,
                j: 23,
                a_ij: -46.8000,
            }),
            23035 => Ok(InteractionParameter {
                i: 23,
                j: 35,
                a_ij: 363.7000,
            }),
            35023 => Ok(InteractionParameter {
                i: 35,
                j: 23,
                a_ij: -343.6000,
            }),
            23037 => Ok(InteractionParameter {
                i: 23,
                j: 37,
                a_ij: 111.2000,
            }),
            37023 => Ok(InteractionParameter {
                i: 37,
                j: 23,
                a_ij: -149.8000,
            }),
            23041 => Ok(InteractionParameter {
                i: 23,
                j: 41,
                a_ij: 255.8000,
            }),
            41023 => Ok(InteractionParameter {
                i: 41,
                j: 23,
                a_ij: -193.0000,
            }),
            23044 => Ok(InteractionParameter {
                i: 23,
                j: 44,
                a_ij: -35.6800,
            }),
            44023 => Ok(InteractionParameter {
                i: 44,
                j: 23,
                a_ij: -196.2000,
            }),
            23048 => Ok(InteractionParameter {
                i: 23,
                j: 48,
                a_ij: 565.9000,
            }),
            48023 => Ok(InteractionParameter {
                i: 48,
                j: 23,
                a_ij: -363.1000,
            }),
            24025 => Ok(InteractionParameter {
                i: 24,
                j: 25,
                a_ij: -75.9700,
            }),
            25024 => Ok(InteractionParameter {
                i: 25,
                j: 24,
                a_ij: 248.4000,
            }),
            24026 => Ok(InteractionParameter {
                i: 24,
                j: 26,
                a_ij: 490.8800,
            }),
            26024 => Ok(InteractionParameter {
                i: 26,
                j: 24,
                a_ij: -34.6800,
            }),
            24027 => Ok(InteractionParameter {
                i: 24,
                j: 27,
                a_ij: 534.7000,
            }),
            27024 => Ok(InteractionParameter {
                i: 27,
                j: 24,
                a_ij: 514.6000,
            }),
            24028 => Ok(InteractionParameter {
                i: 24,
                j: 28,
                a_ij: 132.2000,
            }),
            28024 => Ok(InteractionParameter {
                i: 28,
                j: 24,
                a_ij: -60.7100,
            }),
            24030 => Ok(InteractionParameter {
                i: 24,
                j: 30,
                a_ij: 546.6800,
            }),
            30024 => Ok(InteractionParameter {
                i: 30,
                j: 24,
                a_ij: -133.1600,
            }),
            24032 => Ok(InteractionParameter {
                i: 24,
                j: 32,
                a_ij: 247.8000,
            }),
            32024 => Ok(InteractionParameter {
                i: 32,
                j: 24,
                a_ij: 48.4900,
            }),
            24033 => Ok(InteractionParameter {
                i: 24,
                j: 33,
                a_ij: 146.6000,
            }),
            33024 => Ok(InteractionParameter {
                i: 33,
                j: 24,
                a_ij: 77.5500,
            }),
            24035 => Ok(InteractionParameter {
                i: 24,
                j: 35,
                a_ij: 337.7000,
            }),
            35024 => Ok(InteractionParameter {
                i: 35,
                j: 24,
                a_ij: -58.4300,
            }),
            24036 => Ok(InteractionParameter {
                i: 24,
                j: 36,
                a_ij: 369.4900,
            }),
            36024 => Ok(InteractionParameter {
                i: 36,
                j: 24,
                a_ij: -85.1480,
            }),
            24037 => Ok(InteractionParameter {
                i: 24,
                j: 37,
                a_ij: 187.1000,
            }),
            37024 => Ok(InteractionParameter {
                i: 37,
                j: 24,
                a_ij: -134.2000,
            }),
            24038 => Ok(InteractionParameter {
                i: 24,
                j: 38,
                a_ij: 215.2000,
            }),
            38024 => Ok(InteractionParameter {
                i: 38,
                j: 24,
                a_ij: -124.6000,
            }),
            24039 => Ok(InteractionParameter {
                i: 24,
                j: 39,
                a_ij: 498.6000,
            }),
            39024 => Ok(InteractionParameter {
                i: 39,
                j: 24,
                a_ij: -186.7000,
            }),
            24041 => Ok(InteractionParameter {
                i: 24,
                j: 41,
                a_ij: 256.5000,
            }),
            41024 => Ok(InteractionParameter {
                i: 41,
                j: 24,
                a_ij: 335.7000,
            }),
            24043 => Ok(InteractionParameter {
                i: 24,
                j: 43,
                a_ij: 233.1000,
            }),
            43024 => Ok(InteractionParameter {
                i: 43,
                j: 24,
                a_ij: 70.8100,
            }),
            24047 => Ok(InteractionParameter {
                i: 24,
                j: 47,
                a_ij: 423.1000,
            }),
            47024 => Ok(InteractionParameter {
                i: 47,
                j: 24,
                a_ij: 3.1630,
            }),
            24048 => Ok(InteractionParameter {
                i: 24,
                j: 48,
                a_ij: 63.9500,
            }),
            48024 => Ok(InteractionParameter {
                i: 48,
                j: 24,
                a_ij: -11.3000,
            }),
            24050 => Ok(InteractionParameter {
                i: 24,
                j: 50,
                a_ij: 108.5000,
            }),
            50024 => Ok(InteractionParameter {
                i: 50,
                j: 24,
                a_ij: -79.3400,
            }),
            24055 => Ok(InteractionParameter {
                i: 24,
                j: 55,
                a_ij: 585.1900,
            }),
            55024 => Ok(InteractionParameter {
                i: 55,
                j: 24,
                a_ij: 75.0400,
            }),
            25026 => Ok(InteractionParameter {
                i: 25,
                j: 26,
                a_ij: 132.7000,
            }),
            26025 => Ok(InteractionParameter {
                i: 26,
                j: 25,
                a_ij: 132.9000,
            }),
            25027 => Ok(InteractionParameter {
                i: 25,
                j: 27,
                a_ij: 2213.0000,
            }),
            27025 => Ok(InteractionParameter {
                i: 27,
                j: 25,
                a_ij: -123.1000,
            }),
            25033 => Ok(InteractionParameter {
                i: 25,
                j: 33,
                a_ij: 593.4000,
            }),
            33025 => Ok(InteractionParameter {
                i: 33,
                j: 25,
                a_ij: -185.3000,
            }),
            25035 => Ok(InteractionParameter {
                i: 25,
                j: 35,
                a_ij: 1337.3700,
            }),
            35025 => Ok(InteractionParameter {
                i: 35,
                j: 25,
                a_ij: -334.1200,
            }),
            25039 => Ok(InteractionParameter {
                i: 25,
                j: 39,
                a_ij: 5143.1401,
            }),
            39025 => Ok(InteractionParameter {
                i: 39,
                j: 25,
                a_ij: -374.1600,
            }),
            25040 => Ok(InteractionParameter {
                i: 25,
                j: 40,
                a_ij: 309.5800,
            }),
            40025 => Ok(InteractionParameter {
                i: 40,
                j: 25,
                a_ij: 33.9500,
            }),
            25041 => Ok(InteractionParameter {
                i: 25,
                j: 41,
                a_ij: -71.1800,
            }),
            41025 => Ok(InteractionParameter {
                i: 41,
                j: 25,
                a_ij: 956.1000,
            }),
            25044 => Ok(InteractionParameter {
                i: 25,
                j: 44,
                a_ij: -209.7000,
            }),
            44025 => Ok(InteractionParameter {
                i: 44,
                j: 25,
                a_ij: 161.5000,
            }),
            25047 => Ok(InteractionParameter {
                i: 25,
                j: 47,
                a_ij: 434.1000,
            }),
            47025 => Ok(InteractionParameter {
                i: 47,
                j: 25,
                a_ij: 7.0820,
            }),
            26027 => Ok(InteractionParameter {
                i: 26,
                j: 27,
                a_ij: 533.2000,
            }),
            27026 => Ok(InteractionParameter {
                i: 27,
                j: 26,
                a_ij: -85.1200,
            }),
            26028 => Ok(InteractionParameter {
                i: 26,
                j: 28,
                a_ij: 320.2000,
            }),
            28026 => Ok(InteractionParameter {
                i: 28,
                j: 26,
                a_ij: 277.8000,
            }),
            26031 => Ok(InteractionParameter {
                i: 26,
                j: 31,
                a_ij: 139.8220,
            }),
            31026 => Ok(InteractionParameter {
                i: 31,
                j: 26,
                a_ij: 481.3480,
            }),
            26032 => Ok(InteractionParameter {
                i: 26,
                j: 32,
                a_ij: 304.3000,
            }),
            32026 => Ok(InteractionParameter {
                i: 32,
                j: 26,
                a_ij: 64.2800,
            }),
            26033 => Ok(InteractionParameter {
                i: 26,
                j: 33,
                a_ij: 10.1700,
            }),
            33026 => Ok(InteractionParameter {
                i: 33,
                j: 26,
                a_ij: 125.3000,
            }),
            26034 => Ok(InteractionParameter {
                i: 26,
                j: 34,
                a_ij: -27.7010,
            }),
            34026 => Ok(InteractionParameter {
                i: 34,
                j: 26,
                a_ij: 174.4330,
            }),
            26037 => Ok(InteractionParameter {
                i: 26,
                j: 37,
                a_ij: 10.7600,
            }),
            37026 => Ok(InteractionParameter {
                i: 37,
                j: 26,
                a_ij: 379.4000,
            }),
            26039 => Ok(InteractionParameter {
                i: 26,
                j: 39,
                a_ij: -223.1000,
            }),
            39026 => Ok(InteractionParameter {
                i: 39,
                j: 26,
                a_ij: 223.6000,
            }),
            26041 => Ok(InteractionParameter {
                i: 26,
                j: 41,
                a_ij: 248.4000,
            }),
            41026 => Ok(InteractionParameter {
                i: 41,
                j: 26,
                a_ij: -124.7000,
            }),
            26045 => Ok(InteractionParameter {
                i: 26,
                j: 45,
                a_ij: -218.9000,
            }),
            45026 => Ok(InteractionParameter {
                i: 45,
                j: 26,
                a_ij: 844.0000,
            }),
            26050 => Ok(InteractionParameter {
                i: 26,
                j: 50,
                a_ij: -4.5650,
            }),
            50026 => Ok(InteractionParameter {
                i: 50,
                j: 26,
                a_ij: 176.3000,
            }),
            27032 => Ok(InteractionParameter {
                i: 27,
                j: 32,
                a_ij: 2990.0000,
            }),
            32027 => Ok(InteractionParameter {
                i: 32,
                j: 27,
                a_ij: 2448.0000,
            }),
            27033 => Ok(InteractionParameter {
                i: 27,
                j: 33,
                a_ij: -124.0000,
            }),
            33027 => Ok(InteractionParameter {
                i: 33,
                j: 27,
                a_ij: 4288.0000,
            }),
            28032 => Ok(InteractionParameter {
                i: 28,
                j: 32,
                a_ij: 292.7000,
            }),
            32028 => Ok(InteractionParameter {
                i: 32,
                j: 28,
                a_ij: -27.4500,
            }),
            28037 => Ok(InteractionParameter {
                i: 28,
                j: 37,
                a_ij: -47.3700,
            }),
            37028 => Ok(InteractionParameter {
                i: 37,
                j: 28,
                a_ij: 167.9000,
            }),
            28041 => Ok(InteractionParameter {
                i: 28,
                j: 41,
                a_ij: 469.8000,
            }),
            41028 => Ok(InteractionParameter {
                i: 41,
                j: 28,
                a_ij: 885.5000,
            }),
            29035 => Ok(InteractionParameter {
                i: 29,
                j: 35,
                a_ij: 31.6600,
            }),
            35029 => Ok(InteractionParameter {
                i: 35,
                j: 29,
                a_ij: 85.7000,
            }),
            29039 => Ok(InteractionParameter {
                i: 29,
                j: 39,
                a_ij: 78.9200,
            }),
            39029 => Ok(InteractionParameter {
                i: 39,
                j: 29,
                a_ij: -71.0000,
            }),
            29044 => Ok(InteractionParameter {
                i: 29,
                j: 44,
                a_ij: 1004.2000,
            }),
            44029 => Ok(InteractionParameter {
                i: 44,
                j: 29,
                a_ij: -274.1000,
            }),
            29048 => Ok(InteractionParameter {
                i: 29,
                j: 48,
                a_ij: -18.2700,
            }),
            48029 => Ok(InteractionParameter {
                i: 48,
                j: 29,
                a_ij: 6.9710,
            }),
            30041 => Ok(InteractionParameter {
                i: 30,
                j: 41,
                a_ij: 43.3700,
            }),
            41030 => Ok(InteractionParameter {
                i: 41,
                j: 30,
                a_ij: -64.2800,
            }),
            31035 => Ok(InteractionParameter {
                i: 31,
                j: 35,
                a_ij: -417.2000,
            }),
            35031 => Ok(InteractionParameter {
                i: 35,
                j: 31,
                a_ij: 535.8000,
            }),
            31039 => Ok(InteractionParameter {
                i: 31,
                j: 39,
                a_ij: 302.2000,
            }),
            39031 => Ok(InteractionParameter {
                i: 39,
                j: 31,
                a_ij: -191.7000,
            }),
            31041 => Ok(InteractionParameter {
                i: 31,
                j: 41,
                a_ij: 347.8000,
            }),
            41031 => Ok(InteractionParameter {
                i: 41,
                j: 31,
                a_ij: -264.3000,
            }),
            31044 => Ok(InteractionParameter {
                i: 31,
                j: 44,
                a_ij: -262.0000,
            }),
            44031 => Ok(InteractionParameter {
                i: 44,
                j: 31,
                a_ij: 262.0000,
            }),
            31047 => Ok(InteractionParameter {
                i: 31,
                j: 47,
                a_ij: -353.5000,
            }),
            47031 => Ok(InteractionParameter {
                i: 47,
                j: 31,
                a_ij: 515.8000,
            }),
            32033 => Ok(InteractionParameter {
                i: 32,
                j: 33,
                a_ij: 6.3700,
            }),
            33032 => Ok(InteractionParameter {
                i: 33,
                j: 32,
                a_ij: 37.1000,
            }),
            32041 => Ok(InteractionParameter {
                i: 32,
                j: 41,
                a_ij: 68.5500,
            }),
            41032 => Ok(InteractionParameter {
                i: 41,
                j: 32,
                a_ij: 288.1000,
            }),
            33035 => Ok(InteractionParameter {
                i: 33,
                j: 35,
                a_ij: 32.9000,
            }),
            35033 => Ok(InteractionParameter {
                i: 35,
                j: 33,
                a_ij: -111.2000,
            }),
            33037 => Ok(InteractionParameter {
                i: 33,
                j: 37,
                a_ij: -48.3300,
            }),
            37033 => Ok(InteractionParameter {
                i: 37,
                j: 33,
                a_ij: 322.4200,
            }),
            33039 => Ok(InteractionParameter {
                i: 33,
                j: 39,
                a_ij: 336.2500,
            }),
            39033 => Ok(InteractionParameter {
                i: 39,
                j: 33,
                a_ij: -176.2600,
            }),
            33041 => Ok(InteractionParameter {
                i: 33,
                j: 41,
                a_ij: -195.1000,
            }),
            41033 => Ok(InteractionParameter {
                i: 41,
                j: 33,
                a_ij: 627.7000,
            }),
            34037 => Ok(InteractionParameter {
                i: 34,
                j: 37,
                a_ij: 2073.2000,
            }),
            37034 => Ok(InteractionParameter {
                i: 37,
                j: 34,
                a_ij: 631.5000,
            }),
            34039 => Ok(InteractionParameter {
                i: 34,
                j: 39,
                a_ij: -119.8000,
            }),
            39034 => Ok(InteractionParameter {
                i: 39,
                j: 34,
                a_ij: 6.6990,
            }),
            35039 => Ok(InteractionParameter {
                i: 35,
                j: 39,
                a_ij: -97.7100,
            }),
            39035 => Ok(InteractionParameter {
                i: 39,
                j: 35,
                a_ij: 136.6000,
            }),
            35041 => Ok(InteractionParameter {
                i: 35,
                j: 41,
                a_ij: 153.7000,
            }),
            41035 => Ok(InteractionParameter {
                i: 41,
                j: 35,
                a_ij: -29.3400,
            }),
            36037 => Ok(InteractionParameter {
                i: 36,
                j: 37,
                a_ij: -208.8000,
            }),
            37036 => Ok(InteractionParameter {
                i: 37,
                j: 36,
                a_ij: 837.2000,
            }),
            36039 => Ok(InteractionParameter {
                i: 36,
                j: 39,
                a_ij: -8.8040,
            }),
            39036 => Ok(InteractionParameter {
                i: 39,
                j: 36,
                a_ij: 5.1500,
            }),
            36041 => Ok(InteractionParameter {
                i: 36,
                j: 41,
                a_ij: 423.4000,
            }),
            41036 => Ok(InteractionParameter {
                i: 41,
                j: 36,
                a_ij: -53.9100,
            }),
            37039 => Ok(InteractionParameter {
                i: 37,
                j: 39,
                a_ij: 255.0000,
            }),
            39037 => Ok(InteractionParameter {
                i: 39,
                j: 37,
                a_ij: -137.7000,
            }),
            37041 => Ok(InteractionParameter {
                i: 37,
                j: 41,
                a_ij: 730.8000,
            }),
            41037 => Ok(InteractionParameter {
                i: 41,
                j: 37,
                a_ij: -198.0000,
            }),
            37044 => Ok(InteractionParameter {
                i: 37,
                j: 44,
                a_ij: 26.3500,
            }),
            44037 => Ok(InteractionParameter {
                i: 44,
                j: 37,
                a_ij: -66.3100,
            }),
            37048 => Ok(InteractionParameter {
                i: 37,
                j: 48,
                a_ij: 2429.0000,
            }),
            48037 => Ok(InteractionParameter {
                i: 48,
                j: 37,
                a_ij: 148.9000,
            }),
            38039 => Ok(InteractionParameter {
                i: 38,
                j: 39,
                a_ij: -110.6500,
            }),
            39038 => Ok(InteractionParameter {
                i: 39,
                j: 38,
                a_ij: 50.0600,
            }),
            38040 => Ok(InteractionParameter {
                i: 38,
                j: 40,
                a_ij: -117.1700,
            }),
            40038 => Ok(InteractionParameter {
                i: 40,
                j: 38,
                a_ij: 185.6000,
            }),
            39040 => Ok(InteractionParameter {
                i: 39,
                j: 40,
                a_ij: -5.5790,
            }),
            40039 => Ok(InteractionParameter {
                i: 40,
                j: 39,
                a_ij: 55.8000,
            }),
            39041 => Ok(InteractionParameter {
                i: 39,
                j: 41,
                a_ij: 72.3100,
            }),
            41039 => Ok(InteractionParameter {
                i: 41,
                j: 39,
                a_ij: -28.6500,
            }),
            40045 => Ok(InteractionParameter {
                i: 40,
                j: 45,
                a_ij: 111.8000,
            }),
            45040 => Ok(InteractionParameter {
                i: 45,
                j: 40,
                a_ij: -32.1700,
            }),
            41047 => Ok(InteractionParameter {
                i: 41,
                j: 47,
                a_ij: 122.4000,
            }),
            47041 => Ok(InteractionParameter {
                i: 47,
                j: 41,
                a_ij: 101.2000,
            }),
            42043 => Ok(InteractionParameter {
                i: 42,
                j: 43,
                a_ij: -2166.0000,
            }),
            43042 => Ok(InteractionParameter {
                i: 43,
                j: 42,
                a_ij: 745.3000,
            }),
            84085 => Ok(InteractionParameter {
                i: 84,
                j: 85,
                a_ij: 1517.5000,
            }),
            85084 => Ok(InteractionParameter {
                i: 85,
                j: 84,
                a_ij: -1869.9000,
            }),
            _ => Err("Interaction parameter does not exist"),
        }
    }
}
