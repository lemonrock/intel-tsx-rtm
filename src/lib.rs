// This file is part of intel-tsx-rtm. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT. No part of intel-tsx-rtm, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-tsx-rtm. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![warn(missing_docs)]


//! This crates provides a simple set of wrappers around Intel's TSX RTM instructions and associated intrinsics.
//! It needs a C compiler to create a small shim.
//! This is important because Rust's compiler does not like code with multiple returns.
//! It does not depend on your compiler having the necessary headers (`<immintrin.h>`), and so can work with older compilers and other Operating Systems.
//! It uses third-party self-modifying code (Andi Kleen's `tsx-tools`) to provide runtime detection of CPUs without TSX and fallback to non-hardware paths.
//!


#[macro_use] extern crate bitflags;
#[macro_use] extern crate rust_c;


use ::std::mem::transmute;


include!("TransactionResult.rs");


c!
{
	#include <rtm.h>
	#include <stdint.h>
	
	#[inline(always)]
	fn hsx_xtest() -> i32 as "int"
	{
		return _xtest();
	}
	
	#[inline(always)]
	fn hsx_transaction(transaction_callback_function_pointer: extern fn() -> u8 as "void *") -> u32 as "unsigned"
	{
		int (*transaction_callback)() = transaction_callback_function_pointer;
		
		unsigned status = _xbegin();
		
		if (status != _XBEGIN_STARTED)
		{
			return status;
		}
		else
		{
			uint8_t result = transaction_callback();
			
			// This odd-looking code is because `_xabort(status)` has to take a constant value for `status`.
			switch(result)
			{
				case 0:
					_xend();
					return 0;
					
				case 1:
					_xabort(1);
					break;
		
				case 2:
					_xabort(2);
					break;
		
				case 3:
					_xabort(3);
					break;
		
				case 4:
					_xabort(4);
					break;
		
				case 5:
					_xabort(5);
					break;
		
				case 6:
					_xabort(6);
					break;
		
				case 7:
					_xabort(7);
					break;
		
				case 8:
					_xabort(8);
					break;
		
				case 9:
					_xabort(9);
					break;
		
				case 10:
					_xabort(10);
					break;
		
				case 11:
					_xabort(11);
					break;
		
				case 12:
					_xabort(12);
					break;
		
				case 13:
					_xabort(13);
					break;
		
				case 14:
					_xabort(14);
					break;
		
				case 15:
					_xabort(15);
					break;
		
				case 16:
					_xabort(16);
					break;
		
				case 17:
					_xabort(17);
					break;
		
				case 18:
					_xabort(18);
					break;
		
				case 19:
					_xabort(19);
					break;
		
				case 20:
					_xabort(20);
					break;
		
				case 21:
					_xabort(21);
					break;
		
				case 22:
					_xabort(22);
					break;
		
				case 23:
					_xabort(23);
					break;
		
				case 24:
					_xabort(24);
					break;
		
				case 25:
					_xabort(25);
					break;
		
				case 26:
					_xabort(26);
					break;
		
				case 27:
					_xabort(27);
					break;
		
				case 28:
					_xabort(28);
					break;
		
				case 29:
					_xabort(29);
					break;
		
				case 30:
					_xabort(30);
					break;
		
				case 31:
					_xabort(31);
					break;
		
				case 32:
					_xabort(32);
					break;
		
				case 33:
					_xabort(33);
					break;
		
				case 34:
					_xabort(34);
					break;
		
				case 35:
					_xabort(35);
					break;
		
				case 36:
					_xabort(36);
					break;
		
				case 37:
					_xabort(37);
					break;
		
				case 38:
					_xabort(38);
					break;
		
				case 39:
					_xabort(39);
					break;
		
				case 40:
					_xabort(40);
					break;
		
				case 41:
					_xabort(41);
					break;
		
				case 42:
					_xabort(42);
					break;
		
				case 43:
					_xabort(43);
					break;
		
				case 44:
					_xabort(44);
					break;
		
				case 45:
					_xabort(45);
					break;
		
				case 46:
					_xabort(46);
					break;
		
				case 47:
					_xabort(47);
					break;
		
				case 48:
					_xabort(48);
					break;
		
				case 49:
					_xabort(49);
					break;
		
				case 50:
					_xabort(50);
					break;
		
				case 51:
					_xabort(51);
					break;
		
				case 52:
					_xabort(52);
					break;
		
				case 53:
					_xabort(53);
					break;
		
				case 54:
					_xabort(54);
					break;
		
				case 55:
					_xabort(55);
					break;
		
				case 56:
					_xabort(56);
					break;
		
				case 57:
					_xabort(57);
					break;
		
				case 58:
					_xabort(58);
					break;
		
				case 59:
					_xabort(59);
					break;
		
				case 60:
					_xabort(60);
					break;
		
				case 61:
					_xabort(61);
					break;
		
				case 62:
					_xabort(62);
					break;
		
				case 63:
					_xabort(63);
					break;
		
				case 64:
					_xabort(64);
					break;
		
				case 65:
					_xabort(65);
					break;
		
				case 66:
					_xabort(66);
					break;
		
				case 67:
					_xabort(67);
					break;
		
				case 68:
					_xabort(68);
					break;
		
				case 69:
					_xabort(69);
					break;
		
				case 70:
					_xabort(70);
					break;
		
				case 71:
					_xabort(71);
					break;
		
				case 72:
					_xabort(72);
					break;
		
				case 73:
					_xabort(73);
					break;
		
				case 74:
					_xabort(74);
					break;
		
				case 75:
					_xabort(75);
					break;
		
				case 76:
					_xabort(76);
					break;
		
				case 77:
					_xabort(77);
					break;
		
				case 78:
					_xabort(78);
					break;
		
				case 79:
					_xabort(79);
					break;
		
				case 80:
					_xabort(80);
					break;
		
				case 81:
					_xabort(81);
					break;
		
				case 82:
					_xabort(82);
					break;
		
				case 83:
					_xabort(83);
					break;
		
				case 84:
					_xabort(84);
					break;
		
				case 85:
					_xabort(85);
					break;
		
				case 86:
					_xabort(86);
					break;
		
				case 87:
					_xabort(87);
					break;
		
				case 88:
					_xabort(88);
					break;
		
				case 89:
					_xabort(89);
					break;
		
				case 90:
					_xabort(90);
					break;
		
				case 91:
					_xabort(91);
					break;
		
				case 92:
					_xabort(92);
					break;
		
				case 93:
					_xabort(93);
					break;
		
				case 94:
					_xabort(94);
					break;
		
				case 95:
					_xabort(95);
					break;
		
				case 96:
					_xabort(96);
					break;
		
				case 97:
					_xabort(97);
					break;
		
				case 98:
					_xabort(98);
					break;
		
				case 99:
					_xabort(99);
					break;
		
				case 100:
					_xabort(100);
					break;
		
				case 101:
					_xabort(101);
					break;
		
				case 102:
					_xabort(102);
					break;
		
				case 103:
					_xabort(103);
					break;
		
				case 104:
					_xabort(104);
					break;
		
				case 105:
					_xabort(105);
					break;
		
				case 106:
					_xabort(106);
					break;
		
				case 107:
					_xabort(107);
					break;
		
				case 108:
					_xabort(108);
					break;
		
				case 109:
					_xabort(109);
					break;
		
				case 110:
					_xabort(110);
					break;
		
				case 111:
					_xabort(111);
					break;
		
				case 112:
					_xabort(112);
					break;
		
				case 113:
					_xabort(113);
					break;
		
				case 114:
					_xabort(114);
					break;
		
				case 115:
					_xabort(115);
					break;
		
				case 116:
					_xabort(116);
					break;
		
				case 117:
					_xabort(117);
					break;
		
				case 118:
					_xabort(118);
					break;
		
				case 119:
					_xabort(119);
					break;
		
				case 120:
					_xabort(120);
					break;
		
				case 121:
					_xabort(121);
					break;
		
				case 122:
					_xabort(122);
					break;
		
				case 123:
					_xabort(123);
					break;
		
				case 124:
					_xabort(124);
					break;
		
				case 125:
					_xabort(125);
					break;
		
				case 126:
					_xabort(126);
					break;
		
				case 127:
					_xabort(127);
					break;
		
				case 128:
					_xabort(128);
					break;
		
				case 129:
					_xabort(129);
					break;
		
				case 130:
					_xabort(130);
					break;
		
				case 131:
					_xabort(131);
					break;
		
				case 132:
					_xabort(132);
					break;
		
				case 133:
					_xabort(133);
					break;
		
				case 134:
					_xabort(134);
					break;
		
				case 135:
					_xabort(135);
					break;
		
				case 136:
					_xabort(136);
					break;
		
				case 137:
					_xabort(137);
					break;
		
				case 138:
					_xabort(138);
					break;
		
				case 139:
					_xabort(139);
					break;
		
				case 140:
					_xabort(140);
					break;
		
				case 141:
					_xabort(141);
					break;
		
				case 142:
					_xabort(142);
					break;
		
				case 143:
					_xabort(143);
					break;
		
				case 144:
					_xabort(144);
					break;
		
				case 145:
					_xabort(145);
					break;
		
				case 146:
					_xabort(146);
					break;
		
				case 147:
					_xabort(147);
					break;
		
				case 148:
					_xabort(148);
					break;
		
				case 149:
					_xabort(149);
					break;
		
				case 150:
					_xabort(150);
					break;
		
				case 151:
					_xabort(151);
					break;
		
				case 152:
					_xabort(152);
					break;
		
				case 153:
					_xabort(153);
					break;
		
				case 154:
					_xabort(154);
					break;
		
				case 155:
					_xabort(155);
					break;
		
				case 156:
					_xabort(156);
					break;
		
				case 157:
					_xabort(157);
					break;
		
				case 158:
					_xabort(158);
					break;
		
				case 159:
					_xabort(159);
					break;
		
				case 160:
					_xabort(160);
					break;
		
				case 161:
					_xabort(161);
					break;
		
				case 162:
					_xabort(162);
					break;
		
				case 163:
					_xabort(163);
					break;
		
				case 164:
					_xabort(164);
					break;
		
				case 165:
					_xabort(165);
					break;
		
				case 166:
					_xabort(166);
					break;
		
				case 167:
					_xabort(167);
					break;
		
				case 168:
					_xabort(168);
					break;
		
				case 169:
					_xabort(169);
					break;
		
				case 170:
					_xabort(170);
					break;
		
				case 171:
					_xabort(171);
					break;
		
				case 172:
					_xabort(172);
					break;
		
				case 173:
					_xabort(173);
					break;
		
				case 174:
					_xabort(174);
					break;
		
				case 175:
					_xabort(175);
					break;
		
				case 176:
					_xabort(176);
					break;
		
				case 177:
					_xabort(177);
					break;
		
				case 178:
					_xabort(178);
					break;
		
				case 179:
					_xabort(179);
					break;
		
				case 180:
					_xabort(180);
					break;
		
				case 181:
					_xabort(181);
					break;
		
				case 182:
					_xabort(182);
					break;
		
				case 183:
					_xabort(183);
					break;
		
				case 184:
					_xabort(184);
					break;
		
				case 185:
					_xabort(185);
					break;
		
				case 186:
					_xabort(186);
					break;
		
				case 187:
					_xabort(187);
					break;
		
				case 188:
					_xabort(188);
					break;
		
				case 189:
					_xabort(189);
					break;
		
				case 190:
					_xabort(190);
					break;
		
				case 191:
					_xabort(191);
					break;
		
				case 192:
					_xabort(192);
					break;
		
				case 193:
					_xabort(193);
					break;
		
				case 194:
					_xabort(194);
					break;
		
				case 195:
					_xabort(195);
					break;
		
				case 196:
					_xabort(196);
					break;
		
				case 197:
					_xabort(197);
					break;
		
				case 198:
					_xabort(198);
					break;
		
				case 199:
					_xabort(199);
					break;
		
				case 200:
					_xabort(200);
					break;
		
				case 201:
					_xabort(201);
					break;
		
				case 202:
					_xabort(202);
					break;
		
				case 203:
					_xabort(203);
					break;
		
				case 204:
					_xabort(204);
					break;
		
				case 205:
					_xabort(205);
					break;
		
				case 206:
					_xabort(206);
					break;
		
				case 207:
					_xabort(207);
					break;
		
				case 208:
					_xabort(208);
					break;
		
				case 209:
					_xabort(209);
					break;
		
				case 210:
					_xabort(210);
					break;
		
				case 211:
					_xabort(211);
					break;
		
				case 212:
					_xabort(212);
					break;
		
				case 213:
					_xabort(213);
					break;
		
				case 214:
					_xabort(214);
					break;
		
				case 215:
					_xabort(215);
					break;
		
				case 216:
					_xabort(216);
					break;
		
				case 217:
					_xabort(217);
					break;
		
				case 218:
					_xabort(218);
					break;
		
				case 219:
					_xabort(219);
					break;
		
				case 220:
					_xabort(220);
					break;
		
				case 221:
					_xabort(221);
					break;
		
				case 222:
					_xabort(222);
					break;
		
				case 223:
					_xabort(223);
					break;
		
				case 224:
					_xabort(224);
					break;
		
				case 225:
					_xabort(225);
					break;
		
				case 226:
					_xabort(226);
					break;
		
				case 227:
					_xabort(227);
					break;
		
				case 228:
					_xabort(228);
					break;
		
				case 229:
					_xabort(229);
					break;
		
				case 230:
					_xabort(230);
					break;
		
				case 231:
					_xabort(231);
					break;
		
				case 232:
					_xabort(232);
					break;
		
				case 233:
					_xabort(233);
					break;
		
				case 234:
					_xabort(234);
					break;
		
				case 235:
					_xabort(235);
					break;
		
				case 236:
					_xabort(236);
					break;
		
				case 237:
					_xabort(237);
					break;
		
				case 238:
					_xabort(238);
					break;
		
				case 239:
					_xabort(239);
					break;
		
				case 240:
					_xabort(240);
					break;
		
				case 241:
					_xabort(241);
					break;
		
				case 242:
					_xabort(242);
					break;
		
				case 243:
					_xabort(243);
					break;
		
				case 244:
					_xabort(244);
					break;
		
				case 245:
					_xabort(245);
					break;
		
				case 246:
					_xabort(246);
					break;
		
				case 247:
					_xabort(247);
					break;
		
				case 248:
					_xabort(248);
					break;
		
				case 249:
					_xabort(249);
					break;
		
				case 250:
					_xabort(250);
					break;
		
				case 251:
					_xabort(251);
					break;
		
				case 252:
					_xabort(252);
					break;
		
				case 253:
					_xabort(253);
					break;
		
				case 254:
					_xabort(254);
					break;
		
				case 255:
					_xabort(255);
					break;
			}
			
			// Impossible to reach
			return 0;
		}
	}
}
