//! Выборка из около 20М текстовых материалов включает литературу и компьютерные технологии
//!
//! Японская таблица частот, применяемая как к S-JIS, так и к EUC-JP
//! Они отсортированы по порядку.
//!
//! Char в таблице FreqOrder,  от hz6763
//!
//! - 128  --> 0.77094
//! - 256  --> 0.85710
//! - 512  --> 0.92635
//! - 1024 --> 0.97130
//! - 2048 --> 0.99431
//!
//! Коэффициент идеального распределения (IDR) = 0.92635 / (1 - 0.92635) = 12.58
//!
//! Коэффициент случайного распределение (RDR) = 512 / (2965 + 62 + 83 + 86 - 512) = 0.191

/// Типичный коэффициент распределения, 25% от IDR
pub const JIS_TYPICAL_DISTRIBUTION_RATIO: f64 = 3.0;

pub const JIS_TABLE_SIZE: usize = 4368;

#[rustfmt::skip]
pub const JIS_CHAR_TO_FREQ_ORDER: [i32; JIS_TABLE_SIZE] = [
      40,    1,    6,  182,  152,  180,  295, 2127,  285,  381, 3295, 4304, 3068, 4606, 3165, 3510, //    16
    3511, 1822, 2785, 4607, 1193, 2226, 5070, 4608,  171, 2996, 1247,   18,  179, 5071,  856, 1661, //    32
    1262, 5072,  619,  127, 3431, 3512, 3230, 1899, 1700,  232,  228, 1294, 1298,  284,  283, 2041, //    48
    2042, 1061, 1062,   48,   49,   44,   45,  433,  434, 1040, 1041,  996,  787, 2997, 1255, 4305, //    64
    2108, 4609, 1684, 1648, 5073, 5074, 5075, 5076, 5077, 5078, 3687, 5079, 4610, 5080, 3927, 3928, //    80
    5081, 3296, 3432,  290, 2285, 1471, 2187, 5082, 2580, 2825, 1303, 2140, 1739, 1445, 2691, 3375, //    96
    1691, 3297, 4306, 4307, 4611,  452, 3376, 1182, 2713, 3688, 3069, 4308, 5083, 5084, 5085, 5086, //   112
    5087, 5088, 5089, 5090, 5091, 5092, 5093, 5094, 5095, 5096, 5097, 5098, 5099, 5100, 5101, 5102, //   128
    5103, 5104, 5105, 5106, 5107, 5108, 5109, 5110, 5111, 5112, 4097, 5113, 5114, 5115, 5116, 5117, //   144
    5118, 5119, 5120, 5121, 5122, 5123, 5124, 5125, 5126, 5127, 5128, 5129, 5130, 5131, 5132, 5133, //   160
    5134, 5135, 5136, 5137, 5138, 5139, 5140, 5141, 5142, 5143, 5144, 5145, 5146, 5147, 5148, 5149, //   176
    5150, 5151, 5152, 4612, 5153, 5154, 5155, 5156, 5157, 5158, 5159, 5160, 5161, 5162, 5163, 5164, //   192
    5165, 5166, 5167, 5168, 5169, 5170, 5171, 5172, 5173, 5174, 5175, 1472,  598,  618,  820, 1205, //   208
    1309, 1412, 1858, 1307, 1692, 5176, 5177, 5178, 5179, 5180, 5181, 5182, 1142, 1452, 1234, 1172, //   224
    1875, 2043, 2149, 1793, 1382, 2973,  925, 2404, 1067, 1241,  960, 1377, 2935, 1491,  919, 1217, //   240
    1865, 2030, 1406, 1499, 2749, 4098, 5183, 5184, 5185, 5186, 5187, 5188, 2561, 4099, 3117, 1804, //   256
    2049, 3689, 4309, 3513, 1663, 5189, 3166, 3118, 3298, 1587, 1561, 3433, 5190, 3119, 1625, 2998, //   272
    3299, 4613, 1766, 3690, 2786, 4614, 5191, 5192, 5193, 5194, 2161,   26, 3377,    2, 3929,   20, //   288
    3691,   47, 4100,   50,   17,   16,   35,  268,   27,  243,   42,  155,   24,  154,   29,  184, //   304
       4,   91,   14,   92,   53,  396,   33,  289,    9,   37,   64,  620,   21,   39,  321,    5, //   320
      12,   11,   52,   13,    3,  208,  138,    0,    7,   60,  526,  141,  151, 1069,  181,  275, //   336
    1591,   83,  132, 1475,  126,  331,  829,   15,   69,  160,   59,   22,  157,   55, 1079,  312, //   352
     109,   38,   23,   25,   10,   19,   79, 5195,   61,  382, 1124,    8,   30, 5196, 5197, 5198, //   368
    5199, 5200, 5201, 5202, 5203, 5204, 5205, 5206,   89,   62,   74,   34, 2416,  112,  139,  196, //   384
     271,  149,   84,  607,  131,  765,   46,   88,  153,  683,   76,  874,  101,  258,   57,   80, //   400
      32,  364,  121, 1508,  169, 1547,   68,  235,  145, 2999,   41,  360, 3027,   70,   63,   31, //   416
      43,  259,  262, 1383,   99,  533,  194,   66,   93,  846,  217,  192,   56,  106,   58,  565, //   432
     280,  272,  311,  256,  146,   82,  308,   71,  100,  128,  214,  655,  110,  261,  104, 1140, //   448
      54,   51,   36,   87,   67, 3070,  185, 2618, 2936, 2020,   28, 1066, 2390, 2059, 5207, 5208, //   464
    5209, 5210, 5211, 5212, 5213, 5214, 5215, 5216, 4615, 5217, 5218, 5219, 5220, 5221, 5222, 5223, //   480
    5224, 5225, 5226, 5227, 5228, 5229, 5230, 5231, 5232, 5233, 5234, 5235, 5236, 3514, 5237, 5238, //   496
    5239, 5240, 5241, 5242, 5243, 5244, 2297, 2031, 4616, 4310, 3692, 5245, 3071, 5246, 3598, 5247, //   512
    4617, 3231, 3515, 5248, 4101, 4311, 4618, 3808, 4312, 4102, 5249, 4103, 4104, 3599, 5250, 5251, //   528
    5252, 5253, 5254, 5255, 5256, 5257, 5258, 5259, 5260, 5261, 5262, 5263, 5264, 5265, 5266, 5267, //   544
    5268, 5269, 5270, 5271, 5272, 5273, 5274, 5275, 5276, 5277, 5278, 5279, 5280, 5281, 5282, 5283, //   560
    5284, 5285, 5286, 5287, 5288, 5289, 5290, 5291, 5292, 5293, 5294, 5295, 5296, 5297, 5298, 5299, //   576
    5300, 5301, 5302, 5303, 5304, 5305, 5306, 5307, 5308, 5309, 5310, 5311, 5312, 5313, 5314, 5315, //   592
    5316, 5317, 5318, 5319, 5320, 5321, 5322, 5323, 5324, 5325, 5326, 5327, 5328, 5329, 5330, 5331, //   608
    5332, 5333, 5334, 5335, 5336, 5337, 5338, 5339, 5340, 5341, 5342, 5343, 5344, 5345, 5346, 5347, //   624
    5348, 5349, 5350, 5351, 5352, 5353, 5354, 5355, 5356, 5357, 5358, 5359, 5360, 5361, 5362, 5363, //   640
    5364, 5365, 5366, 5367, 5368, 5369, 5370, 5371, 5372, 5373, 5374, 5375, 5376, 5377, 5378, 5379, //   656
    5380, 5381,  363,  642, 2787, 2878, 2788, 2789, 2316, 3232, 2317, 3434, 2011,  165, 1942, 3930, //   672
    3931, 3932, 3933, 5382, 4619, 5383, 4620, 5384, 5385, 5386, 5387, 5388, 5389, 5390, 5391, 5392, //   688
    5393, 5394, 5395, 5396, 5397, 5398, 5399, 5400, 5401, 5402, 5403, 5404, 5405, 5406, 5407, 5408, //   704
    5409, 5410, 5411, 5412, 5413, 5414, 5415, 5416, 5417, 5418, 5419, 5420, 5421, 5422, 5423, 5424, //   720
    5425, 5426, 5427, 5428, 5429, 5430, 5431, 5432, 5433, 5434, 5435, 5436, 5437, 5438, 5439, 5440, //   736
    5441, 5442, 5443, 5444, 5445, 5446, 5447, 5448, 5449, 5450, 5451, 5452, 5453, 5454, 5455, 5456, //   752
    5457, 5458, 5459, 5460, 5461, 5462, 5463, 5464, 5465, 5466, 5467, 5468, 5469, 5470, 5471, 5472, //   768
    5473, 5474, 5475, 5476, 5477, 5478, 5479, 5480, 5481, 5482, 5483, 5484, 5485, 5486, 5487, 5488, //   784
    5489, 5490, 5491, 5492, 5493, 5494, 5495, 5496, 5497, 5498, 5499, 5500, 5501, 5502, 5503, 5504, //   800
    5505, 5506, 5507, 5508, 5509, 5510, 5511, 5512, 5513, 5514, 5515, 5516, 5517, 5518, 5519, 5520, //   816
    5521, 5522, 5523, 5524, 5525, 5526, 5527, 5528, 5529, 5530, 5531, 5532, 5533, 5534, 5535, 5536, //   832
    5537, 5538, 5539, 5540, 5541, 5542, 5543, 5544, 5545, 5546, 5547, 5548, 5549, 5550, 5551, 5552, //   848
    5553, 5554, 5555, 5556, 5557, 5558, 5559, 5560, 5561, 5562, 5563, 5564, 5565, 5566, 5567, 5568, //   864
    5569, 5570, 5571, 5572, 5573, 5574, 5575, 5576, 5577, 5578, 5579, 5580, 5581, 5582, 5583, 5584, //   880
    5585, 5586, 5587, 5588, 5589, 5590, 5591, 5592, 5593, 5594, 5595, 5596, 5597, 5598, 5599, 5600, //   896
    5601, 5602, 5603, 5604, 5605, 5606, 5607, 5608, 5609, 5610, 5611, 5612, 5613, 5614, 5615, 5616, //   912
    5617, 5618, 5619, 5620, 5621, 5622, 5623, 5624, 5625, 5626, 5627, 5628, 5629, 5630, 5631, 5632, //   928
    5633, 5634, 5635, 5636, 5637, 5638, 5639, 5640, 5641, 5642, 5643, 5644, 5645, 5646, 5647, 5648, //   944
    5649, 5650, 5651, 5652, 5653, 5654, 5655, 5656, 5657, 5658, 5659, 5660, 5661, 5662, 5663, 5664, //   960
    5665, 5666, 5667, 5668, 5669, 5670, 5671, 5672, 5673, 5674, 5675, 5676, 5677, 5678, 5679, 5680, //   976
    5681, 5682, 5683, 5684, 5685, 5686, 5687, 5688, 5689, 5690, 5691, 5692, 5693, 5694, 5695, 5696, //   992
    5697, 5698, 5699, 5700, 5701, 5702, 5703, 5704, 5705, 5706, 5707, 5708, 5709, 5710, 5711, 5712, //  1008
    5713, 5714, 5715, 5716, 5717, 5718, 5719, 5720, 5721, 5722, 5723, 5724, 5725, 5726, 5727, 5728, //  1024
    5729, 5730, 5731, 5732, 5733, 5734, 5735, 5736, 5737, 5738, 5739, 5740, 5741, 5742, 5743, 5744, //  1040
    5745, 5746, 5747, 5748, 5749, 5750, 5751, 5752, 5753, 5754, 5755, 5756, 5757, 5758, 5759, 5760, //  1056
    5761, 5762, 5763, 5764, 5765, 5766, 5767, 5768, 5769, 5770, 5771, 5772, 5773, 5774, 5775, 5776, //  1072
    5777, 5778, 5779, 5780, 5781, 5782, 5783, 5784, 5785, 5786, 5787, 5788, 5789, 5790, 5791, 5792, //  1088
    5793, 5794, 5795, 5796, 5797, 5798, 5799, 5800, 5801, 5802, 5803, 5804, 5805, 5806, 5807, 5808, //  1104
    5809, 5810, 5811, 5812, 5813, 5814, 5815, 5816, 5817, 5818, 5819, 5820, 5821, 5822, 5823, 5824, //  1120
    5825, 5826, 5827, 5828, 5829, 5830, 5831, 5832, 5833, 5834, 5835, 5836, 5837, 5838, 5839, 5840, //  1136
    5841, 5842, 5843, 5844, 5845, 5846, 5847, 5848, 5849, 5850, 5851, 5852, 5853, 5854, 5855, 5856, //  1152
    5857, 5858, 5859, 5860, 5861, 5862, 5863, 5864, 5865, 5866, 5867, 5868, 5869, 5870, 5871, 5872, //  1168
    5873, 5874, 5875, 5876, 5877, 5878, 5879, 5880, 5881, 5882, 5883, 5884, 5885, 5886, 5887, 5888, //  1184
    5889, 5890, 5891, 5892, 5893, 5894, 5895, 5896, 5897, 5898, 5899, 5900, 5901, 5902, 5903, 5904, //  1200
    5905, 5906, 5907, 5908, 5909, 5910, 5911, 5912, 5913, 5914, 5915, 5916, 5917, 5918, 5919, 5920, //  1216
    5921, 5922, 5923, 5924, 5925, 5926, 5927, 5928, 5929, 5930, 5931, 5932, 5933, 5934, 5935, 5936, //  1232
    5937, 5938, 5939, 5940, 5941, 5942, 5943, 5944, 5945, 5946, 5947, 5948, 5949, 5950, 5951, 5952, //  1248
    5953, 5954, 5955, 5956, 5957, 5958, 5959, 5960, 5961, 5962, 5963, 5964, 5965, 5966, 5967, 5968, //  1264
    5969, 5970, 5971, 5972, 5973, 5974, 5975, 5976, 5977, 5978, 5979, 5980, 5981, 5982, 5983, 5984, //  1280
    5985, 5986, 5987, 5988, 5989, 5990, 5991, 5992, 5993, 5994, 5995, 5996, 5997, 5998, 5999, 6000, //  1296
    6001, 6002, 6003, 6004, 6005, 6006, 6007, 6008, 6009, 6010, 6011, 6012, 6013, 6014, 6015, 6016, //  1312
    6017, 6018, 6019, 6020, 6021, 6022, 6023, 6024, 6025, 6026, 6027, 6028, 6029, 6030, 6031, 6032, //  1328
    6033, 6034, 6035, 6036, 6037, 6038, 6039, 6040, 6041, 6042, 6043, 6044, 6045, 6046, 6047, 6048, //  1344
    6049, 6050, 6051, 6052, 6053, 6054, 6055, 6056, 6057, 6058, 6059, 6060, 6061, 6062, 6063, 6064, //  1360
    6065, 6066, 6067, 6068, 6069, 6070, 6071, 6072, 6073, 6074, 6075, 6076, 6077, 6078, 6079, 6080, //  1376
    6081, 6082, 6083, 6084, 6085, 6086, 6087, 6088, 6089, 6090, 6091, 6092, 6093, 6094, 6095, 6096, //  1392
    6097, 6098, 6099, 6100, 6101, 6102, 6103, 6104, 6105, 6106, 6107, 6108, 6109, 6110, 6111, 6112, //  1408
    6113, 6114, 2044, 2060, 4621,  997, 1235,  473, 1186, 4622,  920, 3378, 6115, 6116,  379, 1108, //  1424
    4313, 2657, 2735, 3934, 6117, 3809,  636, 3233,  573, 1026, 3693, 3435, 2974, 3300, 2298, 4105, //  1440
     854, 2937, 2463,  393, 2581, 2417,  539,  752, 1280, 2750, 2480,  140, 1161,  440,  708, 1569, //  1456
     665, 2497, 1746, 1291, 1523, 3000,  164, 1603,  847, 1331,  537, 1997,  486,  508, 1693, 2418, //  1472
    1970, 2227,  878, 1220,  299, 1030,  969,  652, 2751,  624, 1137, 3301, 2619,   65, 3302, 2045, //  1488
    1761, 1859, 3120, 1930, 3694, 3516,  663, 1767,  852,  835, 3695,  269,  767, 2826, 2339, 1305, //  1504
     896, 1150,  770, 1616, 6118,  506, 1502, 2075, 1012, 2519,  775, 2520, 2975, 2340, 2938, 4314, //  1520
    3028, 2086, 1224, 1943, 2286, 6119, 3072, 4315, 2240, 1273, 1987, 3935, 1557,  175,  597,  985, //  1536
    3517, 2419, 2521, 1416, 3029,  585,  938, 1931, 1007, 1052, 1932, 1685, 6120, 3379, 4316, 4623, //  1552
     804,  599, 3121, 1333, 2128, 2539, 1159, 1554, 2032, 3810,  687, 2033, 2904,  952,  675, 1467, //  1568
    3436, 6121, 2241, 1096, 1786, 2440, 1543, 1924,  980, 1813, 2228,  781, 2692, 1879,  728, 1918, //  1584
    3696, 4624,  548, 1950, 4625, 1809, 1088, 1356, 3303, 2522, 1944,  502,  972,  373,  513, 2827, //  1600
     586, 2377, 2391, 1003, 1976, 1631, 6122, 2464, 1084,  648, 1776, 4626, 2141,  324,  962, 2012, //  1616
    2177, 2076, 1384,  742, 2178, 1448, 1173, 1810,  222,  102,  301,  445,  125, 2420,  662, 2498, //  1632
     277,  200, 1476, 1165, 1068,  224, 2562, 1378, 1446,  450, 1880,  659,  791,  582, 4627, 2939, //  1648
    3936, 1516, 1274,  555, 2099, 3697, 1020, 1389, 1526, 3380, 1762, 1723, 1787, 2229,  412, 2114, //  1664
    1900, 2392, 3518,  512, 2597,  427, 1925, 2341, 3122, 1653, 1686, 2465, 2499,  697,  330,  273, //  1680
     380, 2162,  951,  832,  780,  991, 1301, 3073,  965, 2270, 3519,  668, 2523, 2636, 1286,  535, //  1696
    1407,  518,  671,  957, 2658, 2378,  267,  611, 2197, 3030, 6123,  248, 2299,  967, 1799, 2356, //  1712
     850, 1418, 3437, 1876, 1256, 1480, 2828, 1718, 6124, 6125, 1755, 1664, 2405, 6126, 4628, 2879, //  1728
    2829,  499, 2179,  676, 4629,  557, 2329, 2214, 2090,  325, 3234,  464,  811, 3001,  992, 2342, //  1744
    2481, 1232, 1469,  303, 2242,  466, 1070, 2163,  603, 1777, 2091, 4630, 2752, 4631, 2714,  322, //  1760
    2659, 1964, 1768,  481, 2188, 1463, 2330, 2857, 3600, 2092, 3031, 2421, 4632, 2318, 2070, 1849, //  1776
    2598, 4633, 1302, 2254, 1668, 1701, 2422, 3811, 2905, 3032, 3123, 2046, 4106, 1763, 1694, 4634, //  1792
    1604,  943, 1724, 1454,  917,  868, 2215, 1169, 2940,  552, 1145, 1800, 1228, 1823, 1955,  316, //  1808
    1080, 2510,  361, 1807, 2830, 4107, 2660, 3381, 1346, 1423, 1134, 4108, 6127,  541, 1263, 1229, //  1824
    1148, 2540,  545,  465, 1833, 2880, 3438, 1901, 3074, 2482,  816, 3937,  713, 1788, 2500,  122, //  1840
    1575,  195, 1451, 2501, 1111, 6128,  859,  374, 1225, 2243, 2483, 4317,  390, 1033, 3439, 3075, //  1856
    2524, 1687,  266,  793, 1440, 2599,  946,  779,  802,  507,  897, 1081,  528, 2189, 1292,  711, //  1872
    1866, 1725, 1167, 1640,  753,  398, 2661, 1053,  246,  348, 4318,  137, 1024, 3440, 1600, 2077, //  1888
    2129,  825, 4319,  698,  238,  521,  187, 2300, 1157, 2423, 1641, 1605, 1464, 1610, 1097, 2541, //  1904
    1260, 1436,  759, 2255, 1814, 2150,  705, 3235,  409, 2563, 3304,  561, 3033, 2005, 2564,  726, //  1920
    1956, 2343, 3698, 4109,  949, 3812, 3813, 3520, 1669,  653, 1379, 2525,  881, 2198,  632, 2256, //  1936
    1027,  778, 1074,  733, 1957,  514, 1481, 2466,  554, 2180,  702, 3938, 1606, 1017, 1398, 6129, //  1952
    1380, 3521,  921,  993, 1313,  594,  449, 1489, 1617, 1166,  768, 1426, 1360,  495, 1794, 3601, //  1968
    1177, 3602, 1170, 4320, 2344,  476,  425, 3167, 4635, 3168, 1424,  401, 2662, 1171, 3382, 1998, //  1984
    1089, 4110,  477, 3169,  474, 6130, 1909,  596, 2831, 1842,  494,  693, 1051, 1028, 1207, 3076, //  2000
     606, 2115,  727, 2790, 1473, 1115,  743, 3522,  630,  805, 1532, 4321, 2021,  366, 1057,  838, //  2016
     684, 1114, 2142, 4322, 2050, 1492, 1892, 1808, 2271, 3814, 2424, 1971, 1447, 1373, 3305, 1090, //  2032
    1536, 3939, 3523, 3306, 1455, 2199,  336,  369, 2331, 1035,  584, 2393,  902,  718, 2600, 6131, //  2048
    2753,  463, 2151, 1149, 1611, 2467,  715, 1308, 3124, 1268,  343, 1413, 3236, 1517, 1347, 2663, //  2064
    2093, 3940, 2022, 1131, 1553, 2100, 2941, 1427, 3441, 2942, 1323, 2484, 6132, 1980,  872, 2368, //  2080
    2441, 2943,  320, 2369, 2116, 1082,  679, 1933, 3941, 2791, 3815,  625, 1143, 2023,  422, 2200, //  2096
    3816, 6133,  730, 1695,  356, 2257, 1626, 2301, 2858, 2637, 1627, 1778,  937,  883, 2906, 2693, //  2112
    3002, 1769, 1086,  400, 1063, 1325, 3307, 2792, 4111, 3077,  456, 2345, 1046,  747, 6134, 1524, //  2128
     884, 1094, 3383, 1474, 2164, 1059,  974, 1688, 2181, 2258, 1047,  345, 1665, 1187,  358,  875, //  2144
    3170,  305,  660, 3524, 2190, 1334, 1135, 3171, 1540, 1649, 2542, 1527,  927,  968, 2793,  885, //  2160
    1972, 1850,  482,  500, 2638, 1218, 1109, 1085, 2543, 1654, 2034,  876,   78, 2287, 1482, 1277, //  2176
     861, 1675, 1083, 1779,  724, 2754,  454,  397, 1132, 1612, 2332,  893,  672, 1237,  257, 2259, //  2192
    2370,  135, 3384,  337, 2244,  547,  352,  340,  709, 2485, 1400,  788, 1138, 2511,  540,  772, //  2208
    1682, 2260, 2272, 2544, 2013, 1843, 1902, 4636, 1999, 1562, 2288, 4637, 2201, 1403, 1533,  407, //  2224
     576, 3308, 1254, 2071,  978, 3385,  170,  136, 1201, 3125, 2664, 3172, 2394,  213,  912,  873, //  2240
    3603, 1713, 2202,  699, 3604, 3699,  813, 3442,  493,  531, 1054,  468, 2907, 1483,  304,  281, //  2256
    4112, 1726, 1252, 2094,  339, 2319, 2130, 2639,  756, 1563, 2944,  748,  571, 2976, 1588, 2425, //  2272
    2715, 1851, 1460, 2426, 1528, 1392, 1973, 3237,  288, 3309,  685, 3386,  296,  892, 2716, 2216, //  2288
    1570, 2245,  722, 1747, 2217,  905, 3238, 1103, 6135, 1893, 1441, 1965,  251, 1805, 2371, 3700, //  2304
    2601, 1919, 1078,   75, 2182, 1509, 1592, 1270, 2640, 4638, 2152, 6136, 3310, 3817,  524,  706, //  2320
    1075,  292, 3818, 1756, 2602,  317,   98, 3173, 3605, 3525, 1844, 2218, 3819, 2502,  814,  567, //  2336
     385, 2908, 1534, 6137,  534, 1642, 3239,  797, 6138, 1670, 1529,  953, 4323,  188, 1071,  538, //  2352
     178,  729, 3240, 2109, 1226, 1374, 2000, 2357, 2977,  731, 2468, 1116, 2014, 2051, 6139, 1261, //  2368
    1593,  803, 2859, 2736, 3443,  556,  682,  823, 1541, 6140, 1369, 2289, 1706, 2794,  845,  462, //  2384
    2603, 2665, 1361,  387,  162, 2358, 1740,  739, 1770, 1720, 1304, 1401, 3241, 1049,  627, 1571, //  2400
    2427, 3526, 1877, 3942, 1852, 1500,  431, 1910, 1503,  677,  297, 2795,  286, 1433, 1038, 1198, //  2416
    2290, 1133, 1596, 4113, 4639, 2469, 1510, 1484, 3943, 6141, 2442,  108,  712, 4640, 2372,  866, //  2432
    3701, 2755, 3242, 1348,  834, 1945, 1408, 3527, 2395, 3243, 1811,  824,  994, 1179, 2110, 1548, //  2448
    1453,  790, 3003,  690, 4324, 4325, 2832, 2909, 3820, 1860, 3821,  225, 1748,  310,  346, 1780, //  2464
    2470,  821, 1993, 2717, 2796,  828,  877, 3528, 2860, 2471, 1702, 2165, 2910, 2486, 1789,  453, //  2480
     359, 2291, 1676,   73, 1164, 1461, 1127, 3311,  421,  604,  314, 1037,  589,  116, 2487,  737, //  2496
     837, 1180,  111,  244,  735, 6142, 2261, 1861, 1362,  986,  523,  418,  581, 2666, 3822,  103, //  2512
     855,  503, 1414, 1867, 2488, 1091,  657, 1597,  979,  605, 1316, 4641, 1021, 2443, 2078, 2001, //  2528
    1209,   96,  587, 2166, 1032,  260, 1072, 2153,  173,   94,  226, 3244,  819, 2006, 4642, 4114, //  2544
    2203,  231, 1744,  782,   97, 2667,  786, 3387,  887,  391,  442, 2219, 4326, 1425, 6143, 2694, //  2560
     633, 1544, 1202,  483, 2015,  592, 2052, 1958, 2472, 1655,  419,  129, 4327, 3444, 3312, 1714, //  2576
    1257, 3078, 4328, 1518, 1098,  865, 1310, 1019, 1885, 1512, 1734,  469, 2444,  148,  773,  436, //  2592
    1815, 1868, 1128, 1055, 4329, 1245, 2756, 3445, 2154, 1934, 1039, 4643,  579, 1238,  932, 2320, //  2608
     353,  205,  801,  115, 2428,  944, 2321, 1881,  399, 2565, 1211,  678,  766, 3944,  335, 2101, //  2624
    1459, 1781, 1402, 3945, 2737, 2131, 1010,  844,  981, 1326, 1013,  550, 1816, 1545, 2620, 1335, //  2640
    1008,  371, 2881,  936, 1419, 1613, 3529, 1456, 1395, 2273, 1834, 2604, 1317, 2738, 2503,  416, //  2656
    1643, 4330,  806, 1126,  229,  591, 3946, 1314, 1981, 1576, 1837, 1666,  347, 1790,  977, 3313, //  2672
     764, 2861, 1853,  688, 2429, 1920, 1462,   77,  595,  415, 2002, 3034,  798, 1192, 4115, 6144, //  2688
    2978, 4331, 3035, 2695, 2582, 2072, 2566,  430, 2430, 1727,  842, 1396, 3947, 3702,  613,  377, //  2704
     278,  236, 1417, 3388, 3314, 3174,  757, 1869,  107, 3530, 6145, 1194,  623, 2262,  207, 1253, //  2720
    2167, 3446, 3948,  492, 1117, 1935,  536, 1838, 2757, 1246, 4332,  696, 2095, 2406, 1393, 1572, //  2736
    3175, 1782,  583,  190,  253, 1390, 2230,  830, 3126, 3389,  934, 3245, 1703, 1749, 2979, 1870, //  2752
    2545, 1656, 2204,  869, 2346, 4116, 3176, 1817,  496, 1764, 4644,  942, 1504,  404, 1903, 1122, //  2768
    1580, 3606, 2945, 1022,  515,  372, 1735,  955, 2431, 3036, 6146, 2797, 1110, 2302, 2798,  617, //  2784
    6147,  441,  762, 1771, 3447, 3607, 3608, 1904,  840, 3037,   86,  939, 1385,  572, 1370, 2445, //  2800
    1336,  114, 3703,  898,  294,  203, 3315,  703, 1583, 2274,  429,  961, 4333, 1854, 1951, 3390, //  2816
    2373, 3704, 4334, 1318, 1381,  966, 1911, 2322, 1006, 1155,  309,  989,  458, 2718, 1795, 1372, //  2832
    1203,  252, 1689, 1363, 3177,  517, 1936,  168, 1490,  562,  193, 3823, 1042, 4117, 1835,  551, //  2848
     470, 4645,  395,  489, 3448, 1871, 1465, 2583, 2641,  417, 1493,  279, 1295,  511, 1236, 1119, //  2864
      72, 1231, 1982, 1812, 3004,  871, 1564,  984, 3449, 1667, 2696, 2096, 4646, 2347, 2833, 1673, //  2880
    3609,  695, 3246, 2668,  807, 1183, 4647,  890,  388, 2333, 1801, 1457, 2911, 1765, 1477, 1031, //  2896
    3316, 3317, 1278, 3391, 2799, 2292, 2526,  163, 3450, 4335, 2669, 1404, 1802, 6148, 2323, 2407, //  2912
    1584, 1728, 1494, 1824, 1269,  298,  909, 3318, 1034, 1632,  375,  776, 1683, 2061,  291,  210, //  2928
    1123,  809, 1249, 1002, 2642, 3038,  206, 1011, 2132,  144,  975,  882, 1565,  342,  667,  754, //  2944
    1442, 2143, 1299, 2303, 2062,  447,  626, 2205, 1221, 2739, 2912, 1144, 1214, 2206, 2584,  760, //  2960
    1715,  614,  950, 1281, 2670, 2621,  810,  577, 1287, 2546, 4648,  242, 2168,  250, 2643,  691, //  2976
     123, 2644,  647,  313, 1029,  689, 1357, 2946, 1650,  216,  771, 1339, 1306,  808, 2063,  549, //  2992
     913, 1371, 2913, 2914, 6149, 1466, 1092, 1174, 1196, 1311, 2605, 2396, 1783, 1796, 3079,  406, //  3008
    2671, 2117, 3949, 4649,  487, 1825, 2220, 6150, 2915,  448, 2348, 1073, 6151, 2397, 1707,  130, //  3024
     900, 1598,  329,  176, 1959, 2527, 1620, 6152, 2275, 4336, 3319, 1983, 2191, 3705, 3610, 2155, //  3040
    3706, 1912, 1513, 1614, 6153, 1988,  646,  392, 2304, 1589, 3320, 3039, 1826, 1239, 1352, 1340, //  3056
    2916,  505, 2567, 1709, 1437, 2408, 2547,  906, 6154, 2672,  384, 1458, 1594, 1100, 1329,  710, //  3072
     423, 3531, 2064, 2231, 2622, 1989, 2673, 1087, 1882,  333,  841, 3005, 1296, 2882, 2379,  580, //  3088
    1937, 1827, 1293, 2585,  601,  574,  249, 1772, 4118, 2079, 1120,  645,  901, 1176, 1690,  795, //  3104
    2207,  478, 1434,  516, 1190, 1530,  761, 2080,  930, 1264,  355,  435, 1552,  644, 1791,  987, //  3120
     220, 1364, 1163, 1121, 1538,  306, 2169, 1327, 1222,  546, 2645,  218,  241,  610, 1704, 3321, //  3136
    1984, 1839, 1966, 2528,  451, 6155, 2586, 3707, 2568,  907, 3178,  254, 2947,  186, 1845, 4650, //  3152
     745,  432, 1757,  428, 1633,  888, 2246, 2221, 2489, 3611, 2118, 1258, 1265,  956, 3127, 1784, //  3168
    4337, 2490,  319,  510,  119,  457, 3612,  274, 2035, 2007, 4651, 1409, 3128,  970, 2758,  590, //  3184
    2800,  661, 2247, 4652, 2008, 3950, 1420, 1549, 3080, 3322, 3951, 1651, 1375, 2111,  485, 2491, //  3200
    1429, 1156, 6156, 2548, 2183, 1495,  831, 1840, 2529, 2446,  501, 1657,  307, 1894, 3247, 1341, //  3216
     666,  899, 2156, 1539, 2549, 1559,  886,  349, 2208, 3081, 2305, 1736, 3824, 2170, 2759, 1014, //  3232
    1913, 1386,  542, 1397, 2948,  490,  368,  716,  362,  159,  282, 2569, 1129, 1658, 1288, 1750, //  3248
    2674,  276,  649, 2016,  751, 1496,  658, 1818, 1284, 1862, 2209, 2087, 2512, 3451,  622, 2834, //  3264
     376,  117, 1060, 2053, 1208, 1721, 1101, 1443,  247, 1250, 3179, 1792, 3952, 2760, 2398, 3953, //  3280
    6157, 2144, 3708,  446, 2432, 1151, 2570, 3452, 2447, 2761, 2835, 1210, 2448, 3082,  424, 2222, //  3296
    1251, 2449, 2119, 2836,  504, 1581, 4338,  602,  817,  857, 3825, 2349, 2306,  357, 3826, 1470, //  3312
    1883, 2883,  255,  958,  929, 2917, 3248,  302, 4653, 1050, 1271, 1751, 2307, 1952, 1430, 2697, //  3328
    2719, 2359,  354, 3180,  777,  158, 2036, 4339, 1659, 4340, 4654, 2308, 2949, 2248, 1146, 2232, //  3344
    3532, 2720, 1696, 2623, 3827, 6158, 3129, 1550, 2698, 1485, 1297, 1428,  637,  931, 2721, 2145, //  3360
     914, 2550, 2587,   81, 2450,  612,  827, 2646, 1242, 4655, 1118, 2884,  472, 1855, 3181, 3533, //  3376
    3534,  569, 1353, 2699, 1244, 1758, 2588, 4119, 2009, 2762, 2171, 3709, 1312, 1531, 6159, 1152, //  3392
    1938,  134, 1830,  471, 3710, 2276, 1112, 1535, 3323, 3453, 3535,  982, 1337, 2950,  488,  826, //  3408
     674, 1058, 1628, 4120, 2017,  522, 2399,  211,  568, 1367, 3454,  350,  293, 1872, 1139, 3249, //  3424
    1399, 1946, 3006, 1300, 2360, 3324,  588,  736, 6160, 2606,  744,  669, 3536, 3828, 6161, 1358, //  3440
     199,  723,  848,  933,  851, 1939, 1505, 1514, 1338, 1618, 1831, 4656, 1634, 3613,  443, 2740, //  3456
    3829,  717, 1947,  491, 1914, 6162, 2551, 1542, 4121, 1025, 6163, 1099, 1223,  198, 3040, 2722, //  3472
     370,  410, 1905, 2589,  998, 1248, 3182, 2380,  519, 1449, 4122, 1710,  947,  928, 1153, 4341, //  3488
    2277,  344, 2624, 1511,  615,  105,  161, 1212, 1076, 1960, 3130, 2054, 1926, 1175, 1906, 2473, //  3504
     414, 1873, 2801, 6164, 2309,  315, 1319, 3325,  318, 2018, 2146, 2157,  963,  631,  223, 4342, //  3520
    4343, 2675,  479, 3711, 1197, 2625, 3712, 2676, 2361, 6165, 4344, 4123, 6166, 2451, 3183, 1886, //  3536
    2184, 1674, 1330, 1711, 1635, 1506,  799,  219, 3250, 3083, 3954, 1677, 3713, 3326, 2081, 3614, //  3552
    1652, 2073, 4657, 1147, 3041, 1752,  643, 1961,  147, 1974, 3955, 6167, 1716, 2037,  918, 3007, //  3568
    1994,  120, 1537,  118,  609, 3184, 4345,  740, 3455, 1219,  332, 1615, 3830, 6168, 1621, 2980, //  3584
    1582,  783,  212,  553, 2350, 3714, 1349, 2433, 2082, 4124,  889, 6169, 2310, 1275, 1410,  973, //  3600
     166, 1320, 3456, 1797, 1215, 3185, 2885, 1846, 2590, 2763, 4658,  629,  822, 3008,  763,  940, //  3616
    1990, 2862,  439, 2409, 1566, 1240, 1622,  926, 1282, 1907, 2764,  654, 2210, 1607,  327, 1130, //  3632
    3956, 1678, 1623, 6170, 2434, 2192,  686,  608, 3831, 3715,  903, 3957, 3042, 6171, 2741, 1522, //  3648
    1915, 1105, 1555, 2552, 1359,  323, 3251, 4346, 3457,  738, 1354, 2553, 2311, 2334, 1828, 2003, //  3664
    3832, 1753, 2351, 1227, 6172, 1887, 4125, 1478, 6173, 2410, 1874, 1712, 1847,  520, 1204, 2607, //  3680
     264, 4659,  836, 2677, 2102,  600, 4660, 3833, 2278, 3084, 6174, 4347, 3615, 1342,  640,  532, //  3696
     543, 2608, 1888, 2400, 2591, 1009, 4348, 1497,  341, 1737, 3616, 2723, 1394,  529, 3252, 1321, //  3712
     983, 4661, 1515, 2120,  971, 2592,  924,  287, 1662, 3186, 4349, 2700, 4350, 1519,  908, 1948, //  3728
    2452,  156,  796, 1629, 1486, 2223, 2055,  694, 4126, 1259, 1036, 3392, 1213, 2249, 2742, 1889, //  3744
    1230, 3958, 1015,  910,  408,  559, 3617, 4662,  746,  725,  935, 4663, 3959, 3009, 1289,  563, //  3760
     867, 4664, 3960, 1567, 2981, 2038, 2626,  988, 2263, 2381, 4351,  143, 2374,  704, 1895, 6175, //  3776
    1188, 3716, 2088,  673, 3085, 2362, 4352,  484, 1608, 1921, 2765, 2918,  215,  904, 3618, 3537, //  3792
     894,  509,  976, 3043, 2701, 3961, 4353, 2837, 2982,  498, 6176, 6177, 1102, 3538, 1332, 3393, //  3808
    1487, 1636, 1637,  233,  245, 3962,  383,  650,  995, 3044,  460, 1520, 1206, 2352,  749, 3327, //  3824
     530,  700,  389, 1438, 1560, 1773, 3963, 2264,  719, 2951, 2724, 3834,  870, 1832, 1644, 1000, //  3840
     839, 2474, 3717,  197, 1630, 3394,  365, 2886, 3964, 1285, 2133,  734,  922,  818, 1106,  732, //  3856
     480, 2083, 1774, 3458,  923, 2279, 1350,  221, 3086,   85, 2233, 2234, 3835, 1585, 3010, 2147, //  3872
    1387, 1705, 2382, 1619, 2475,  133,  239, 2802, 1991, 1016, 2084, 2383,  411, 2838, 1113,  651, //  3888
    1985, 1160, 3328,  990, 1863, 3087, 1048, 1276, 2647,  265, 2627, 1599, 3253, 2056,  150,  638, //  3904
    2019,  656,  853,  326, 1479,  680, 1439, 4354, 1001, 1759,  413, 3459, 3395, 2492, 1431,  459, //  3920
    4355, 1125, 3329, 2265, 1953, 1450, 2065, 2863,  849,  351, 2678, 3131, 3254, 3255, 1104, 1577, //  3936
     227, 1351, 1645, 2453, 2193, 1421, 2887,  812, 2121,  634,   95, 2435,  201, 2312, 4665, 1646, //  3952
    1671, 2743, 1601, 2554, 2702, 2648, 2280, 1315, 1366, 2089, 3132, 1573, 3718, 3965, 1729, 1189, //  3968
     328, 2679, 1077, 1940, 1136,  558, 1283,  964, 1195,  621, 2074, 1199, 1743, 3460, 3619, 1896, //  3984
    1916, 1890, 3836, 2952, 1154, 2112, 1064,  862,  378, 3011, 2066, 2113, 2803, 1568, 2839, 6178, //  4000
    3088, 2919, 1941, 1660, 2004, 1992, 2194,  142,  707, 1590, 1708, 1624, 1922, 1023, 1836, 1233, //  4016
    1004, 2313,  789,  741, 3620, 6179, 1609, 2411, 1200, 4127, 3719, 3720, 4666, 2057, 3721,  593, //  4032
    2840,  367, 2920, 1878, 6180, 3461, 1521,  628, 1168,  692, 2211, 2649,  300,  720, 2067, 2571, //  4048
    2953, 3396,  959, 2504, 3966, 3539, 3462, 1977,  701, 6181,  954, 1043,  800,  681,  183, 3722, //  4064
    1803, 1730, 3540, 4128, 2103,  815, 2314,  174,  467,  230, 2454, 1093, 2134,  755, 3541, 3397, //  4080
    1141, 1162, 6182, 1738, 2039,  270, 3256, 2513, 1005, 1647, 2185, 3837,  858, 1679, 1897, 1719, //  4096
    2954, 2324, 1806,  402,  670,  167, 4129, 1498, 2158, 2104,  750, 6183,  915,  189, 1680, 1551, //  4112
     455, 4356, 1501, 2455,  405, 1095, 2955,  338, 1586, 1266, 1819,  570,  641, 1324,  237, 1556, //  4128
    2650, 1388, 3723, 6184, 1368, 2384, 1343, 1978, 3089, 2436,  879, 3724,  792, 1191,  758, 3012, //  4144
    1411, 2135, 1322, 4357,  240, 4667, 1848, 3725, 1574, 6185,  420, 3045, 1546, 1391,  714, 4358, //  4160
    1967,  941, 1864,  863,  664,  426,  560, 1731, 2680, 1785, 2864, 1949, 2363,  403, 3330, 1415, //  4176
    1279, 2136, 1697, 2335,  204,  721, 2097, 3838,   90, 6186, 2085, 2505,  191, 3967,  124, 2148, //  4192
    1376, 1798, 1178, 1107, 1898, 1405,  860, 4359, 1243, 1272, 2375, 2983, 1558, 2456, 1638,  113, //  4208
    3621,  578, 1923, 2609,  880,  386, 4130,  784, 2186, 2266, 1422, 2956, 2172, 1722,  497,  263, //  4224
    2514, 1267, 2412, 2610,  177, 2703, 3542,  774, 1927, 1344,  616, 1432, 1595, 1018,  172, 4360, //  4240
    2325,  911, 4361,  438, 1468, 3622,  794, 3968, 2024, 2173, 1681, 1829, 2957,  945,  895, 3090, //  4256
     575, 2212, 2476,  475, 2401, 2681,  785, 2744, 1745, 2293, 2555, 1975, 3133, 2865,  394, 4668, //  4272
    3839,  635, 4131,  639,  202, 1507, 2195, 2766, 1345, 1435, 2572, 3726, 1908, 1184, 1181, 2457, //  4288
    3727, 3134, 4362,  843, 2611,  437,  916, 4669,  234,  769, 1884, 3046, 3047, 3623,  833, 6187, //  4304
    1639, 2250, 2402, 1355, 1185, 2010, 2047,  999,  525, 1732, 1290, 1488, 2612,  948, 1578, 3728, //  4320
    2413, 2477, 1216, 2725, 2159,  334, 3840, 1328, 3624, 2921, 1525, 4132,  564, 1056,  891, 4363, //  4336
    1444, 1698, 2385, 2251, 3729, 1365, 2281, 2235, 1717, 6188,  864, 3841, 2515,  444,  527, 2767, //  4352
    2922, 3625,  544,  461, 6189,  566,  209, 2437, 3398, 2098, 1065, 2068, 3331, 3626, 3257, 2137, //  4368
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jis_char_to_freq_order_len() {
        assert_eq!(JIS_CHAR_TO_FREQ_ORDER.len(), JIS_TABLE_SIZE);
    }
}
