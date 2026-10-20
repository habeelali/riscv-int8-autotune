
hardware/esp32c3-v4/build/riscv_int8_generalize.elf:     file format elf32-littleriscv


Disassembly of section .iram0.text:

Disassembly of section .flash.text:

42028e18 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE>:
42028e18:	81010113          	addi	sp,sp,-2032
42028e1c:	7e112623          	sw	ra,2028(sp)
42028e20:	7e812423          	sw	s0,2024(sp)
42028e24:	7e912223          	sw	s1,2020(sp)
42028e28:	7f212023          	sw	s2,2016(sp)
42028e2c:	7d312e23          	sw	s3,2012(sp)
42028e30:	7d412c23          	sw	s4,2008(sp)
42028e34:	7d512a23          	sw	s5,2004(sp)
42028e38:	7d612823          	sw	s6,2000(sp)
42028e3c:	7d712623          	sw	s7,1996(sp)
42028e40:	7d812423          	sw	s8,1992(sp)
42028e44:	7d912223          	sw	s9,1988(sp)
42028e48:	7da12023          	sw	s10,1984(sp)
42028e4c:	7bb12e23          	sw	s11,1980(sp)
42028e50:	80010113          	addi	sp,sp,-2048
42028e54:	d5010113          	addi	sp,sp,-688
42028e58:	8c3e                	mv	s8,a5
42028e5a:	ce3a                	sw	a4,28(sp)
42028e5c:	cc36                	sw	a3,24(sp)
42028e5e:	ca32                	sw	a2,20(sp)
42028e60:	842e                	mv	s0,a1
42028e62:	d22a                	sw	a0,36(sp)
42028e64:	40000bb7          	lui	s7,0x40000
42028e68:	4525                	li	a0,9
42028e6a:	00951613          	slli	a2,a0,0x9
42028e6e:	10e8                	addi	a0,sp,108
42028e70:	4581                	li	a1,0
42028e72:	de32                	sw	a2,60(sp)
42028e74:	fdfd7097          	auipc	ra,0xfdfd7
42028e78:	4e0080e7          	jalr	1248(ra) # 40000354 <memset>
42028e7c:	4701                	li	a4,0
42028e7e:	c822                	sw	s0,16(sp)
42028e80:	0405                	addi	s0,s0,1
42028e82:	c622                	sw	s0,12(sp)
42028e84:	451d                	li	a0,7
42028e86:	04000613          	li	a2,64
42028e8a:	45ed                	li	a1,27
42028e8c:	6685                	lui	a3,0x1
42028e8e:	059e                	slli	a1,a1,0x7
42028e90:	dc2e                	sw	a1,56(sp)
42028e92:	b4068593          	addi	a1,a3,-1216 # b40 <RvExcFrameSize+0xaac>
42028e96:	da2e                	sw	a1,52(sp)
42028e98:	c42a                	sw	a0,8(sp)
42028e9a:	d2ba                	sw	a4,100(sp)
42028e9c:	04000513          	li	a0,64
42028ea0:	42c71463          	bne	a4,a2,420292c8 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x4b0>
42028ea4:	d802                	sw	zero,48(sp)
42028ea6:	4c81                	li	s9,0
42028ea8:	5716                	lw	a4,100(sp)
42028eaa:	fc070613          	addi	a2,a4,-64
42028eae:	45fd                	li	a1,31
42028eb0:	00163513          	seqz	a0,a2
42028eb4:	d02a                	sw	a0,32(sp)
42028eb6:	4609                	li	a2,2
42028eb8:	4532                	lw	a0,12(sp)
42028eba:	d4aa                	sw	a0,104(sp)
42028ebc:	a015                	j	42028ee0 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xc8>
42028ebe:	5532                	lw	a0,44(sp)
42028ec0:	8caa                	mv	s9,a0
42028ec2:	0509                	addi	a0,a0,2
42028ec4:	56a2                	lw	a3,40(sp)
42028ec6:	fff68593          	addi	a1,a3,-1
42028eca:	5626                	lw	a2,104(sp)
42028ecc:	48060613          	addi	a2,a2,1152
42028ed0:	d4b2                	sw	a2,104(sp)
42028ed2:	5642                	lw	a2,48(sp)
42028ed4:	0605                	addi	a2,a2,1
42028ed6:	d832                	sw	a2,48(sp)
42028ed8:	862a                	mv	a2,a0
42028eda:	5716                	lw	a4,100(sp)
42028edc:	72068163          	beqz	a3,420295fe <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x7e6>
42028ee0:	24000513          	li	a0,576
42028ee4:	02ac86b3          	mul	a3,s9,a0
42028ee8:	87ae                	mv	a5,a1
42028eea:	04000513          	li	a0,64
42028eee:	b8000593          	li	a1,-1152
42028ef2:	7eb6f363          	bgeu	a3,a1,420296d8 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x8c0>
42028ef6:	d43e                	sw	a5,40(sp)
42028ef8:	d632                	sw	a2,44(sp)
42028efa:	fca702e3          	beq	a4,a0,42028ebe <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xa6>
42028efe:	03f00513          	li	a0,63
42028f02:	199568e3          	bltu	a0,s9,42029892 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xa7a>
42028f06:	4a81                	li	s5,0
42028f08:	48000513          	li	a0,1152
42028f0c:	55c2                	lw	a1,48(sp)
42028f0e:	02a585b3          	mul	a1,a1,a0
42028f12:	040c8513          	addi	a0,s9,64
42028f16:	ccaa                	sw	a0,88(sp)
42028f18:	080c8513          	addi	a0,s9,128
42028f1c:	c6aa                	sw	a0,76(sp)
42028f1e:	0c0c8513          	addi	a0,s9,192
42028f22:	c4aa                	sw	a0,72(sp)
42028f24:	002c9893          	slli	a7,s9,0x2
42028f28:	4542                	lw	a0,16(sp)
42028f2a:	95aa                	add	a1,a1,a0
42028f2c:	46d2                	lw	a3,20(sp)
42028f2e:	01168533          	add	a0,a3,a7
42028f32:	d0aa                	sw	a0,96(sp)
42028f34:	00488813          	addi	a6,a7,4
42028f38:	4662                	lw	a2,24(sp)
42028f3a:	01160533          	add	a0,a2,a7
42028f3e:	caaa                	sw	a0,84(sp)
42028f40:	4572                	lw	a0,28(sp)
42028f42:	98aa                	add	a7,a7,a0
42028f44:	c8c6                	sw	a7,80(sp)
42028f46:	48158d93          	addi	s11,a1,1153
42028f4a:	96c2                	add	a3,a3,a6
42028f4c:	ceb6                	sw	a3,92(sp)
42028f4e:	9642                	add	a2,a2,a6
42028f50:	c2b2                	sw	a2,68(sp)
42028f52:	9542                	add	a0,a0,a6
42028f54:	c0aa                	sw	a0,64(sp)
42028f56:	4991                	li	s3,4
42028f58:	4ec10d13          	addi	s10,sp,1260
42028f5c:	5a02                	lw	s4,32(sp)
42028f5e:	a829                	j	42028f78 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x160>
42028f60:	00b380a3          	sb	a1,1(t2)
42028f64:	8ace                	mv	s5,s3
42028f66:	0991                	addi	s3,s3,4
42028f68:	7ffd0513          	addi	a0,s10,2047
42028f6c:	001a7593          	andi	a1,s4,1
42028f70:	10150d13          	addi	s10,a0,257
42028f74:	4a05                	li	s4,1
42028f76:	f5a1                	bnez	a1,42028ebe <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xa6>
42028f78:	24000513          	li	a0,576
42028f7c:	02aa86b3          	mul	a3,s5,a0
42028f80:	24068713          	addi	a4,a3,576
42028f84:	dbf00513          	li	a0,-577
42028f88:	6cd56463          	bltu	a0,a3,42029650 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x838>
42028f8c:	5572                	lw	a0,60(sp)
42028f8e:	6ce56b63          	bltu	a0,a4,42029664 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x84c>
42028f92:	48068593          	addi	a1,a3,1152
42028f96:	6ee5e263          	bltu	a1,a4,4202967a <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x862>
42028f9a:	5562                	lw	a0,56(sp)
42028f9c:	6ed56863          	bltu	a0,a3,4202968c <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x874>
42028fa0:	5552                	lw	a0,52(sp)
42028fa2:	70d56163          	bltu	a0,a3,420296a4 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x88c>
42028fa6:	45a5                	li	a1,9
42028fa8:	05a2                	slli	a1,a1,0x8
42028faa:	70d5e963          	bltu	a1,a3,420296bc <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x8a4>
42028fae:	5506                	lw	a0,96(sp)
42028fb0:	00052283          	lw	t0,0(a0)
42028fb4:	4576                	lw	a0,92(sp)
42028fb6:	00052883          	lw	a7,0(a0)
42028fba:	85ea                	mv	a1,s10
42028fbc:	5726                	lw	a4,104(sp)
42028fbe:	8e96                	mv	t4,t0
42028fc0:	8916                	mv	s2,t0
42028fc2:	8816                	mv	a6,t0
42028fc4:	80c6                	mv	ra,a7
42028fc6:	8fc6                	mv	t6,a7
42028fc8:	8f46                	mv	t5,a7
42028fca:	b8058503          	lb	a0,-1152(a1)
42028fce:	24058483          	lb	s1,576(a1)
42028fd2:	fff70783          	lb	a5,-1(a4)
42028fd6:	00058403          	lb	s0,0(a1)
42028fda:	dc058603          	lb	a2,-576(a1)
42028fde:	00070683          	lb	a3,0(a4)
42028fe2:	02978333          	mul	t1,a5,s1
42028fe6:	028783b3          	mul	t2,a5,s0
42028fea:	02c78e33          	mul	t3,a5,a2
42028fee:	02a787b3          	mul	a5,a5,a0
42028ff2:	02a68533          	mul	a0,a3,a0
42028ff6:	02c68633          	mul	a2,a3,a2
42028ffa:	02868433          	mul	s0,a3,s0
42028ffe:	029686b3          	mul	a3,a3,s1
42029002:	981a                	add	a6,a6,t1
42029004:	991e                	add	s2,s2,t2
42029006:	0709                	addi	a4,a4,2
42029008:	9ef2                	add	t4,t4,t3
4202900a:	92be                	add	t0,t0,a5
4202900c:	98aa                	add	a7,a7,a0
4202900e:	90b2                	add	ra,ra,a2
42029010:	9fa2                	add	t6,t6,s0
42029012:	9f36                	add	t5,t5,a3
42029014:	0585                	addi	a1,a1,1
42029016:	fbb71ae3          	bne	a4,s11,42028fca <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x1b2>
4202901a:	5396                	lw	t2,100(sp)
4202901c:	93d6                	add	t2,t2,s5
4202901e:	039a                	slli	t2,t2,0x6
42029020:	0193eab3          	or	s5,t2,s9
42029024:	00cad593          	srli	a1,s5,0xc
42029028:	040591e3          	bnez	a1,4202986a <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xa52>
4202902c:	4546                	lw	a0,80(sp)
4202902e:	410c                	lw	a1,0(a0)
42029030:	4556                	lw	a0,84(sp)
42029032:	00052e03          	lw	t3,0(a0)
42029036:	fff5c713          	not	a4,a1
4202903a:	40b005b3          	neg	a1,a1
4202903e:	025e1b33          	mulh	s6,t3,t0
42029042:	025e06b3          	mul	a3,t3,t0
42029046:	4505                	li	a0,1
42029048:	00e51333          	sll	t1,a0,a4
4202904c:	01f5f293          	andi	t0,a1,31
42029050:	017685b3          	add	a1,a3,s7
42029054:	00d5b6b3          	sltu	a3,a1,a3
42029058:	81fd                	srli	a1,a1,0x1f
4202905a:	96da                	add	a3,a3,s6
4202905c:	0686                	slli	a3,a3,0x1
4202905e:	8dd5                	or	a1,a1,a3
42029060:	86fd                	srai	a3,a3,0x1f
42029062:	959a                	add	a1,a1,t1
42029064:	95b6                	add	a1,a1,a3
42029066:	4055d5b3          	sra	a1,a1,t0
4202906a:	15f9                	addi	a1,a1,-2
4202906c:	07f00693          	li	a3,127
42029070:	02d5d063          	bge	a1,a3,42029090 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x278>
42029074:	f8000693          	li	a3,-128
42029078:	02b6d263          	bge	a3,a1,4202909c <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x284>
4202907c:	9ae2                	add	s5,s5,s8
4202907e:	4566                	lw	a0,88(sp)
42029080:	00a3eb33          	or	s6,t2,a0
42029084:	00cb5693          	srli	a3,s6,0xc
42029088:	00ba8023          	sb	a1,0(s5)
4202908c:	c685                	beqz	a3,420290b4 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x29c>
4202908e:	af5d                	j	42029844 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xa2c>
42029090:	07f00593          	li	a1,127
42029094:	f8000693          	li	a3,-128
42029098:	feb6c2e3          	blt	a3,a1,4202907c <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x264>
4202909c:	f8000593          	li	a1,-128
420290a0:	9ae2                	add	s5,s5,s8
420290a2:	4566                	lw	a0,88(sp)
420290a4:	00a3eb33          	or	s6,t2,a0
420290a8:	00cb5693          	srli	a3,s6,0xc
420290ac:	00ba8023          	sb	a1,0(s5)
420290b0:	78069a63          	bnez	a3,42029844 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xa2c>
420290b4:	03de15b3          	mulh	a1,t3,t4
420290b8:	03de06b3          	mul	a3,t3,t4
420290bc:	01768733          	add	a4,a3,s7
420290c0:	00d736b3          	sltu	a3,a4,a3
420290c4:	837d                	srli	a4,a4,0x1f
420290c6:	95b6                	add	a1,a1,a3
420290c8:	0586                	slli	a1,a1,0x1
420290ca:	8f4d                	or	a4,a4,a1
420290cc:	85fd                	srai	a1,a1,0x1f
420290ce:	971a                	add	a4,a4,t1
420290d0:	95ba                	add	a1,a1,a4
420290d2:	4055d5b3          	sra	a1,a1,t0
420290d6:	15f9                	addi	a1,a1,-2
420290d8:	07f00693          	li	a3,127
420290dc:	02d5d063          	bge	a1,a3,420290fc <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x2e4>
420290e0:	f8000693          	li	a3,-128
420290e4:	02b6d263          	bge	a3,a1,42029108 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x2f0>
420290e8:	9b62                	add	s6,s6,s8
420290ea:	4536                	lw	a0,76(sp)
420290ec:	00a3e6b3          	or	a3,t2,a0
420290f0:	00c6d713          	srli	a4,a3,0xc
420290f4:	00bb0023          	sb	a1,0(s6)
420290f8:	c705                	beqz	a4,42029120 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x308>
420290fa:	a751                	j	4202987e <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xa66>
420290fc:	07f00593          	li	a1,127
42029100:	f8000693          	li	a3,-128
42029104:	feb6c2e3          	blt	a3,a1,420290e8 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x2d0>
42029108:	f8000593          	li	a1,-128
4202910c:	9b62                	add	s6,s6,s8
4202910e:	4536                	lw	a0,76(sp)
42029110:	00a3e6b3          	or	a3,t2,a0
42029114:	00c6d713          	srli	a4,a3,0xc
42029118:	00bb0023          	sb	a1,0(s6)
4202911c:	76071163          	bnez	a4,4202987e <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xa66>
42029120:	032e15b3          	mulh	a1,t3,s2
42029124:	032e0633          	mul	a2,t3,s2
42029128:	01760733          	add	a4,a2,s7
4202912c:	00c73633          	sltu	a2,a4,a2
42029130:	837d                	srli	a4,a4,0x1f
42029132:	95b2                	add	a1,a1,a2
42029134:	0586                	slli	a1,a1,0x1
42029136:	8f4d                	or	a4,a4,a1
42029138:	85fd                	srai	a1,a1,0x1f
4202913a:	971a                	add	a4,a4,t1
4202913c:	95ba                	add	a1,a1,a4
4202913e:	4055d5b3          	sra	a1,a1,t0
42029142:	15f9                	addi	a1,a1,-2
42029144:	07f00613          	li	a2,127
42029148:	02c5d163          	bge	a1,a2,4202916a <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x352>
4202914c:	f8000613          	li	a2,-128
42029150:	02b65363          	bge	a2,a1,42029176 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x35e>
42029154:	00dc0eb3          	add	t4,s8,a3
42029158:	4526                	lw	a0,72(sp)
4202915a:	00a3e533          	or	a0,t2,a0
4202915e:	00c55713          	srli	a4,a0,0xc
42029162:	00be8023          	sb	a1,0(t4)
42029166:	c70d                	beqz	a4,42029190 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x378>
42029168:	adc5                	j	42029858 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xa40>
4202916a:	07f00593          	li	a1,127
4202916e:	f8000613          	li	a2,-128
42029172:	feb641e3          	blt	a2,a1,42029154 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x33c>
42029176:	f8000593          	li	a1,-128
4202917a:	00dc0eb3          	add	t4,s8,a3
4202917e:	4526                	lw	a0,72(sp)
42029180:	00a3e533          	or	a0,t2,a0
42029184:	00c55713          	srli	a4,a0,0xc
42029188:	00be8023          	sb	a1,0(t4)
4202918c:	6c071663          	bnez	a4,42029858 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xa40>
42029190:	030e15b3          	mulh	a1,t3,a6
42029194:	030e0733          	mul	a4,t3,a6
42029198:	01770433          	add	s0,a4,s7
4202919c:	00e43733          	sltu	a4,s0,a4
420291a0:	807d                	srli	s0,s0,0x1f
420291a2:	95ba                	add	a1,a1,a4
420291a4:	0586                	slli	a1,a1,0x1
420291a6:	8c4d                	or	s0,s0,a1
420291a8:	85fd                	srai	a1,a1,0x1f
420291aa:	9322                	add	t1,t1,s0
420291ac:	959a                	add	a1,a1,t1
420291ae:	4055d5b3          	sra	a1,a1,t0
420291b2:	15f9                	addi	a1,a1,-2
420291b4:	07f00313          	li	t1,127
420291b8:	0065c463          	blt	a1,t1,420291c0 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x3a8>
420291bc:	07f00593          	li	a1,127
420291c0:	f8000293          	li	t0,-128
420291c4:	00b2c463          	blt	t0,a1,420291cc <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x3b4>
420291c8:	f8000593          	li	a1,-128
420291cc:	4606                	lw	a2,64(sp)
420291ce:	4218                	lw	a4,0(a2)
420291d0:	4616                	lw	a2,68(sp)
420291d2:	4200                	lw	s0,0(a2)
420291d4:	4385                	li	t2,1
420291d6:	fff74613          	not	a2,a4
420291da:	031416b3          	mulh	a3,s0,a7
420291de:	031407b3          	mul	a5,s0,a7
420291e2:	00c39e33          	sll	t3,t2,a2
420291e6:	01778633          	add	a2,a5,s7
420291ea:	00f637b3          	sltu	a5,a2,a5
420291ee:	97b6                	add	a5,a5,a3
420291f0:	00ac03b3          	add	t2,s8,a0
420291f4:	40e006b3          	neg	a3,a4
420291f8:	827d                	srli	a2,a2,0x1f
420291fa:	0786                	slli	a5,a5,0x1
420291fc:	8e5d                	or	a2,a2,a5
420291fe:	87fd                	srai	a5,a5,0x1f
42029200:	9672                	add	a2,a2,t3
42029202:	963e                	add	a2,a2,a5
42029204:	40d65733          	sra	a4,a2,a3
42029208:	1779                	addi	a4,a4,-2
4202920a:	00b38023          	sb	a1,0(t2)
4202920e:	00674463          	blt	a4,t1,42029216 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x3fe>
42029212:	07f00713          	li	a4,127
42029216:	00e2c463          	blt	t0,a4,4202921e <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x406>
4202921a:	f8000713          	li	a4,-128
4202921e:	02141533          	mulh	a0,s0,ra
42029222:	021405b3          	mul	a1,s0,ra
42029226:	01758633          	add	a2,a1,s7
4202922a:	00b635b3          	sltu	a1,a2,a1
4202922e:	952e                	add	a0,a0,a1
42029230:	827d                	srli	a2,a2,0x1f
42029232:	0506                	slli	a0,a0,0x1
42029234:	8e49                	or	a2,a2,a0
42029236:	857d                	srai	a0,a0,0x1f
42029238:	9672                	add	a2,a2,t3
4202923a:	9532                	add	a0,a0,a2
4202923c:	40d555b3          	sra	a1,a0,a3
42029240:	15f9                	addi	a1,a1,-2
42029242:	00ea80a3          	sb	a4,1(s5)
42029246:	0065c463          	blt	a1,t1,4202924e <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x436>
4202924a:	07f00593          	li	a1,127
4202924e:	00b2c463          	blt	t0,a1,42029256 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x43e>
42029252:	f8000593          	li	a1,-128
42029256:	03f41533          	mulh	a0,s0,t6
4202925a:	03f40633          	mul	a2,s0,t6
4202925e:	01760733          	add	a4,a2,s7
42029262:	00c73633          	sltu	a2,a4,a2
42029266:	9532                	add	a0,a0,a2
42029268:	837d                	srli	a4,a4,0x1f
4202926a:	0506                	slli	a0,a0,0x1
4202926c:	8f49                	or	a4,a4,a0
4202926e:	857d                	srai	a0,a0,0x1f
42029270:	9772                	add	a4,a4,t3
42029272:	953a                	add	a0,a0,a4
42029274:	40d55733          	sra	a4,a0,a3
42029278:	1779                	addi	a4,a4,-2
4202927a:	00bb00a3          	sb	a1,1(s6)
4202927e:	00674463          	blt	a4,t1,42029286 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x46e>
42029282:	07f00713          	li	a4,127
42029286:	00e2c463          	blt	t0,a4,4202928e <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x476>
4202928a:	f8000713          	li	a4,-128
4202928e:	03e41533          	mulh	a0,s0,t5
42029292:	03e405b3          	mul	a1,s0,t5
42029296:	01758633          	add	a2,a1,s7
4202929a:	00b635b3          	sltu	a1,a2,a1
4202929e:	952e                	add	a0,a0,a1
420292a0:	827d                	srli	a2,a2,0x1f
420292a2:	0506                	slli	a0,a0,0x1
420292a4:	8e49                	or	a2,a2,a0
420292a6:	857d                	srai	a0,a0,0x1f
420292a8:	9672                	add	a2,a2,t3
420292aa:	9532                	add	a0,a0,a2
420292ac:	40d555b3          	sra	a1,a0,a3
420292b0:	15f9                	addi	a1,a1,-2
420292b2:	00ee80a3          	sb	a4,1(t4)
420292b6:	0065c463          	blt	a1,t1,420292be <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x4a6>
420292ba:	07f00593          	li	a1,127
420292be:	cab2c1e3          	blt	t0,a1,42028f60 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x148>
420292c2:	f8000593          	li	a1,-128
420292c6:	b969                	j	42028f60 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x148>
420292c8:	5d96                	lw	s11,100(sp)
420292ca:	41b50533          	sub	a0,a0,s11
420292ce:	45a1                	li	a1,8
420292d0:	00b56363          	bltu	a0,a1,420292d6 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x4be>
420292d4:	4521                	li	a0,8
420292d6:	4401                	li	s0,0
420292d8:	4681                	li	a3,0
420292da:	24000593          	li	a1,576
420292de:	02b507b3          	mul	a5,a0,a1
420292e2:	06c10a13          	addi	s4,sp,108
420292e6:	597d                	li	s2,-1
420292e8:	16c10c93          	addi	s9,sp,364
420292ec:	1568                	addi	a0,sp,684
420292ee:	a089                	j	42029330 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x518>
420292f0:	08000593          	li	a1,128
420292f4:	04000613          	li	a2,64
420292f8:	8536                	mv	a0,a3
420292fa:	fdfd7097          	auipc	ra,0xfdfd7
420292fe:	05a080e7          	jalr	90(ra) # 40000354 <memset>
42029302:	5406                	lw	s0,96(sp)
42029304:	47d6                	lw	a5,84(sp)
42029306:	dc078793          	addi	a5,a5,-576
4202930a:	240c8c93          	addi	s9,s9,576
4202930e:	0d85                	addi	s11,s11,1
42029310:	0905                	addi	s2,s2,1
42029312:	04040413          	addi	s0,s0,64
42029316:	2407b513          	sltiu	a0,a5,576
4202931a:	157d                	addi	a0,a0,-1
4202931c:	24057513          	andi	a0,a0,576
42029320:	4a66                	lw	s4,88(sp)
42029322:	9552                	add	a0,a0,s4
42029324:	46f6                	lw	a3,92(sp)
42029326:	0685                	addi	a3,a3,1
42029328:	24000593          	li	a1,576
4202932c:	b6b7ece3          	bltu	a5,a1,42028ea4 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x8c>
42029330:	872a                	mv	a4,a0
42029332:	003dda93          	srli	s5,s11,0x3
42029336:	f00c8513          	addi	a0,s9,-256
4202933a:	0076f593          	andi	a1,a3,7
4202933e:	009a9613          	slli	a2,s5,0x9
42029342:	fffa8493          	addi	s1,s5,-1
42029346:	059a                	slli	a1,a1,0x6
42029348:	c8ae                	sw	a1,80(sp)
4202934a:	d0a2                	sw	s0,96(sp)
4202934c:	00c409b3          	add	s3,s0,a2
42029350:	d4b2                	sw	a2,104(sp)
42029352:	e0060593          	addi	a1,a2,-512
42029356:	c6ae                	sw	a1,76(sp)
42029358:	009965b3          	or	a1,s2,s1
4202935c:	e4098b13          	addi	s6,s3,-448
42029360:	5612                	lw	a2,36(sp)
42029362:	01360d33          	add	s10,a2,s3
42029366:	e0098413          	addi	s0,s3,-512
4202936a:	4621                	li	a2,8
4202936c:	ceb6                	sw	a3,92(sp)
4202936e:	ccba                	sw	a4,88(sp)
42029370:	cabe                	sw	a5,84(sp)
42029372:	02c5f663          	bgeu	a1,a2,4202939e <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x586>
42029376:	dc098593          	addi	a1,s3,-576
4202937a:	3ab46a63          	bltu	s0,a1,4202972e <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x916>
4202937e:	6a05                	lui	s4,0x1
42029380:	3c8a6763          	bltu	s4,s0,4202974e <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x936>
42029384:	dc0d0593          	addi	a1,s10,-576
42029388:	04000613          	li	a2,64
4202938c:	fdfd7097          	auipc	ra,0xfdfd7
42029390:	fcc080e7          	jalr	-52(ra) # 40000358 <memcpy>
42029394:	f40c8513          	addi	a0,s9,-192
42029398:	028b7363          	bgeu	s6,s0,420293be <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x5a6>
4202939c:	ae89                	j	420296ee <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x8d6>
4202939e:	08000593          	li	a1,128
420293a2:	04000613          	li	a2,64
420293a6:	fdfd7097          	auipc	ra,0xfdfd7
420293aa:	fae080e7          	jalr	-82(ra) # 40000354 <memset>
420293ae:	4521                	li	a0,8
420293b0:	10a4fb63          	bgeu	s1,a0,420294c6 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x6ae>
420293b4:	040a0513          	addi	a0,s4,64 # 1040 <RvExcFrameSize+0xfac>
420293b8:	6a05                	lui	s4,0x1
420293ba:	328b6a63          	bltu	s6,s0,420296ee <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x8d6>
420293be:	356a6a63          	bltu	s4,s6,42029712 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x8fa>
420293c2:	e00d0593          	addi	a1,s10,-512
420293c6:	04000613          	li	a2,64
420293ca:	fdfd7097          	auipc	ra,0xfdfd7
420293ce:	f8e080e7          	jalr	-114(ra) # 40000358 <memcpy>
420293d2:	00290593          	addi	a1,s2,2
420293d6:	842e                	mv	s0,a1
420293d8:	8dc5                	or	a1,a1,s1
420293da:	f80c8513          	addi	a0,s9,-128
420293de:	4621                	li	a2,8
420293e0:	10c5f763          	bgeu	a1,a2,420294ee <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x6d6>
420293e4:	e8098593          	addi	a1,s3,-384
420293e8:	3965e163          	bltu	a1,s6,4202976a <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x952>
420293ec:	3cba6163          	bltu	s4,a1,420297ae <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x996>
420293f0:	04000493          	li	s1,64
420293f4:	e40d0593          	addi	a1,s10,-448
420293f8:	04000613          	li	a2,64
420293fc:	fdfd7097          	auipc	ra,0xfdfd7
42029400:	f5c080e7          	jalr	-164(ra) # 40000358 <memcpy>
42029404:	015965b3          	or	a1,s2,s5
42029408:	fc0c8513          	addi	a0,s9,-64
4202940c:	4621                	li	a2,8
4202940e:	8b22                	mv	s6,s0
42029410:	10c5e163          	bltu	a1,a2,42029512 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x6fa>
42029414:	08000593          	li	a1,128
42029418:	04000613          	li	a2,64
4202941c:	fdfd7097          	auipc	ra,0xfdfd7
42029420:	f38080e7          	jalr	-200(ra) # 40000354 <memset>
42029424:	5426                	lw	s0,104(sp)
42029426:	109df763          	bgeu	s11,s1,42029534 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x71c>
4202942a:	04098513          	addi	a0,s3,64
4202942e:	38aa6f63          	bltu	s4,a0,420297cc <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x9b4>
42029432:	04000613          	li	a2,64
42029436:	8566                	mv	a0,s9
42029438:	85ea                	mv	a1,s10
4202943a:	fdfd7097          	auipc	ra,0xfdfd7
4202943e:	f1e080e7          	jalr	-226(ra) # 40000358 <memcpy>
42029442:	015b6533          	or	a0,s6,s5
42029446:	040c8693          	addi	a3,s9,64
4202944a:	45a1                	li	a1,8
4202944c:	10b56463          	bltu	a0,a1,42029554 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x73c>
42029450:	08000593          	li	a1,128
42029454:	04000613          	li	a2,64
42029458:	8536                	mv	a0,a3
4202945a:	fdfd7097          	auipc	ra,0xfdfd7
4202945e:	efa080e7          	jalr	-262(ra) # 40000354 <memset>
42029462:	0a85                	addi	s5,s5,1
42029464:	20040493          	addi	s1,s0,512
42029468:	01596533          	or	a0,s2,s5
4202946c:	080c8693          	addi	a3,s9,128
42029470:	45a1                	li	a1,8
42029472:	10b57c63          	bgeu	a0,a1,4202958a <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x772>
42029476:	1c098713          	addi	a4,s3,448
4202947a:	20098513          	addi	a0,s3,512
4202947e:	36e56d63          	bltu	a0,a4,420297f8 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x9e0>
42029482:	34aa6563          	bltu	s4,a0,420297cc <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x9b4>
42029486:	1c0d0593          	addi	a1,s10,448
4202948a:	04000613          	li	a2,64
4202948e:	8536                	mv	a0,a3
42029490:	fdfd7097          	auipc	ra,0xfdfd7
42029494:	ec8080e7          	jalr	-312(ra) # 40000358 <memcpy>
42029498:	0c0c8693          	addi	a3,s9,192
4202949c:	03800513          	li	a0,56
420294a0:	10ade463          	bltu	s11,a0,420295a8 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x790>
420294a4:	08000593          	li	a1,128
420294a8:	04000613          	li	a2,64
420294ac:	8536                	mv	a0,a3
420294ae:	fdfd7097          	auipc	ra,0xfdfd7
420294b2:	ea6080e7          	jalr	-346(ra) # 40000354 <memset>
420294b6:	015b6533          	or	a0,s6,s5
420294ba:	100c8693          	addi	a3,s9,256
420294be:	45a1                	li	a1,8
420294c0:	e2b578e3          	bgeu	a0,a1,420292f0 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x4d8>
420294c4:	aa11                	j	420295d8 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x7c0>
420294c6:	f40c8513          	addi	a0,s9,-192
420294ca:	08000593          	li	a1,128
420294ce:	04000613          	li	a2,64
420294d2:	fdfd7097          	auipc	ra,0xfdfd7
420294d6:	e82080e7          	jalr	-382(ra) # 40000354 <memset>
420294da:	6a05                	lui	s4,0x1
420294dc:	00290593          	addi	a1,s2,2
420294e0:	842e                	mv	s0,a1
420294e2:	8dc5                	or	a1,a1,s1
420294e4:	f80c8513          	addi	a0,s9,-128
420294e8:	4621                	li	a2,8
420294ea:	eec5ede3          	bltu	a1,a2,420293e4 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x5cc>
420294ee:	04000493          	li	s1,64
420294f2:	08000593          	li	a1,128
420294f6:	04000613          	li	a2,64
420294fa:	fdfd7097          	auipc	ra,0xfdfd7
420294fe:	e5a080e7          	jalr	-422(ra) # 40000354 <memset>
42029502:	015965b3          	or	a1,s2,s5
42029506:	fc0c8513          	addi	a0,s9,-64
4202950a:	4621                	li	a2,8
4202950c:	8b22                	mv	s6,s0
4202950e:	f0c5f3e3          	bgeu	a1,a2,42029414 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x5fc>
42029512:	fc098593          	addi	a1,s3,-64
42029516:	26b9ec63          	bltu	s3,a1,4202978e <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x976>
4202951a:	2b3a6863          	bltu	s4,s3,420297ca <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x9b2>
4202951e:	fc0d0593          	addi	a1,s10,-64
42029522:	04000613          	li	a2,64
42029526:	fdfd7097          	auipc	ra,0xfdfd7
4202952a:	e32080e7          	jalr	-462(ra) # 40000358 <memcpy>
4202952e:	5426                	lw	s0,104(sp)
42029530:	ee9dede3          	bltu	s11,s1,4202942a <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x612>
42029534:	08000593          	li	a1,128
42029538:	04000613          	li	a2,64
4202953c:	8566                	mv	a0,s9
4202953e:	fdfd7097          	auipc	ra,0xfdfd7
42029542:	e16080e7          	jalr	-490(ra) # 40000354 <memset>
42029546:	015b6533          	or	a0,s6,s5
4202954a:	040c8693          	addi	a3,s9,64
4202954e:	45a1                	li	a1,8
42029550:	f0b570e3          	bgeu	a0,a1,42029450 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x638>
42029554:	04098713          	addi	a4,s3,64
42029558:	08098513          	addi	a0,s3,128
4202955c:	28e56163          	bltu	a0,a4,420297de <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x9c6>
42029560:	26aa6663          	bltu	s4,a0,420297cc <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x9b4>
42029564:	040d0593          	addi	a1,s10,64
42029568:	04000613          	li	a2,64
4202956c:	8536                	mv	a0,a3
4202956e:	fdfd7097          	auipc	ra,0xfdfd7
42029572:	dea080e7          	jalr	-534(ra) # 40000358 <memcpy>
42029576:	0a85                	addi	s5,s5,1
42029578:	20040493          	addi	s1,s0,512
4202957c:	01596533          	or	a0,s2,s5
42029580:	080c8693          	addi	a3,s9,128
42029584:	45a1                	li	a1,8
42029586:	eeb568e3          	bltu	a0,a1,42029476 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x65e>
4202958a:	08000593          	li	a1,128
4202958e:	04000613          	li	a2,64
42029592:	8536                	mv	a0,a3
42029594:	fdfd7097          	auipc	ra,0xfdfd7
42029598:	dc0080e7          	jalr	-576(ra) # 40000354 <memset>
4202959c:	0c0c8693          	addi	a3,s9,192
420295a0:	03800513          	li	a0,56
420295a4:	f0adf0e3          	bgeu	s11,a0,420294a4 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x68c>
420295a8:	1c040513          	addi	a0,s0,448
420295ac:	26a4e163          	bltu	s1,a0,4202980e <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x9f6>
420295b0:	24098513          	addi	a0,s3,576
420295b4:	20aa6c63          	bltu	s4,a0,420297cc <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x9b4>
420295b8:	200d0593          	addi	a1,s10,512
420295bc:	04000613          	li	a2,64
420295c0:	8536                	mv	a0,a3
420295c2:	fdfd7097          	auipc	ra,0xfdfd7
420295c6:	d96080e7          	jalr	-618(ra) # 40000358 <memcpy>
420295ca:	015b6533          	or	a0,s6,s5
420295ce:	100c8693          	addi	a3,s9,256
420295d2:	45a1                	li	a1,8
420295d4:	d0b57ee3          	bgeu	a0,a1,420292f0 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x4d8>
420295d8:	24098713          	addi	a4,s3,576
420295dc:	28098513          	addi	a0,s3,640
420295e0:	24e56563          	bltu	a0,a4,4202982a <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0xa12>
420295e4:	5406                	lw	s0,96(sp)
420295e6:	1eaa6363          	bltu	s4,a0,420297cc <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x9b4>
420295ea:	240d0593          	addi	a1,s10,576
420295ee:	04000613          	li	a2,64
420295f2:	8536                	mv	a0,a3
420295f4:	fdfd7097          	auipc	ra,0xfdfd7
420295f8:	d64080e7          	jalr	-668(ra) # 40000358 <memcpy>
420295fc:	b321                	j	42029304 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x4ec>
420295fe:	45a2                	lw	a1,8(sp)
42029600:	fff58513          	addi	a0,a1,-1
42029604:	0721                	addi	a4,a4,8
42029606:	04000613          	li	a2,64
4202960a:	880597e3          	bnez	a1,42028e98 <_ZN7kernels12r_tile8_b2p47layer_93run17hd4a8756f509e8c7bE+0x80>
4202960e:	7f010113          	addi	sp,sp,2032
42029612:	2c010113          	addi	sp,sp,704
42029616:	7ec12083          	lw	ra,2028(sp)
4202961a:	7e812403          	lw	s0,2024(sp)
4202961e:	7e412483          	lw	s1,2020(sp)
42029622:	7e012903          	lw	s2,2016(sp)
42029626:	7dc12983          	lw	s3,2012(sp)
4202962a:	7d812a03          	lw	s4,2008(sp)
4202962e:	7d412a83          	lw	s5,2004(sp)
42029632:	7d012b03          	lw	s6,2000(sp)
42029636:	7cc12b83          	lw	s7,1996(sp)
4202963a:	7c812c03          	lw	s8,1992(sp)
4202963e:	7c412c83          	lw	s9,1988(sp)
42029642:	7c012d03          	lw	s10,1984(sp)
42029646:	7bc12d83          	lw	s11,1980(sp)
4202964a:	7f010113          	addi	sp,sp,2032
4202964e:	8082                	ret
42029650:	3c2b6637          	lui	a2,0x3c2b6
42029654:	5fc60613          	addi	a2,a2,1532 # 3c2b65fc <c7_weights_0+0x17fc>
42029658:	8536                	mv	a0,a3
4202965a:	85ba                	mv	a1,a4
4202965c:	00010097          	auipc	ra,0x10
42029660:	0ae080e7          	jalr	174(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
42029664:	3c2b6637          	lui	a2,0x3c2b6
42029668:	5fc60613          	addi	a2,a2,1532 # 3c2b65fc <c7_weights_0+0x17fc>
4202966c:	45a5                	li	a1,9
4202966e:	05a6                	slli	a1,a1,0x9
42029670:	853a                	mv	a0,a4
42029672:	00010097          	auipc	ra,0x10
42029676:	088080e7          	jalr	136(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
4202967a:	3c2b6637          	lui	a2,0x3c2b6
4202967e:	60c60613          	addi	a2,a2,1548 # 3c2b660c <c7_weights_0+0x180c>
42029682:	853a                	mv	a0,a4
42029684:	00010097          	auipc	ra,0x10
42029688:	086080e7          	jalr	134(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
4202968c:	3c2b6637          	lui	a2,0x3c2b6
42029690:	60c60613          	addi	a2,a2,1548 # 3c2b660c <c7_weights_0+0x180c>
42029694:	46a5                	li	a3,9
42029696:	06a6                	slli	a3,a3,0x9
42029698:	852e                	mv	a0,a1
4202969a:	85b6                	mv	a1,a3
4202969c:	00010097          	auipc	ra,0x10
420296a0:	05e080e7          	jalr	94(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
420296a4:	6c068513          	addi	a0,a3,1728
420296a8:	3c2b6637          	lui	a2,0x3c2b6
420296ac:	61c60613          	addi	a2,a2,1564 # 3c2b661c <c7_weights_0+0x181c>
420296b0:	45a5                	li	a1,9
420296b2:	05a6                	slli	a1,a1,0x9
420296b4:	00010097          	auipc	ra,0x10
420296b8:	046080e7          	jalr	70(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
420296bc:	7ff68513          	addi	a0,a3,2047
420296c0:	10150513          	addi	a0,a0,257
420296c4:	3c2b6637          	lui	a2,0x3c2b6
420296c8:	62c60613          	addi	a2,a2,1580 # 3c2b662c <c7_weights_0+0x182c>
420296cc:	45a5                	li	a1,9
420296ce:	05a6                	slli	a1,a1,0x9
420296d0:	00010097          	auipc	ra,0x10
420296d4:	02a080e7          	jalr	42(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
420296d8:	48068593          	addi	a1,a3,1152
420296dc:	3c2b6637          	lui	a2,0x3c2b6
420296e0:	5ec60613          	addi	a2,a2,1516 # 3c2b65ec <c7_weights_0+0x17ec>
420296e4:	8536                	mv	a0,a3
420296e6:	00010097          	auipc	ra,0x10
420296ea:	024080e7          	jalr	36(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
420296ee:	4546                	lw	a0,80(sp)
420296f0:	45b6                	lw	a1,76(sp)
420296f2:	952e                	add	a0,a0,a1
420296f4:	5606                	lw	a2,96(sp)
420296f6:	55a6                	lw	a1,104(sp)
420296f8:	962e                	add	a2,a2,a1
420296fa:	04050593          	addi	a1,a0,64
420296fe:	e0060513          	addi	a0,a2,-512
42029702:	3c2b6637          	lui	a2,0x3c2b6
42029706:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
4202970a:	00010097          	auipc	ra,0x10
4202970e:	000080e7          	jalr	ra # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
42029712:	5506                	lw	a0,96(sp)
42029714:	55a6                	lw	a1,104(sp)
42029716:	952e                	add	a0,a0,a1
42029718:	e4050513          	addi	a0,a0,-448
4202971c:	3c2b6637          	lui	a2,0x3c2b6
42029720:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
42029724:	6585                	lui	a1,0x1
42029726:	00010097          	auipc	ra,0x10
4202972a:	fd4080e7          	jalr	-44(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
4202972e:	5506                	lw	a0,96(sp)
42029730:	55a6                	lw	a1,104(sp)
42029732:	952e                	add	a0,a0,a1
42029734:	dc050713          	addi	a4,a0,-576
42029738:	e0050593          	addi	a1,a0,-512
4202973c:	3c2b6637          	lui	a2,0x3c2b6
42029740:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
42029744:	853a                	mv	a0,a4
42029746:	00010097          	auipc	ra,0x10
4202974a:	fc4080e7          	jalr	-60(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
4202974e:	5506                	lw	a0,96(sp)
42029750:	55a6                	lw	a1,104(sp)
42029752:	952e                	add	a0,a0,a1
42029754:	e0050513          	addi	a0,a0,-512
42029758:	3c2b6637          	lui	a2,0x3c2b6
4202975c:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
42029760:	6585                	lui	a1,0x1
42029762:	00010097          	auipc	ra,0x10
42029766:	f98080e7          	jalr	-104(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
4202976a:	4546                	lw	a0,80(sp)
4202976c:	45b6                	lw	a1,76(sp)
4202976e:	952e                	add	a0,a0,a1
42029770:	5606                	lw	a2,96(sp)
42029772:	55a6                	lw	a1,104(sp)
42029774:	962e                	add	a2,a2,a1
42029776:	08050593          	addi	a1,a0,128
4202977a:	e4060513          	addi	a0,a2,-448
4202977e:	3c2b6637          	lui	a2,0x3c2b6
42029782:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
42029786:	00010097          	auipc	ra,0x10
4202978a:	f84080e7          	jalr	-124(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
4202978e:	4546                	lw	a0,80(sp)
42029790:	5626                	lw	a2,104(sp)
42029792:	00c565b3          	or	a1,a0,a2
42029796:	5506                	lw	a0,96(sp)
42029798:	9532                	add	a0,a0,a2
4202979a:	fc050513          	addi	a0,a0,-64
4202979e:	3c2b6637          	lui	a2,0x3c2b6
420297a2:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
420297a6:	00010097          	auipc	ra,0x10
420297aa:	f64080e7          	jalr	-156(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
420297ae:	5506                	lw	a0,96(sp)
420297b0:	55a6                	lw	a1,104(sp)
420297b2:	952e                	add	a0,a0,a1
420297b4:	e8050513          	addi	a0,a0,-384
420297b8:	3c2b6637          	lui	a2,0x3c2b6
420297bc:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
420297c0:	6585                	lui	a1,0x1
420297c2:	00010097          	auipc	ra,0x10
420297c6:	f38080e7          	jalr	-200(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
420297ca:	854e                	mv	a0,s3
420297cc:	3c2b6637          	lui	a2,0x3c2b6
420297d0:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
420297d4:	6585                	lui	a1,0x1
420297d6:	00010097          	auipc	ra,0x10
420297da:	f24080e7          	jalr	-220(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
420297de:	4546                	lw	a0,80(sp)
420297e0:	9522                	add	a0,a0,s0
420297e2:	08050593          	addi	a1,a0,128
420297e6:	3c2b6637          	lui	a2,0x3c2b6
420297ea:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
420297ee:	853a                	mv	a0,a4
420297f0:	00010097          	auipc	ra,0x10
420297f4:	f1a080e7          	jalr	-230(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
420297f8:	45c6                	lw	a1,80(sp)
420297fa:	8dc5                	or	a1,a1,s1
420297fc:	3c2b6637          	lui	a2,0x3c2b6
42029800:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
42029804:	853a                	mv	a0,a4
42029806:	00010097          	auipc	ra,0x10
4202980a:	f04080e7          	jalr	-252(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
4202980e:	4546                	lw	a0,80(sp)
42029810:	94aa                	add	s1,s1,a0
42029812:	04048593          	addi	a1,s1,64
42029816:	20098513          	addi	a0,s3,512
4202981a:	3c2b6637          	lui	a2,0x3c2b6
4202981e:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
42029822:	00010097          	auipc	ra,0x10
42029826:	ee8080e7          	jalr	-280(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
4202982a:	4546                	lw	a0,80(sp)
4202982c:	94aa                	add	s1,s1,a0
4202982e:	08048593          	addi	a1,s1,128
42029832:	3c2b6637          	lui	a2,0x3c2b6
42029836:	68c60613          	addi	a2,a2,1676 # 3c2b668c <c7_weights_0+0x188c>
4202983a:	853a                	mv	a0,a4
4202983c:	00010097          	auipc	ra,0x10
42029840:	ece080e7          	jalr	-306(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
42029844:	3c2b6637          	lui	a2,0x3c2b6
42029848:	65c60613          	addi	a2,a2,1628 # 3c2b665c <c7_weights_0+0x185c>
4202984c:	6585                	lui	a1,0x1
4202984e:	855a                	mv	a0,s6
42029850:	00010097          	auipc	ra,0x10
42029854:	ba0080e7          	jalr	-1120(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42029858:	3c2b6637          	lui	a2,0x3c2b6
4202985c:	67c60613          	addi	a2,a2,1660 # 3c2b667c <c7_weights_0+0x187c>
42029860:	6585                	lui	a1,0x1
42029862:	00010097          	auipc	ra,0x10
42029866:	b8e080e7          	jalr	-1138(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
4202986a:	3c2b6637          	lui	a2,0x3c2b6
4202986e:	64c60613          	addi	a2,a2,1612 # 3c2b664c <c7_weights_0+0x184c>
42029872:	6585                	lui	a1,0x1
42029874:	8556                	mv	a0,s5
42029876:	00010097          	auipc	ra,0x10
4202987a:	b7a080e7          	jalr	-1158(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
4202987e:	3c2b6637          	lui	a2,0x3c2b6
42029882:	66c60613          	addi	a2,a2,1644 # 3c2b666c <c7_weights_0+0x186c>
42029886:	6585                	lui	a1,0x1
42029888:	8536                	mv	a0,a3
4202988a:	00010097          	auipc	ra,0x10
4202988e:	b66080e7          	jalr	-1178(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42029892:	3c2b6637          	lui	a2,0x3c2b6
42029896:	63c60613          	addi	a2,a2,1596 # 3c2b663c <c7_weights_0+0x183c>
4202989a:	04000593          	li	a1,64
4202989e:	8566                	mv	a0,s9
420298a0:	00010097          	auipc	ra,0x10
420298a4:	b50080e7          	jalr	-1200(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
