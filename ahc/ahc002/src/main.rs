use proconio::{input, source::once::OnceSource};
use rand::Rng;
use std::collections::HashSet;

fn main() {
    input! {
        // from OnceSource::from(source()),
        si: usize,
        sj: usize,
        t: [[usize; 50]; 50],
        _p: [[usize; 50]; 50]
    }
    let mut answer = "".to_string();
    for _ in 0..200 {
        let mut visited = HashSet::new();
        visited.insert((si, sj));
        for e in same(&t, (si, sj)) {
            visited.insert(e);
        }

        let mut point = (si, sj);
        let mut ans = "".to_string();

        let mut rng = rand::thread_rng();
        if rng.gen() {};

        loop {
            let able = check(&visited, point);
            if able.len() == 0 {
                break;
            }
            let choice = rng.gen::<usize>() % able.len();
            let new_point = able[choice];
            insert_s_p(&mut visited, new_point, &t);
            if point.0 - 1 == new_point.0 {
                ans = ans + "U";
            } else if point.1 + 1 == new_point.1 {
                ans = ans + "R";
            } else if point.0 + 1 == new_point.0 {
                ans = ans + "D";
            } else {
                ans = ans + "L";
            }
            point = new_point;
        }

        if answer.len() < ans.len() {
            answer = ans;
        }
    }
    println!("{}", answer);
}

fn check(set: &HashSet<(usize, usize)>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut v = vec![];
    if (!set.contains(&(point.0 - 1, point.1))) && point.0 != 0 {
        v.push((point.0 - 1, point.1));
    }
    if (!set.contains(&(point.0, point.1 + 1))) && point.1 + 1 < 50 {
        v.push((point.0, point.1 + 1))
    }
    if (!set.contains(&(point.0 + 1, point.1))) && point.0 + 1 < 50 {
        v.push((point.0 + 1, point.1));
    }
    if (!set.contains(&(point.0, point.1 - 1))) && point.1 != 0 {
        v.push((point.0, point.1 - 1));
    }
    v
}

fn same(t: &Vec<Vec<usize>>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut v = vec![];
    if point.0 != 0 && t[point.0][point.1] == t[point.0 - 1][point.1] {
        v.push((point.0 - 1, point.1));
    }
    if point.0 + 1 != 50 && t[point.0][point.1] == t[point.0 + 1][point.1] {
        v.push((point.0 + 1, point.1));
    }
    if point.1 != 0 && t[point.0][point.1] == t[point.0][point.1 - 1] {
        v.push((point.0, point.1 - 1));
    }
    if point.1 + 1 != 50 && t[point.0][point.1] == t[point.0][point.1 + 1] {
        v.push((point.0, point.1 + 1))
    }
    v
}

fn insert_s_p(set: &mut HashSet<(usize, usize)>, new_point: (usize, usize), t: &Vec<Vec<usize>>) {
    set.insert(new_point);
    for e in same(&t, new_point) {
        set.insert(e);
    }
}

fn source() -> &'static str {
    "
    40 40
0 1 2 3 3 4 4 5 6 6 7 8 8 9 10 10 11 11 12 12 13 14 15 15 16 16 17 17 18 19 19 20 20 21 22 23 24 25 25 26 26 27 27 28 29 30 30 31 31 32
0 1 33 33 34 35 35 5 36 36 37 37 38 9 39 39 40 41 42 43 13 44 44 45 46 47 48 48 18 49 49 50 50 21 22 23 24 51 51 52 52 53 53 28 54 54 55 56 57 32
58 59 60 61 34 62 62 63 64 65 66 67 68 68 69 70 71 41 42 72 72 73 73 45 46 74 74 75 76 77 78 79 80 81 82 83 84 85 86 86 87 87 88 88 89 89 55 56 90 91
58 92 60 93 93 94 94 63 64 65 66 95 96 97 69 98 71 99 99 100 101 101 102 102 103 103 104 75 76 77 105 79 106 81 82 83 84 107 107 108 108 109 110 110 111 111 112 113 90 114
115 92 116 116 117 117 118 119 119 120 120 95 96 121 121 98 122 123 124 124 125 125 126 127 127 128 104 129 130 131 105 132 106 133 133 134 134 135 136 137 137 109 138 139 139 140 112 113 141 114
142 143 143 144 145 145 118 146 147 148 148 149 149 150 151 152 122 123 153 154 154 155 156 157 157 158 158 129 130 159 160 132 161 162 162 163 163 135 136 164 164 165 165 166 166 140 167 167 141 168
142 169 170 144 171 172 172 146 147 173 173 174 175 150 151 152 176 176 177 178 178 155 156 179 180 181 181 182 183 159 184 185 185 186 187 187 188 188 189 189 190 191 191 192 193 194 194 195 195 168
196 169 170 197 197 198 198 199 199 200 200 174 175 201 201 202 203 203 177 204 205 206 206 179 180 207 207 182 208 209 184 210 211 186 212 213 213 214 215 215 216 216 217 218 193 219 219 220 221 222
223 223 224 225 225 226 226 227 228 228 229 229 230 231 231 202 232 232 233 233 205 234 234 235 236 236 237 237 208 238 238 210 211 239 239 240 240 214 241 242 242 243 243 218 244 245 245 220 221 222
246 247 248 249 249 250 250 227 251 252 252 253 254 255 256 257 258 258 259 259 260 261 261 262 263 264 264 265 265 266 266 267 267 268 269 270 270 271 241 272 273 273 274 274 244 275 276 276 277 277
278 247 248 279 279 280 281 281 282 282 283 283 254 255 256 284 285 286 287 288 260 289 289 262 263 290 291 292 293 294 294 295 295 268 269 296 296 297 298 272 299 299 300 301 301 275 302 303 303 304
278 305 305 306 306 280 307 308 309 310 311 311 312 312 313 284 285 314 287 288 315 316 317 318 319 290 291 292 293 320 320 321 322 323 323 324 325 297 298 326 327 327 300 328 328 329 330 330 331 331
332 333 334 335 335 336 336 308 309 337 338 338 339 340 340 341 341 314 342 343 315 316 317 318 319 344 345 346 346 347 347 321 348 349 350 324 325 351 351 326 352 353 353 354 355 329 356 357 358 358
332 333 359 360 361 362 362 363 363 337 364 365 366 366 367 367 368 369 370 343 371 372 373 374 374 344 345 375 376 376 377 377 348 349 350 378 379 380 381 381 352 382 383 354 384 384 356 385 385 386
387 387 359 360 361 388 389 389 390 390 364 365 391 392 393 394 368 369 370 395 371 372 396 397 398 399 400 375 401 401 402 402 403 404 404 378 379 380 405 406 406 407 383 408 408 409 409 410 410 386
411 412 412 413 413 414 414 415 416 416 417 417 391 392 393 394 418 418 419 395 420 421 396 397 398 399 400 422 423 423 424 425 403 426 426 427 427 428 405 429 429 407 430 431 431 432 433 433 434 434
411 435 435 436 436 437 437 438 438 439 440 440 441 441 442 442 443 443 444 444 420 445 445 446 446 447 447 422 448 449 424 425 450 451 452 453 454 428 455 455 456 457 430 458 458 432 459 459 460 461
462 463 463 464 465 465 466 466 467 439 468 468 469 470 470 471 471 472 473 473 474 474 475 475 476 477 477 478 448 449 479 479 450 480 452 453 454 481 481 482 456 457 483 483 484 485 485 486 486 461
487 488 488 464 489 489 490 491 492 493 493 494 469 495 495 496 496 497 497 498 499 500 501 501 476 502 502 478 503 504 505 505 506 480 507 508 509 509 510 482 511 512 513 514 484 515 516 517 518 518
487 519 519 520 520 521 522 491 492 523 523 524 524 525 526 526 527 528 529 529 499 500 530 531 531 532 532 533 503 504 534 534 535 536 507 508 537 538 538 539 511 540 513 514 541 541 516 517 542 542
543 544 545 545 546 547 522 548 549 550 550 551 552 553 554 555 556 528 557 557 558 558 530 559 560 561 562 563 563 564 564 565 535 566 567 568 537 569 569 539 570 540 571 571 572 572 573 574 574 575
543 544 576 576 546 547 577 548 578 578 579 579 552 553 580 555 556 581 582 583 583 584 585 559 586 561 562 587 588 589 590 565 591 566 567 568 592 593 594 595 570 596 596 597 598 599 599 600 600 575
601 601 602 602 603 604 577 605 605 606 607 607 608 609 580 610 610 581 611 611 612 584 585 613 586 614 614 615 588 589 590 616 616 617 617 618 618 593 594 595 619 619 620 621 598 622 623 623 624 624
625 626 626 627 603 604 628 629 629 606 630 631 631 609 632 632 633 634 635 636 612 637 637 613 638 639 640 615 641 641 642 642 643 644 645 645 646 647 647 648 649 649 620 621 650 622 651 652 653 654
655 655 656 627 657 658 659 659 660 660 661 661 662 662 663 664 633 634 635 636 665 665 666 667 667 639 640 668 669 670 670 671 671 644 672 673 674 675 675 648 676 677 677 678 650 679 651 680 653 654
681 681 682 682 657 658 683 684 684 685 686 687 688 689 663 664 690 691 692 693 693 694 666 695 695 696 696 668 697 697 698 699 700 700 672 673 674 701 701 702 676 703 703 704 704 705 705 680 706 706
707 707 708 709 709 710 711 712 712 685 686 687 688 689 713 713 690 691 692 714 714 715 715 716 716 717 717 718 719 719 698 699 720 721 721 722 723 724 725 702 726 727 727 728 728 729 729 730 731 731
732 732 708 733 734 710 711 735 735 736 736 737 738 739 740 741 742 743 744 744 745 746 747 747 748 749 750 750 751 752 752 753 753 754 754 722 723 724 725 755 726 756 757 758 758 759 759 760 761 762
763 764 764 733 765 766 766 767 768 769 769 737 770 739 740 741 771 743 772 773 745 774 774 775 775 749 776 776 751 777 778 778 779 779 780 781 782 783 784 755 785 756 757 786 787 787 788 760 761 762
763 789 790 790 765 791 791 767 768 792 792 793 770 794 794 795 771 796 772 797 797 798 799 799 800 801 801 802 802 777 803 803 804 805 780 781 782 806 784 807 808 809 810 786 811 811 788 812 813 813
814 789 815 816 817 818 818 819 820 820 821 793 822 822 823 823 824 796 825 826 827 798 828 828 800 829 830 831 831 832 833 833 804 805 834 835 835 806 836 807 808 809 810 837 838 838 839 840 841 842
814 843 844 816 817 845 846 819 847 847 848 849 849 850 851 851 824 852 825 826 827 853 853 854 855 829 830 856 856 832 857 858 858 859 860 860 861 861 862 862 863 864 864 865 866 867 839 840 841 842
868 843 844 869 869 845 870 870 871 871 848 872 872 850 873 873 874 852 875 875 876 877 877 854 878 879 879 880 881 882 883 883 884 859 885 885 886 887 888 889 889 890 890 865 866 867 891 892 892 893
894 894 895 895 896 897 898 899 900 901 901 902 903 904 904 905 874 906 906 907 908 908 909 909 878 910 911 880 881 882 912 912 884 913 913 914 886 915 888 916 916 917 917 918 919 919 920 920 921 893
922 922 923 923 896 897 898 899 924 924 925 902 903 926 926 927 927 928 929 907 930 930 931 932 933 910 911 934 934 935 936 937 938 939 939 914 940 915 941 942 942 943 943 918 944 945 945 946 947 947
948 948 949 949 950 951 952 953 954 955 955 956 957 958 959 960 961 928 929 962 962 963 931 932 964 965 966 967 967 935 936 937 938 968 969 969 940 970 941 971 972 973 973 974 974 975 976 946 977 978
979 980 980 981 950 951 982 953 954 983 984 956 957 958 959 960 961 985 986 987 988 988 989 990 964 965 991 991 992 993 993 994 995 968 996 997 998 970 999 971 972 1000 1001 1002 1002 975 976 1003 977 1004
1005 1006 1007 1008 1008 1009 982 1010 1010 983 1011 1012 1013 1013 1014 1015 1015 985 986 987 1016 1017 989 990 1018 1018 1019 1019 992 1020 1021 994 995 1022 996 997 1023 1024 1025 1026 1027 1000 1028 1028 1029 1030 1031 1031 1032 1004
1005 1006 1007 1033 1033 1009 1034 1034 1035 1036 1011 1012 1037 1037 1014 1038 1038 1039 1039 1040 1016 1017 1041 1042 1043 1044 1044 1045 1046 1020 1021 1047 1048 1049 1050 1050 1023 1024 1025 1026 1027 1051 1052 1052 1029 1053 1053 1054 1055 1055
1056 1056 1057 1058 1058 1059 1060 1060 1035 1036 1061 1061 1062 1062 1063 1063 1064 1065 1066 1066 1067 1067 1068 1042 1043 1069 1070 1070 1046 1071 1072 1072 1048 1049 1073 1074 1075 1076 1077 1077 1078 1079 1080 1081 1081 1082 1082 1054 1083 1084
1085 1085 1057 1086 1087 1059 1088 1089 1090 1091 1092 1093 1093 1094 1095 1095 1064 1096 1097 1098 1099 1100 1068 1101 1101 1069 1102 1102 1103 1103 1104 1105 1106 1106 1107 1074 1075 1108 1109 1109 1078 1079 1110 1110 1111 1111 1112 1113 1113 1084
1114 1115 1115 1086 1116 1116 1088 1089 1090 1091 1092 1117 1117 1118 1118 1119 1120 1096 1097 1098 1099 1100 1121 1121 1122 1122 1123 1124 1125 1125 1104 1105 1126 1126 1107 1127 1127 1108 1128 1128 1129 1130 1131 1131 1132 1133 1134 1134 1135 1136
1114 1137 1138 1138 1139 1139 1140 1141 1142 1143 1144 1144 1145 1145 1146 1119 1120 1147 1148 1149 1149 1150 1151 1151 1152 1153 1123 1124 1154 1155 1156 1157 1157 1158 1159 1159 1160 1161 1161 1162 1129 1130 1163 1163 1132 1164 1165 1166 1135 1167
1168 1137 1169 1170 1171 1171 1140 1141 1142 1143 1172 1173 1174 1174 1146 1175 1175 1147 1148 1176 1177 1150 1178 1179 1152 1153 1180 1180 1181 1155 1156 1182 1183 1184 1184 1185 1160 1186 1187 1162 1188 1188 1189 1189 1190 1164 1165 1191 1191 1167
1192 1192 1169 1170 1193 1194 1195 1196 1196 1197 1172 1173 1198 1199 1199 1200 1200 1201 1201 1176 1177 1202 1178 1179 1203 1204 1205 1206 1181 1207 1208 1208 1183 1209 1210 1210 1211 1186 1187 1212 1213 1214 1215 1216 1217 1218 1219 1220 1221 1221
1222 1222 1223 1224 1225 1194 1195 1226 1227 1227 1228 1229 1198 1230 1230 1231 1231 1232 1232 1233 1234 1202 1235 1236 1203 1204 1205 1237 1237 1207 1238 1238 1239 1240 1240 1241 1211 1242 1242 1212 1213 1214 1215 1243 1217 1218 1219 1220 1244 1244
1245 1245 1223 1224 1225 1246 1246 1226 1247 1248 1228 1229 1249 1249 1250 1251 1252 1252 1253 1233 1234 1254 1235 1255 1255 1256 1257 1258 1258 1259 1260 1261 1239 1262 1262 1241 1263 1264 1264 1265 1265 1266 1267 1243 1268 1269 1270 1271 1272 1273
1274 1275 1276 1276 1277 1277 1278 1278 1247 1279 1279 1280 1280 1281 1281 1251 1282 1282 1253 1283 1283 1284 1284 1285 1285 1256 1257 1286 1287 1259 1260 1261 1288 1289 1289 1290 1290 1291 1292 1293 1294 1266 1295 1295 1268 1296 1270 1271 1272 1273
1274 1275 1297 1297 1298 1298 1299 1299 1300 1300 1301 1302 1303 1304 1305 1305 1306 1307 1308 1309 1310 1311 1311 1312 1313 1313 1314 1286 1315 1316 1317 1317 1288 1318 1318 1319 1320 1320 1292 1293 1294 1321 1322 1323 1323 1296 1324 1325 1325 1326
1327 1327 1328 1328 1329 1329 1330 1331 1331 1332 1332 1302 1303 1304 1333 1333 1306 1307 1308 1309 1334 1334 1335 1312 1336 1336 1314 1337 1315 1316 1338 1338 1339 1340 1340 1319 1341 1342 1342 1343 1343 1321 1322 1344 1345 1345 1324 1346 1346 1326
55 63 54 54 73 30 17 99 88 29 18 27 18 66 94 76 36 69 66 10 36 59 62 10 1 40 14 77 81 71 2 36 85 51 8 70 16 97 33 44 88 11 91 60 31 48 31 89 58 7
9 82 5 55 88 55 52 8 97 81 50 63 28 12 21 5 54 42 54 28 50 40 20 13 28 43 31 87 13 19 60 14 63 29 45 75 64 65 58 69 4 59 26 63 98 88 45 24 43 11
44 42 62 46 37 82 83 43 91 38 52 58 4 85 39 49 56 48 71 25 78 34 34 76 92 9 84 88 88 44 11 27 56 10 88 92 45 35 2 66 80 44 52 11 2 91 13 78 70 20
2 35 13 9 15 48 24 11 94 62 77 60 44 47 32 2 6 67 13 40 16 84 22 43 62 29 16 33 63 14 96 62 94 80 5 17 17 59 12 75 94 8 92 9 11 0 33 32 9 42
50 97 85 95 17 26 34 77 90 39 6 62 71 90 80 60 18 14 46 86 44 86 9 92 22 75 79 2 86 62 4 71 67 16 15 22 79 50 51 34 5 15 20 91 99 2 72 51 27 80
1 63 91 24 39 7 57 97 29 87 49 19 69 37 87 83 91 50 49 73 83 67 55 28 31 59 32 87 71 38 85 61 59 43 59 44 69 48 99 84 70 81 74 98 67 14 58 12 48 85
86 67 10 69 70 63 31 87 53 33 44 41 51 19 9 99 16 28 67 46 0 63 61 33 88 86 89 68 67 49 66 59 97 11 7 84 24 37 87 12 83 17 37 4 11 88 68 44 85 68
7 70 67 43 78 29 12 79 6 31 83 49 7 7 13 24 26 41 24 21 97 50 98 73 46 38 18 72 62 22 83 28 19 34 44 89 27 5 41 64 44 2 33 95 2 58 52 30 56 94
81 33 41 36 76 41 22 22 28 17 67 41 52 18 17 94 96 38 22 22 16 57 90 47 51 49 27 53 69 33 77 6 74 70 86 54 95 47 81 4 20 40 59 35 15 14 23 70 77 10
94 39 9 85 18 45 22 38 17 67 70 25 22 17 28 5 11 45 51 18 77 63 80 46 71 28 70 59 75 79 72 69 91 50 84 10 14 79 7 24 3 48 15 32 9 87 68 99 25 93
25 48 23 96 85 21 27 39 67 2 14 93 47 27 10 20 99 97 12 14 70 34 76 77 46 82 21 68 0 67 55 25 53 73 64 4 47 3 67 64 71 83 61 77 24 52 22 27 72 13
83 99 72 68 71 17 76 13 79 25 27 42 52 67 63 57 19 1 80 71 75 63 79 11 2 24 17 61 43 93 41 80 28 13 63 69 38 36 18 42 52 51 37 56 80 51 10 11 79 58
67 89 38 32 64 35 14 18 87 29 64 56 45 63 49 63 44 57 6 70 59 97 35 55 34 99 52 41 14 45 42 54 35 80 67 47 37 20 71 65 88 52 49 44 52 8 38 84 81 60
37 21 89 8 43 2 7 8 61 44 58 3 42 31 75 33 12 59 53 84 42 79 60 49 34 31 1 0 43 69 65 67 32 4 69 53 2 84 42 89 68 57 91 93 30 22 22 0 91 20
74 39 26 80 35 39 83 6 25 17 50 29 30 47 41 1 54 23 47 91 24 84 17 7 73 97 82 98 42 95 85 49 50 41 12 96 59 97 95 56 4 45 7 2 18 89 26 2 42 94
90 16 94 78 5 88 0 9 90 65 8 62 60 25 65 4 69 59 39 25 88 6 21 20 50 46 41 99 3 60 51 59 24 78 97 11 86 23 20 47 31 39 48 70 43 74 65 41 83 30
44 55 76 3 31 57 87 66 20 98 68 36 64 86 13 17 20 32 24 67 81 14 12 61 84 86 89 68 74 5 82 92 18 62 97 15 83 10 0 31 76 0 72 81 31 70 25 38 51 85
16 9 4 58 14 26 66 93 35 40 20 84 81 91 62 69 60 90 41 32 58 88 72 72 93 50 44 30 65 8 33 25 98 51 21 9 69 14 87 33 83 98 38 26 79 59 68 94 39 23
33 73 71 58 55 74 59 72 91 95 36 77 28 63 98 11 36 37 69 91 85 56 52 63 79 2 59 54 39 46 6 71 47 31 95 86 26 11 46 7 13 23 50 45 46 83 98 83 28 47
46 56 94 61 65 95 18 33 92 42 54 15 69 42 23 49 83 70 68 95 34 89 87 53 82 41 28 5 24 93 22 5 2 26 32 75 6 74 41 5 25 89 17 89 93 17 80 94 73 64
83 52 94 0 15 83 36 23 79 42 76 75 97 26 10 55 50 64 85 98 3 7 86 46 26 17 26 48 46 58 8 18 76 56 67 89 4 94 10 73 69 36 99 54 95 6 60 69 23 70
26 19 69 23 81 75 37 48 25 3 71 16 80 27 49 70 43 65 27 6 30 98 60 43 61 85 13 26 57 98 84 94 51 52 30 9 14 45 22 35 38 41 71 55 48 29 84 15 71 50
90 17 31 66 91 33 70 99 48 37 40 58 34 39 21 37 86 43 9 84 99 0 7 2 71 75 72 42 32 65 49 24 85 35 76 63 82 12 58 4 12 37 77 21 67 53 76 43 32 88
49 22 88 27 84 5 66 28 65 70 57 69 80 79 53 1 88 66 65 46 14 37 1 30 84 54 33 32 27 63 4 20 83 82 21 32 90 62 68 18 48 95 24 5 33 6 47 55 79 1
35 7 83 8 81 14 42 14 64 16 80 25 76 86 95 78 68 74 72 90 92 32 76 29 80 19 84 28 86 96 90 58 19 52 54 40 37 21 59 22 60 38 88 29 51 9 9 23 13 70
29 38 63 55 4 94 73 55 7 30 98 99 75 78 92 59 23 74 33 83 3 15 21 61 61 22 73 13 62 93 31 87 72 61 90 73 55 24 19 68 26 15 37 0 68 50 63 88 94 25
64 74 20 28 50 9 29 66 45 88 83 50 44 64 63 19 91 51 97 90 66 9 19 61 99 40 8 52 79 54 60 7 98 19 76 8 30 25 28 21 85 81 88 93 66 36 6 99 26 60
44 35 43 97 38 70 72 55 72 20 65 49 32 16 33 80 41 13 9 95 48 96 11 95 48 17 42 33 32 15 47 18 73 18 40 55 77 62 57 20 73 88 45 50 6 7 3 52 11 70
63 68 27 7 3 37 35 74 24 62 62 46 91 28 94 23 96 42 99 42 2 39 65 57 41 44 58 18 78 14 22 14 83 96 10 17 56 6 73 33 26 76 15 44 78 54 81 97 85 68
15 65 24 29 89 86 3 42 76 8 47 46 72 55 34 3 23 59 43 93 98 40 80 79 66 9 87 57 75 9 97 90 43 17 59 35 2 93 65 10 9 45 0 4 74 64 18 13 33 14
56 88 32 80 96 63 76 98 68 95 85 46 14 75 46 48 93 8 69 14 33 2 20 58 99 12 28 37 56 53 29 38 52 7 66 89 97 24 64 47 29 76 84 86 36 94 67 40 57 87
29 97 17 21 69 58 25 53 94 69 71 47 49 54 76 47 85 82 91 3 71 95 61 1 0 4 30 95 26 57 66 98 19 86 99 28 36 62 8 86 69 85 57 2 36 8 40 75 62 40
36 61 51 56 71 35 28 6 42 34 52 17 65 17 27 86 54 22 50 63 66 15 76 14 4 95 11 53 31 97 66 21 66 41 38 74 23 51 3 96 83 88 4 58 77 76 53 66 90 1
25 17 16 80 92 40 58 33 8 26 33 12 23 61 19 37 1 58 2 56 68 77 92 23 99 52 32 53 26 2 77 43 34 55 3 18 53 54 9 72 70 82 61 76 96 36 13 15 8 89
25 75 88 61 98 59 22 42 21 61 14 81 82 20 48 98 64 42 29 51 49 64 10 44 36 10 59 48 43 59 91 51 22 81 78 97 96 73 2 23 78 47 30 50 2 74 9 52 82 6
6 11 49 17 0 1 54 3 56 53 23 8 92 40 39 93 76 78 61 90 17 49 44 31 56 1 57 45 98 18 67 14 46 63 24 75 75 19 54 24 69 96 79 63 23 97 3 37 89 98
94 23 34 2 29 31 92 0 16 26 86 35 69 30 85 83 45 42 75 80 1 2 55 19 91 35 43 68 22 30 94 99 92 81 41 31 32 52 33 69 29 89 19 83 89 43 49 39 29 19
16 27 91 27 92 57 53 15 94 86 0 46 33 34 94 95 4 48 89 3 5 31 82 59 88 18 70 76 61 7 61 71 24 83 49 91 37 15 71 88 30 75 81 65 48 3 40 75 8 7
23 43 19 63 94 75 21 88 19 62 35 86 63 9 91 61 47 37 32 92 95 24 44 60 48 48 2 24 82 48 66 55 33 1 66 6 33 97 29 39 7 41 43 19 65 10 62 98 16 80
59 35 51 74 36 81 10 72 42 57 37 7 93 7 58 10 84 49 5 39 10 97 83 19 83 65 92 76 68 90 83 85 35 0 55 71 26 23 55 85 5 64 34 3 67 91 51 5 80 16
59 65 4 81 16 80 63 34 28 61 7 23 85 37 41 1 98 72 99 38 31 36 97 12 99 20 0 87 10 90 37 1 25 29 49 6 57 54 25 40 95 12 8 90 60 10 6 39 71 57
78 40 94 99 78 2 62 20 64 22 24 23 34 40 43 47 16 56 20 4 47 88 79 33 48 77 91 35 22 83 42 87 87 82 46 89 68 35 49 0 61 45 86 66 57 35 76 51 67 51
72 98 85 61 81 29 77 84 11 10 68 87 74 95 99 93 31 55 73 42 9 20 86 18 82 30 43 70 67 39 61 91 15 53 68 93 51 1 55 74 73 20 51 51 28 19 64 22 82 10
22 38 11 62 55 31 59 12 73 64 20 27 40 88 61 82 19 21 5 53 8 76 95 96 62 69 54 18 29 97 82 15 87 59 78 81 55 68 69 54 54 32 58 78 69 95 73 36 78 77
57 8 69 52 50 79 72 43 11 40 42 77 92 55 31 80 94 27 67 19 95 61 47 45 60 21 3 77 85 34 38 86 16 96 16 29 51 88 85 29 10 68 24 8 6 2 95 55 6 47
50 57 81 20 38 7 86 34 86 45 53 76 54 56 9 32 4 74 49 41 37 64 19 31 35 77 41 11 67 9 41 23 1 14 43 73 50 80 97 15 66 31 12 89 96 8 29 43 73 18
24 82 88 97 75 67 65 81 93 88 49 42 91 97 20 57 99 53 79 82 13 77 93 15 51 59 73 26 90 43 28 12 17 32 71 24 68 54 63 88 11 47 1 15 52 88 13 64 32 75
35 35 17 28 54 58 34 70 32 18 32 18 50 28 23 1 10 56 51 25 37 20 29 19 93 65 10 69 54 39 18 62 8 13 30 22 91 47 24 29 42 27 21 30 76 52 83 84 89 7
43 1 72 65 52 9 86 85 81 13 94 13 48 86 79 28 10 90 84 7 85 91 96 80 20 96 96 61 26 46 8 72 22 49 90 18 31 54 77 71 1 27 9 83 50 37 36 52 43 20
25 15 27 9 8 49 7 32 24 52 92 9 74 51 13 87 12 12 98 98 66 78 91 1 66 85 41 77 94 64 63 69 2 74 46 46 3 15 11 63 67 98 90 94 24 93 86 55 54 75
    "
}
