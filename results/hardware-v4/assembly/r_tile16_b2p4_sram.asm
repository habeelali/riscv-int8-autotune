
hardware/esp32c3-v4/build/riscv_int8_generalize.elf:     file format elf32-littleriscv


Disassembly of section .iram0.text:

Disassembly of section .flash.text:

42033198 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E>:
42033198:	7111                	addi	sp,sp,-256
4203319a:	df86                	sw	ra,252(sp)
4203319c:	dda2                	sw	s0,248(sp)
4203319e:	dba6                	sw	s1,244(sp)
420331a0:	d9ca                	sw	s2,240(sp)
420331a2:	d7ce                	sw	s3,236(sp)
420331a4:	d5d2                	sw	s4,232(sp)
420331a6:	d3d6                	sw	s5,228(sp)
420331a8:	d1da                	sw	s6,224(sp)
420331aa:	cfde                	sw	s7,220(sp)
420331ac:	cde2                	sw	s8,216(sp)
420331ae:	cbe6                	sw	s9,212(sp)
420331b0:	c9ea                	sw	s10,208(sp)
420331b2:	c7ee                	sw	s11,204(sp)
420331b4:	6809                	lui	a6,0x2
420331b6:	3a080813          	addi	a6,a6,928 # 23a0 <RvExcFrameSize+0x230c>
420331ba:	41010133          	sub	sp,sp,a6
420331be:	8b3e                	mv	s6,a5
420331c0:	d03a                	sw	a4,32(sp)
420331c2:	ce36                	sw	a3,28(sp)
420331c4:	cc32                	sw	a2,24(sp)
420331c6:	842e                	mv	s0,a1
420331c8:	84aa                	mv	s1,a0
420331ca:	40000cb7          	lui	s9,0x40000
420331ce:	4525                	li	a0,9
420331d0:	00a51613          	slli	a2,a0,0xa
420331d4:	10e8                	addi	a0,sp,108
420331d6:	4581                	li	a1,0
420331d8:	c0b2                	sw	a2,64(sp)
420331da:	fdfcd097          	auipc	ra,0xfdfcd
420331de:	17a080e7          	jalr	378(ra) # 40000354 <memset>
420331e2:	4701                	li	a4,0
420331e4:	ca22                	sw	s0,20(sp)
420331e6:	0405                	addi	s0,s0,1
420331e8:	c822                	sw	s0,16(sp)
420331ea:	450d                	li	a0,3
420331ec:	04000613          	li	a2,64
420331f0:	6689                	lui	a3,0x2
420331f2:	45ed                	li	a1,27
420331f4:	f8068693          	addi	a3,a3,-128 # 1f80 <RvExcFrameSize+0x1eec>
420331f8:	de36                	sw	a3,60(sp)
420331fa:	05a2                	slli	a1,a1,0x8
420331fc:	dc2e                	sw	a1,56(sp)
420331fe:	d426                	sw	s1,40(sp)
42033200:	40e606b3          	sub	a3,a2,a4
42033204:	45c1                	li	a1,16
42033206:	c62a                	sw	a0,12(sp)
42033208:	c8b6                	sw	a3,80(sp)
4203320a:	00b6e463          	bltu	a3,a1,42033212 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x7a>
4203320e:	4541                	li	a0,16
42033210:	c8aa                	sw	a0,80(sp)
42033212:	d4ba                	sw	a4,104(sp)
42033214:	42c71b63          	bne	a4,a2,4203364a <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x4b2>
42033218:	da02                	sw	zero,52(sp)
4203321a:	4501                	li	a0,0
4203321c:	4646                	lw	a2,80(sp)
4203321e:	8209                	srli	a2,a2,0x2
42033220:	44fd                	li	s1,31
42033222:	fff60693          	addi	a3,a2,-1
42033226:	00d63633          	sltu	a2,a2,a3
4203322a:	167d                	addi	a2,a2,-1
4203322c:	8e75                	and	a2,a2,a3
4203322e:	d232                	sw	a2,36(sp)
42033230:	4789                	li	a5,2
42033232:	49c2                	lw	s3,16(sp)
42033234:	a005                	j	42033254 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0xbc>
42033236:	5642                	lw	a2,48(sp)
42033238:	8532                	mv	a0,a2
4203323a:	00260793          	addi	a5,a2,2
4203323e:	55b2                	lw	a1,44(sp)
42033240:	fff58493          	addi	s1,a1,-1
42033244:	48098993          	addi	s3,s3,1152
42033248:	56d2                	lw	a3,52(sp)
4203324a:	0685                	addi	a3,a3,1
4203324c:	da36                	sw	a3,52(sp)
4203324e:	5726                	lw	a4,104(sp)
42033250:	74058a63          	beqz	a1,420339a4 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x80c>
42033254:	24000613          	li	a2,576
42033258:	02c506b3          	mul	a3,a0,a2
4203325c:	b8000593          	li	a1,-1152
42033260:	02b6f4e3          	bgeu	a3,a1,42033a88 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8f0>
42033264:	d626                	sw	s1,44(sp)
42033266:	d83e                	sw	a5,48(sp)
42033268:	04000593          	li	a1,64
4203326c:	fcb705e3          	beq	a4,a1,42033236 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x9e>
42033270:	03f00593          	li	a1,63
42033274:	06a5ede3          	bltu	a1,a0,42033aee <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x956>
42033278:	4b81                	li	s7,0
4203327a:	48000593          	li	a1,1152
4203327e:	5652                	lw	a2,52(sp)
42033280:	02b605b3          	mul	a1,a2,a1
42033284:	04050613          	addi	a2,a0,64
42033288:	ceb2                	sw	a2,92(sp)
4203328a:	08050613          	addi	a2,a0,128
4203328e:	c8b2                	sw	a2,80(sp)
42033290:	0c050613          	addi	a2,a0,192
42033294:	c6b2                	sw	a2,76(sp)
42033296:	00251c13          	slli	s8,a0,0x2
4203329a:	4652                	lw	a2,20(sp)
4203329c:	95b2                	add	a1,a1,a2
4203329e:	4762                	lw	a4,24(sp)
420332a0:	01870633          	add	a2,a4,s8
420332a4:	d2b2                	sw	a2,100(sp)
420332a6:	004c0f93          	addi	t6,s8,4
420332aa:	46f2                	lw	a3,28(sp)
420332ac:	01868633          	add	a2,a3,s8
420332b0:	ccb2                	sw	a2,88(sp)
420332b2:	5602                	lw	a2,32(sp)
420332b4:	9c32                	add	s8,s8,a2
420332b6:	cae2                	sw	s8,84(sp)
420332b8:	48158f13          	addi	t5,a1,1153
420332bc:	977e                	add	a4,a4,t6
420332be:	d0ba                	sw	a4,96(sp)
420332c0:	96fe                	add	a3,a3,t6
420332c2:	c4b6                	sw	a3,72(sp)
420332c4:	967e                	add	a2,a2,t6
420332c6:	c2b2                	sw	a2,68(sp)
420332c8:	4911                	li	s2,4
420332ca:	4ec10a13          	addi	s4,sp,1260
420332ce:	5592                	lw	a1,36(sp)
420332d0:	a829                	j	420332ea <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x152>
420332d2:	00b380a3          	sb	a1,1(t2)
420332d6:	8bca                	mv	s7,s2
420332d8:	0911                	addi	s2,s2,4
420332da:	fffd8593          	addi	a1,s11,-1
420332de:	7ffa0613          	addi	a2,s4,2047
420332e2:	10160a13          	addi	s4,a2,257
420332e6:	f40d88e3          	beqz	s11,42033236 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x9e>
420332ea:	8dae                	mv	s11,a1
420332ec:	24000593          	li	a1,576
420332f0:	02bb86b3          	mul	a3,s7,a1
420332f4:	24068713          	addi	a4,a3,576
420332f8:	dbf00593          	li	a1,-577
420332fc:	6cd5ef63          	bltu	a1,a3,420339da <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x842>
42033300:	4586                	lw	a1,64(sp)
42033302:	6ee5e663          	bltu	a1,a4,420339ee <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x856>
42033306:	48068593          	addi	a1,a3,1152
4203330a:	6ee5ed63          	bltu	a1,a4,42033a04 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x86c>
4203330e:	5672                	lw	a2,60(sp)
42033310:	70d66363          	bltu	a2,a3,42033a16 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x87e>
42033314:	6589                	lui	a1,0x2
42033316:	d4058593          	addi	a1,a1,-704 # 1d40 <RvExcFrameSize+0x1cac>
4203331a:	70d5ea63          	bltu	a1,a3,42033a2e <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x896>
4203331e:	55e2                	lw	a1,56(sp)
42033320:	72d5e363          	bltu	a1,a3,42033a46 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8ae>
42033324:	5596                	lw	a1,100(sp)
42033326:	0005ad03          	lw	s10,0(a1)
4203332a:	5586                	lw	a1,96(sp)
4203332c:	0005a883          	lw	a7,0(a1)
42033330:	86d2                	mv	a3,s4
42033332:	864e                	mv	a2,s3
42033334:	8eea                	mv	t4,s10
42033336:	8fea                	mv	t6,s10
42033338:	8c6a                	mv	s8,s10
4203333a:	82c6                	mv	t0,a7
4203333c:	8846                	mv	a6,a7
4203333e:	80c6                	mv	ra,a7
42033340:	b8068583          	lb	a1,-1152(a3)
42033344:	24068303          	lb	t1,576(a3)
42033348:	fff60783          	lb	a5,-1(a2)
4203334c:	00068483          	lb	s1,0(a3)
42033350:	dc068403          	lb	s0,-576(a3)
42033354:	00060703          	lb	a4,0(a2)
42033358:	026783b3          	mul	t2,a5,t1
4203335c:	02978e33          	mul	t3,a5,s1
42033360:	02878ab3          	mul	s5,a5,s0
42033364:	02b787b3          	mul	a5,a5,a1
42033368:	02b705b3          	mul	a1,a4,a1
4203336c:	02870433          	mul	s0,a4,s0
42033370:	029704b3          	mul	s1,a4,s1
42033374:	02670733          	mul	a4,a4,t1
42033378:	9c1e                	add	s8,s8,t2
4203337a:	9ff2                	add	t6,t6,t3
4203337c:	0609                	addi	a2,a2,2
4203337e:	9ed6                	add	t4,t4,s5
42033380:	9d3e                	add	s10,s10,a5
42033382:	98ae                	add	a7,a7,a1
42033384:	92a2                	add	t0,t0,s0
42033386:	9826                	add	a6,a6,s1
42033388:	90ba                	add	ra,ra,a4
4203338a:	0685                	addi	a3,a3,1
4203338c:	fbe61ae3          	bne	a2,t5,42033340 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x1a8>
42033390:	53a6                	lw	t2,104(sp)
42033392:	93de                	add	t2,t2,s7
42033394:	039a                	slli	t2,t2,0x6
42033396:	00a3e6b3          	or	a3,t2,a0
4203339a:	00c6d613          	srli	a2,a3,0xc
4203339e:	72061463          	bnez	a2,42033ac6 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x92e>
420333a2:	45d6                	lw	a1,84(sp)
420333a4:	4190                	lw	a2,0(a1)
420333a6:	45e6                	lw	a1,88(sp)
420333a8:	0005ae03          	lw	t3,0(a1)
420333ac:	fff64313          	not	t1,a2
420333b0:	40c00633          	neg	a2,a2
420333b4:	03ae1ab3          	mulh	s5,t3,s10
420333b8:	03ae0433          	mul	s0,t3,s10
420333bc:	4585                	li	a1,1
420333be:	00659333          	sll	t1,a1,t1
420333c2:	01f67d13          	andi	s10,a2,31
420333c6:	01940633          	add	a2,s0,s9
420333ca:	00863433          	sltu	s0,a2,s0
420333ce:	827d                	srli	a2,a2,0x1f
420333d0:	9456                	add	s0,s0,s5
420333d2:	0406                	slli	s0,s0,0x1
420333d4:	8e41                	or	a2,a2,s0
420333d6:	847d                	srai	s0,s0,0x1f
420333d8:	961a                	add	a2,a2,t1
420333da:	9622                	add	a2,a2,s0
420333dc:	41a65633          	sra	a2,a2,s10
420333e0:	1679                	addi	a2,a2,-2
420333e2:	07f00413          	li	s0,127
420333e6:	02865163          	bge	a2,s0,42033408 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x270>
420333ea:	f8000413          	li	s0,-128
420333ee:	02c45363          	bge	s0,a2,42033414 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x27c>
420333f2:	00db0bb3          	add	s7,s6,a3
420333f6:	45f6                	lw	a1,92(sp)
420333f8:	00b3e6b3          	or	a3,t2,a1
420333fc:	00c6d413          	srli	s0,a3,0xc
42033400:	00cb8023          	sb	a2,0(s7)
42033404:	c40d                	beqz	s0,4203342e <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x296>
42033406:	ad61                	j	42033a9e <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x906>
42033408:	07f00613          	li	a2,127
4203340c:	f8000413          	li	s0,-128
42033410:	fec441e3          	blt	s0,a2,420333f2 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x25a>
42033414:	f8000613          	li	a2,-128
42033418:	00db0bb3          	add	s7,s6,a3
4203341c:	45f6                	lw	a1,92(sp)
4203341e:	00b3e6b3          	or	a3,t2,a1
42033422:	00c6d413          	srli	s0,a3,0xc
42033426:	00cb8023          	sb	a2,0(s7)
4203342a:	66041a63          	bnez	s0,42033a9e <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x906>
4203342e:	03de1ab3          	mulh	s5,t3,t4
42033432:	03de0433          	mul	s0,t3,t4
42033436:	01940633          	add	a2,s0,s9
4203343a:	00863433          	sltu	s0,a2,s0
4203343e:	827d                	srli	a2,a2,0x1f
42033440:	9456                	add	s0,s0,s5
42033442:	0406                	slli	s0,s0,0x1
42033444:	8e41                	or	a2,a2,s0
42033446:	847d                	srai	s0,s0,0x1f
42033448:	961a                	add	a2,a2,t1
4203344a:	9622                	add	a2,a2,s0
4203344c:	41a65633          	sra	a2,a2,s10
42033450:	1679                	addi	a2,a2,-2
42033452:	07f00413          	li	s0,127
42033456:	02865163          	bge	a2,s0,42033478 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x2e0>
4203345a:	f8000413          	li	s0,-128
4203345e:	02c45363          	bge	s0,a2,42033484 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x2ec>
42033462:	00db0eb3          	add	t4,s6,a3
42033466:	45c6                	lw	a1,80(sp)
42033468:	00b3e6b3          	or	a3,t2,a1
4203346c:	00c6d413          	srli	s0,a3,0xc
42033470:	00ce8023          	sb	a2,0(t4)
42033474:	c40d                	beqz	s0,4203349e <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x306>
42033476:	a595                	j	42033ada <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x942>
42033478:	07f00613          	li	a2,127
4203347c:	f8000413          	li	s0,-128
42033480:	fec441e3          	blt	s0,a2,42033462 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x2ca>
42033484:	f8000613          	li	a2,-128
42033488:	00db0eb3          	add	t4,s6,a3
4203348c:	45c6                	lw	a1,80(sp)
4203348e:	00b3e6b3          	or	a3,t2,a1
42033492:	00c6d413          	srli	s0,a3,0xc
42033496:	00ce8023          	sb	a2,0(t4)
4203349a:	64041063          	bnez	s0,42033ada <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x942>
4203349e:	03fe1633          	mulh	a2,t3,t6
420334a2:	03fe0433          	mul	s0,t3,t6
420334a6:	019404b3          	add	s1,s0,s9
420334aa:	0084b433          	sltu	s0,s1,s0
420334ae:	80fd                	srli	s1,s1,0x1f
420334b0:	9622                	add	a2,a2,s0
420334b2:	0606                	slli	a2,a2,0x1
420334b4:	8cd1                	or	s1,s1,a2
420334b6:	867d                	srai	a2,a2,0x1f
420334b8:	949a                	add	s1,s1,t1
420334ba:	9626                	add	a2,a2,s1
420334bc:	41a65633          	sra	a2,a2,s10
420334c0:	1679                	addi	a2,a2,-2
420334c2:	07f00413          	li	s0,127
420334c6:	02865163          	bge	a2,s0,420334e8 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x350>
420334ca:	f8000413          	li	s0,-128
420334ce:	02c45363          	bge	s0,a2,420334f4 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x35c>
420334d2:	00db0ab3          	add	s5,s6,a3
420334d6:	45b6                	lw	a1,76(sp)
420334d8:	00b3e6b3          	or	a3,t2,a1
420334dc:	00c6d493          	srli	s1,a3,0xc
420334e0:	00ca8023          	sb	a2,0(s5)
420334e4:	c48d                	beqz	s1,4203350e <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x376>
420334e6:	a3f1                	j	42033ab2 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x91a>
420334e8:	07f00613          	li	a2,127
420334ec:	f8000413          	li	s0,-128
420334f0:	fec441e3          	blt	s0,a2,420334d2 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x33a>
420334f4:	f8000613          	li	a2,-128
420334f8:	00db0ab3          	add	s5,s6,a3
420334fc:	45b6                	lw	a1,76(sp)
420334fe:	00b3e6b3          	or	a3,t2,a1
42033502:	00c6d493          	srli	s1,a3,0xc
42033506:	00ca8023          	sb	a2,0(s5)
4203350a:	5a049463          	bnez	s1,42033ab2 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x91a>
4203350e:	038e1633          	mulh	a2,t3,s8
42033512:	038e07b3          	mul	a5,t3,s8
42033516:	019784b3          	add	s1,a5,s9
4203351a:	00f4b7b3          	sltu	a5,s1,a5
4203351e:	80fd                	srli	s1,s1,0x1f
42033520:	963e                	add	a2,a2,a5
42033522:	0606                	slli	a2,a2,0x1
42033524:	8cd1                	or	s1,s1,a2
42033526:	867d                	srai	a2,a2,0x1f
42033528:	9326                	add	t1,t1,s1
4203352a:	961a                	add	a2,a2,t1
4203352c:	41a65633          	sra	a2,a2,s10
42033530:	1679                	addi	a2,a2,-2
42033532:	07f00d13          	li	s10,127
42033536:	01a64463          	blt	a2,s10,4203353e <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x3a6>
4203353a:	07f00613          	li	a2,127
4203353e:	f8000313          	li	t1,-128
42033542:	00c34463          	blt	t1,a2,4203354a <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x3b2>
42033546:	f8000613          	li	a2,-128
4203354a:	45a6                	lw	a1,72(sp)
4203354c:	419c                	lw	a5,0(a1)
4203354e:	00db03b3          	add	t2,s6,a3
42033552:	4596                	lw	a1,68(sp)
42033554:	4194                	lw	a3,0(a1)
42033556:	03179e33          	mulh	t3,a5,a7
4203355a:	03178433          	mul	s0,a5,a7
4203355e:	019404b3          	add	s1,s0,s9
42033562:	0084b433          	sltu	s0,s1,s0
42033566:	9e22                	add	t3,t3,s0
42033568:	fff6c413          	not	s0,a3
4203356c:	40d008b3          	neg	a7,a3
42033570:	4585                	li	a1,1
42033572:	00859c33          	sll	s8,a1,s0
42033576:	80fd                	srli	s1,s1,0x1f
42033578:	0e06                	slli	t3,t3,0x1
4203357a:	01c4e4b3          	or	s1,s1,t3
4203357e:	41fe5693          	srai	a3,t3,0x1f
42033582:	94e2                	add	s1,s1,s8
42033584:	96a6                	add	a3,a3,s1
42033586:	4116d4b3          	sra	s1,a3,a7
4203358a:	14f9                	addi	s1,s1,-2
4203358c:	00c38023          	sb	a2,0(t2)
42033590:	01a4c463          	blt	s1,s10,42033598 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x400>
42033594:	07f00493          	li	s1,127
42033598:	00934463          	blt	t1,s1,420335a0 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x408>
4203359c:	f8000493          	li	s1,-128
420335a0:	02579633          	mulh	a2,a5,t0
420335a4:	025786b3          	mul	a3,a5,t0
420335a8:	01968433          	add	s0,a3,s9
420335ac:	00d436b3          	sltu	a3,s0,a3
420335b0:	9636                	add	a2,a2,a3
420335b2:	807d                	srli	s0,s0,0x1f
420335b4:	0606                	slli	a2,a2,0x1
420335b6:	8c51                	or	s0,s0,a2
420335b8:	867d                	srai	a2,a2,0x1f
420335ba:	9462                	add	s0,s0,s8
420335bc:	9622                	add	a2,a2,s0
420335be:	41165633          	sra	a2,a2,a7
420335c2:	1679                	addi	a2,a2,-2
420335c4:	009b80a3          	sb	s1,1(s7)
420335c8:	01a64463          	blt	a2,s10,420335d0 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x438>
420335cc:	07f00613          	li	a2,127
420335d0:	00c34463          	blt	t1,a2,420335d8 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x440>
420335d4:	f8000613          	li	a2,-128
420335d8:	030796b3          	mulh	a3,a5,a6
420335dc:	03078733          	mul	a4,a5,a6
420335e0:	019704b3          	add	s1,a4,s9
420335e4:	00e4b733          	sltu	a4,s1,a4
420335e8:	96ba                	add	a3,a3,a4
420335ea:	80fd                	srli	s1,s1,0x1f
420335ec:	0686                	slli	a3,a3,0x1
420335ee:	8cd5                	or	s1,s1,a3
420335f0:	86fd                	srai	a3,a3,0x1f
420335f2:	94e2                	add	s1,s1,s8
420335f4:	96a6                	add	a3,a3,s1
420335f6:	4116d733          	sra	a4,a3,a7
420335fa:	1779                	addi	a4,a4,-2
420335fc:	00ce80a3          	sb	a2,1(t4)
42033600:	01a74463          	blt	a4,s10,42033608 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x470>
42033604:	07f00713          	li	a4,127
42033608:	00e34463          	blt	t1,a4,42033610 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x478>
4203360c:	f8000713          	li	a4,-128
42033610:	02179633          	mulh	a2,a5,ra
42033614:	021785b3          	mul	a1,a5,ra
42033618:	019586b3          	add	a3,a1,s9
4203361c:	00b6b5b3          	sltu	a1,a3,a1
42033620:	95b2                	add	a1,a1,a2
42033622:	82fd                	srli	a3,a3,0x1f
42033624:	0586                	slli	a1,a1,0x1
42033626:	8ecd                	or	a3,a3,a1
42033628:	85fd                	srai	a1,a1,0x1f
4203362a:	96e2                	add	a3,a3,s8
4203362c:	95b6                	add	a1,a1,a3
4203362e:	4115d5b3          	sra	a1,a1,a7
42033632:	15f9                	addi	a1,a1,-2
42033634:	00ea80a3          	sb	a4,1(s5)
42033638:	01a5c463          	blt	a1,s10,42033640 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x4a8>
4203363c:	07f00593          	li	a1,127
42033640:	c8b349e3          	blt	t1,a1,420332d2 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x13a>
42033644:	f8000593          	li	a1,-128
42033648:	b169                	j	420332d2 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x13a>
4203364a:	4601                	li	a2,0
4203364c:	24000513          	li	a0,576
42033650:	45c6                	lw	a1,80(sp)
42033652:	02a587b3          	mul	a5,a1,a0
42033656:	06c10a13          	addi	s4,sp,108
4203365a:	16c10c13          	addi	s8,sp,364
4203365e:	1568                	addi	a0,sp,684
42033660:	a82d                	j	4203369a <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x502>
42033662:	08000593          	li	a1,128
42033666:	04000613          	li	a2,64
4203366a:	8536                	mv	a0,a3
4203366c:	fdfcd097          	auipc	ra,0xfdfcd
42033670:	ce8080e7          	jalr	-792(ra) # 40000354 <memset>
42033674:	47d6                	lw	a5,84(sp)
42033676:	dc078793          	addi	a5,a5,-576
4203367a:	240c0c13          	addi	s8,s8,576
4203367e:	2407b513          	sltiu	a0,a5,576
42033682:	157d                	addi	a0,a0,-1
42033684:	24057513          	andi	a0,a0,576
42033688:	4a66                	lw	s4,88(sp)
4203368a:	9552                	add	a0,a0,s4
4203368c:	4676                	lw	a2,92(sp)
4203368e:	0605                	addi	a2,a2,1
42033690:	5726                	lw	a4,104(sp)
42033692:	24000593          	li	a1,576
42033696:	b8b7e1e3          	bltu	a5,a1,42033218 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x80>
4203369a:	85aa                	mv	a1,a0
4203369c:	f00c0693          	addi	a3,s8,-256
420336a0:	9732                	add	a4,a4,a2
420336a2:	00767d13          	andi	s10,a2,7
420336a6:	d0ba                	sw	a4,96(sp)
420336a8:	00375d93          	srli	s11,a4,0x3
420336ac:	fffd0513          	addi	a0,s10,-1
420336b0:	fffd8993          	addi	s3,s11,-1
420336b4:	009d9b93          	slli	s7,s11,0x9
420336b8:	dc0b8a93          	addi	s5,s7,-576
420336bc:	e00b8493          	addi	s1,s7,-512
420336c0:	d2aa                	sw	a0,100(sp)
420336c2:	01356533          	or	a0,a0,s3
420336c6:	006d1913          	slli	s2,s10,0x6
420336ca:	4421                	li	s0,8
420336cc:	ceb2                	sw	a2,92(sp)
420336ce:	cabe                	sw	a5,84(sp)
420336d0:	ccae                	sw	a1,88(sp)
420336d2:	02857f63          	bgeu	a0,s0,42033710 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x578>
420336d6:	01590533          	add	a0,s2,s5
420336da:	00996733          	or	a4,s2,s1
420336de:	38a76c63          	bltu	a4,a0,42033a76 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8de>
420336e2:	5a22                	lw	s4,40(sp)
420336e4:	6585                	lui	a1,0x1
420336e6:	36e5ee63          	bltu	a1,a4,42033a62 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8ca>
420336ea:	00aa05b3          	add	a1,s4,a0
420336ee:	04000613          	li	a2,64
420336f2:	8536                	mv	a0,a3
420336f4:	fdfcd097          	auipc	ra,0xfdfcd
420336f8:	c64080e7          	jalr	-924(ra) # 40000358 <memcpy>
420336fc:	f40c0693          	addi	a3,s8,-192
42033700:	04090713          	addi	a4,s2,64
42033704:	01570533          	add	a0,a4,s5
42033708:	9726                	add	a4,a4,s1
4203370a:	02a77863          	bgeu	a4,a0,4203373a <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x5a2>
4203370e:	a6a5                	j	42033a76 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8de>
42033710:	08000593          	li	a1,128
42033714:	04000613          	li	a2,64
42033718:	8536                	mv	a0,a3
4203371a:	fdfcd097          	auipc	ra,0xfdfcd
4203371e:	c3a080e7          	jalr	-966(ra) # 40000354 <memset>
42033722:	0c89fd63          	bgeu	s3,s0,420337fc <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x664>
42033726:	040a0693          	addi	a3,s4,64
4203372a:	5a22                	lw	s4,40(sp)
4203372c:	04090713          	addi	a4,s2,64
42033730:	01570533          	add	a0,a4,s5
42033734:	9726                	add	a4,a4,s1
42033736:	34a76063          	bltu	a4,a0,42033a76 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8de>
4203373a:	6585                	lui	a1,0x1
4203373c:	32e5e363          	bltu	a1,a4,42033a62 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8ca>
42033740:	00aa05b3          	add	a1,s4,a0
42033744:	04000613          	li	a2,64
42033748:	8536                	mv	a0,a3
4203374a:	fdfcd097          	auipc	ra,0xfdfcd
4203374e:	c0e080e7          	jalr	-1010(ra) # 40000358 <memcpy>
42033752:	0d05                	addi	s10,s10,1
42033754:	013d6533          	or	a0,s10,s3
42033758:	f80c0693          	addi	a3,s8,-128
4203375c:	4421                	li	s0,8
4203375e:	0c857263          	bgeu	a0,s0,42033822 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x68a>
42033762:	08090713          	addi	a4,s2,128
42033766:	01570533          	add	a0,a4,s5
4203376a:	9726                	add	a4,a4,s1
4203376c:	30a76563          	bltu	a4,a0,42033a76 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8de>
42033770:	6585                	lui	a1,0x1
42033772:	2ee5e863          	bltu	a1,a4,42033a62 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8ca>
42033776:	00aa05b3          	add	a1,s4,a0
4203377a:	04000613          	li	a2,64
4203377e:	8536                	mv	a0,a3
42033780:	fdfcd097          	auipc	ra,0xfdfcd
42033784:	bd8080e7          	jalr	-1064(ra) # 40000358 <memcpy>
42033788:	4421                	li	s0,8
4203378a:	fc0b8493          	addi	s1,s7,-64
4203378e:	5516                	lw	a0,100(sp)
42033790:	01b56533          	or	a0,a0,s11
42033794:	fc0c0693          	addi	a3,s8,-64
42033798:	0a856763          	bltu	a0,s0,42033846 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x6ae>
4203379c:	08000593          	li	a1,128
420337a0:	04000613          	li	a2,64
420337a4:	8536                	mv	a0,a3
420337a6:	fdfcd097          	auipc	ra,0xfdfcd
420337aa:	bae080e7          	jalr	-1106(ra) # 40000354 <memset>
420337ae:	04000513          	li	a0,64
420337b2:	5586                	lw	a1,96(sp)
420337b4:	0ca5f163          	bgeu	a1,a0,42033876 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x6de>
420337b8:	01790533          	add	a0,s2,s7
420337bc:	04050713          	addi	a4,a0,64
420337c0:	6505                	lui	a0,0x1
420337c2:	2ae56063          	bltu	a0,a4,42033a62 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8ca>
420337c6:	017965b3          	or	a1,s2,s7
420337ca:	95d2                	add	a1,a1,s4
420337cc:	04000613          	li	a2,64
420337d0:	8562                	mv	a0,s8
420337d2:	fdfcd097          	auipc	ra,0xfdfcd
420337d6:	b86080e7          	jalr	-1146(ra) # 40000358 <memcpy>
420337da:	4421                	li	s0,8
420337dc:	01bd6533          	or	a0,s10,s11
420337e0:	040c0693          	addi	a3,s8,64
420337e4:	0a856863          	bltu	a0,s0,42033894 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x6fc>
420337e8:	08000593          	li	a1,128
420337ec:	04000613          	li	a2,64
420337f0:	8536                	mv	a0,a3
420337f2:	fdfcd097          	auipc	ra,0xfdfcd
420337f6:	b62080e7          	jalr	-1182(ra) # 40000354 <memset>
420337fa:	a0c9                	j	420338bc <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x724>
420337fc:	f40c0513          	addi	a0,s8,-192
42033800:	08000593          	li	a1,128
42033804:	04000613          	li	a2,64
42033808:	fdfcd097          	auipc	ra,0xfdfcd
4203380c:	b4c080e7          	jalr	-1204(ra) # 40000354 <memset>
42033810:	5a22                	lw	s4,40(sp)
42033812:	0d05                	addi	s10,s10,1
42033814:	013d6533          	or	a0,s10,s3
42033818:	f80c0693          	addi	a3,s8,-128
4203381c:	4421                	li	s0,8
4203381e:	f48562e3          	bltu	a0,s0,42033762 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x5ca>
42033822:	08000593          	li	a1,128
42033826:	04000613          	li	a2,64
4203382a:	8536                	mv	a0,a3
4203382c:	fdfcd097          	auipc	ra,0xfdfcd
42033830:	b28080e7          	jalr	-1240(ra) # 40000354 <memset>
42033834:	fc0b8493          	addi	s1,s7,-64
42033838:	5516                	lw	a0,100(sp)
4203383a:	01b56533          	or	a0,a0,s11
4203383e:	fc0c0693          	addi	a3,s8,-64
42033842:	f4857de3          	bgeu	a0,s0,4203379c <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x604>
42033846:	00990533          	add	a0,s2,s1
4203384a:	01796733          	or	a4,s2,s7
4203384e:	22a76463          	bltu	a4,a0,42033a76 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8de>
42033852:	6585                	lui	a1,0x1
42033854:	20e5e763          	bltu	a1,a4,42033a62 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8ca>
42033858:	00aa05b3          	add	a1,s4,a0
4203385c:	04000613          	li	a2,64
42033860:	8536                	mv	a0,a3
42033862:	fdfcd097          	auipc	ra,0xfdfcd
42033866:	af6080e7          	jalr	-1290(ra) # 40000358 <memcpy>
4203386a:	4421                	li	s0,8
4203386c:	04000513          	li	a0,64
42033870:	5586                	lw	a1,96(sp)
42033872:	f4a5e3e3          	bltu	a1,a0,420337b8 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x620>
42033876:	08000593          	li	a1,128
4203387a:	04000613          	li	a2,64
4203387e:	8562                	mv	a0,s8
42033880:	fdfcd097          	auipc	ra,0xfdfcd
42033884:	ad4080e7          	jalr	-1324(ra) # 40000354 <memset>
42033888:	01bd6533          	or	a0,s10,s11
4203388c:	040c0693          	addi	a3,s8,64
42033890:	f4857ce3          	bgeu	a0,s0,420337e8 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x650>
42033894:	08090713          	addi	a4,s2,128
42033898:	00970533          	add	a0,a4,s1
4203389c:	975e                	add	a4,a4,s7
4203389e:	1ca76c63          	bltu	a4,a0,42033a76 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8de>
420338a2:	6585                	lui	a1,0x1
420338a4:	1ae5ef63          	bltu	a1,a4,42033a62 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8ca>
420338a8:	00aa05b3          	add	a1,s4,a0
420338ac:	04000613          	li	a2,64
420338b0:	8536                	mv	a0,a3
420338b2:	fdfcd097          	auipc	ra,0xfdfcd
420338b6:	aa6080e7          	jalr	-1370(ra) # 40000358 <memcpy>
420338ba:	4421                	li	s0,8
420338bc:	0d85                	addi	s11,s11,1
420338be:	1c0b8993          	addi	s3,s7,448
420338c2:	200b8493          	addi	s1,s7,512
420338c6:	5516                	lw	a0,100(sp)
420338c8:	01b56533          	or	a0,a0,s11
420338cc:	080c0693          	addi	a3,s8,128
420338d0:	04857c63          	bgeu	a0,s0,42033928 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x790>
420338d4:	01390533          	add	a0,s2,s3
420338d8:	00996733          	or	a4,s2,s1
420338dc:	18a76d63          	bltu	a4,a0,42033a76 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8de>
420338e0:	6585                	lui	a1,0x1
420338e2:	18e5e063          	bltu	a1,a4,42033a62 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8ca>
420338e6:	00aa05b3          	add	a1,s4,a0
420338ea:	04000613          	li	a2,64
420338ee:	8536                	mv	a0,a3
420338f0:	fdfcd097          	auipc	ra,0xfdfcd
420338f4:	a68080e7          	jalr	-1432(ra) # 40000358 <memcpy>
420338f8:	4421                	li	s0,8
420338fa:	0c0c0693          	addi	a3,s8,192
420338fe:	03800513          	li	a0,56
42033902:	5586                	lw	a1,96(sp)
42033904:	04a5e263          	bltu	a1,a0,42033948 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x7b0>
42033908:	08000593          	li	a1,128
4203390c:	04000613          	li	a2,64
42033910:	8536                	mv	a0,a3
42033912:	fdfcd097          	auipc	ra,0xfdfcd
42033916:	a42080e7          	jalr	-1470(ra) # 40000354 <memset>
4203391a:	01bd6533          	or	a0,s10,s11
4203391e:	100c0693          	addi	a3,s8,256
42033922:	d48570e3          	bgeu	a0,s0,42033662 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x4ca>
42033926:	a899                	j	4203397c <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x7e4>
42033928:	08000593          	li	a1,128
4203392c:	04000613          	li	a2,64
42033930:	8536                	mv	a0,a3
42033932:	fdfcd097          	auipc	ra,0xfdfcd
42033936:	a22080e7          	jalr	-1502(ra) # 40000354 <memset>
4203393a:	0c0c0693          	addi	a3,s8,192
4203393e:	03800513          	li	a0,56
42033942:	5586                	lw	a1,96(sp)
42033944:	fca5f2e3          	bgeu	a1,a0,42033908 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x770>
42033948:	04090713          	addi	a4,s2,64
4203394c:	01370533          	add	a0,a4,s3
42033950:	9726                	add	a4,a4,s1
42033952:	1334e263          	bltu	s1,s3,42033a76 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8de>
42033956:	6585                	lui	a1,0x1
42033958:	10e5e563          	bltu	a1,a4,42033a62 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8ca>
4203395c:	00aa05b3          	add	a1,s4,a0
42033960:	04000613          	li	a2,64
42033964:	8536                	mv	a0,a3
42033966:	fdfcd097          	auipc	ra,0xfdfcd
4203396a:	9f2080e7          	jalr	-1550(ra) # 40000358 <memcpy>
4203396e:	4421                	li	s0,8
42033970:	01bd6533          	or	a0,s10,s11
42033974:	100c0693          	addi	a3,s8,256
42033978:	ce8575e3          	bgeu	a0,s0,42033662 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x4ca>
4203397c:	08090713          	addi	a4,s2,128
42033980:	01370533          	add	a0,a4,s3
42033984:	9726                	add	a4,a4,s1
42033986:	0ea76863          	bltu	a4,a0,42033a76 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8de>
4203398a:	6585                	lui	a1,0x1
4203398c:	0ce5eb63          	bltu	a1,a4,42033a62 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x8ca>
42033990:	00aa05b3          	add	a1,s4,a0
42033994:	04000613          	li	a2,64
42033998:	8536                	mv	a0,a3
4203399a:	fdfcd097          	auipc	ra,0xfdfcd
4203399e:	9be080e7          	jalr	-1602(ra) # 40000358 <memcpy>
420339a2:	b9c9                	j	42033674 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x4dc>
420339a4:	45b2                	lw	a1,12(sp)
420339a6:	fff58513          	addi	a0,a1,-1 # fff <RvExcFrameSize+0xf6b>
420339aa:	0741                	addi	a4,a4,16
420339ac:	04000613          	li	a2,64
420339b0:	840598e3          	bnez	a1,42033200 <_ZN7kernels18r_tile16_b2p4_sram7layer_93run17h7f04b44a93510415E+0x68>
420339b4:	6509                	lui	a0,0x2
420339b6:	3a050513          	addi	a0,a0,928 # 23a0 <RvExcFrameSize+0x230c>
420339ba:	912a                	add	sp,sp,a0
420339bc:	50fe                	lw	ra,252(sp)
420339be:	546e                	lw	s0,248(sp)
420339c0:	54de                	lw	s1,244(sp)
420339c2:	594e                	lw	s2,240(sp)
420339c4:	59be                	lw	s3,236(sp)
420339c6:	5a2e                	lw	s4,232(sp)
420339c8:	5a9e                	lw	s5,228(sp)
420339ca:	5b0e                	lw	s6,224(sp)
420339cc:	4bfe                	lw	s7,220(sp)
420339ce:	4c6e                	lw	s8,216(sp)
420339d0:	4cde                	lw	s9,212(sp)
420339d2:	4d4e                	lw	s10,208(sp)
420339d4:	4dbe                	lw	s11,204(sp)
420339d6:	6111                	addi	sp,sp,256
420339d8:	8082                	ret
420339da:	3c2b7637          	lui	a2,0x3c2b7
420339de:	21c60613          	addi	a2,a2,540 # 3c2b721c <c7_weights_0+0x241c>
420339e2:	8536                	mv	a0,a3
420339e4:	85ba                	mv	a1,a4
420339e6:	00006097          	auipc	ra,0x6
420339ea:	d24080e7          	jalr	-732(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
420339ee:	3c2b7637          	lui	a2,0x3c2b7
420339f2:	21c60613          	addi	a2,a2,540 # 3c2b721c <c7_weights_0+0x241c>
420339f6:	45a5                	li	a1,9
420339f8:	05aa                	slli	a1,a1,0xa
420339fa:	853a                	mv	a0,a4
420339fc:	00006097          	auipc	ra,0x6
42033a00:	cfe080e7          	jalr	-770(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
42033a04:	3c2b7637          	lui	a2,0x3c2b7
42033a08:	22c60613          	addi	a2,a2,556 # 3c2b722c <c7_weights_0+0x242c>
42033a0c:	853a                	mv	a0,a4
42033a0e:	00006097          	auipc	ra,0x6
42033a12:	cfc080e7          	jalr	-772(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
42033a16:	3c2b7637          	lui	a2,0x3c2b7
42033a1a:	22c60613          	addi	a2,a2,556 # 3c2b722c <c7_weights_0+0x242c>
42033a1e:	46a5                	li	a3,9
42033a20:	06aa                	slli	a3,a3,0xa
42033a22:	852e                	mv	a0,a1
42033a24:	85b6                	mv	a1,a3
42033a26:	00006097          	auipc	ra,0x6
42033a2a:	cd4080e7          	jalr	-812(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
42033a2e:	6c068513          	addi	a0,a3,1728
42033a32:	3c2b7637          	lui	a2,0x3c2b7
42033a36:	23c60613          	addi	a2,a2,572 # 3c2b723c <c7_weights_0+0x243c>
42033a3a:	45a5                	li	a1,9
42033a3c:	05aa                	slli	a1,a1,0xa
42033a3e:	00006097          	auipc	ra,0x6
42033a42:	cbc080e7          	jalr	-836(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
42033a46:	7ff68513          	addi	a0,a3,2047
42033a4a:	10150513          	addi	a0,a0,257
42033a4e:	3c2b7637          	lui	a2,0x3c2b7
42033a52:	24c60613          	addi	a2,a2,588 # 3c2b724c <c7_weights_0+0x244c>
42033a56:	45a5                	li	a1,9
42033a58:	05aa                	slli	a1,a1,0xa
42033a5a:	00006097          	auipc	ra,0x6
42033a5e:	ca0080e7          	jalr	-864(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
42033a62:	3c2b7637          	lui	a2,0x3c2b7
42033a66:	2ac60613          	addi	a2,a2,684 # 3c2b72ac <c7_weights_0+0x24ac>
42033a6a:	6585                	lui	a1,0x1
42033a6c:	853a                	mv	a0,a4
42033a6e:	00006097          	auipc	ra,0x6
42033a72:	c8c080e7          	jalr	-884(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
42033a76:	3c2b7637          	lui	a2,0x3c2b7
42033a7a:	2ac60613          	addi	a2,a2,684 # 3c2b72ac <c7_weights_0+0x24ac>
42033a7e:	85ba                	mv	a1,a4
42033a80:	00006097          	auipc	ra,0x6
42033a84:	c8a080e7          	jalr	-886(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
42033a88:	48068593          	addi	a1,a3,1152
42033a8c:	3c2b7637          	lui	a2,0x3c2b7
42033a90:	20c60613          	addi	a2,a2,524 # 3c2b720c <c7_weights_0+0x240c>
42033a94:	8536                	mv	a0,a3
42033a96:	00006097          	auipc	ra,0x6
42033a9a:	c74080e7          	jalr	-908(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
42033a9e:	3c2b7637          	lui	a2,0x3c2b7
42033aa2:	27c60613          	addi	a2,a2,636 # 3c2b727c <c7_weights_0+0x247c>
42033aa6:	6585                	lui	a1,0x1
42033aa8:	8536                	mv	a0,a3
42033aaa:	00006097          	auipc	ra,0x6
42033aae:	946080e7          	jalr	-1722(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42033ab2:	3c2b7637          	lui	a2,0x3c2b7
42033ab6:	29c60613          	addi	a2,a2,668 # 3c2b729c <c7_weights_0+0x249c>
42033aba:	6585                	lui	a1,0x1
42033abc:	8536                	mv	a0,a3
42033abe:	00006097          	auipc	ra,0x6
42033ac2:	932080e7          	jalr	-1742(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42033ac6:	3c2b7637          	lui	a2,0x3c2b7
42033aca:	26c60613          	addi	a2,a2,620 # 3c2b726c <c7_weights_0+0x246c>
42033ace:	6585                	lui	a1,0x1
42033ad0:	8536                	mv	a0,a3
42033ad2:	00006097          	auipc	ra,0x6
42033ad6:	91e080e7          	jalr	-1762(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42033ada:	3c2b7637          	lui	a2,0x3c2b7
42033ade:	28c60613          	addi	a2,a2,652 # 3c2b728c <c7_weights_0+0x248c>
42033ae2:	6585                	lui	a1,0x1
42033ae4:	8536                	mv	a0,a3
42033ae6:	00006097          	auipc	ra,0x6
42033aea:	90a080e7          	jalr	-1782(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42033aee:	3c2b7637          	lui	a2,0x3c2b7
42033af2:	25c60613          	addi	a2,a2,604 # 3c2b725c <c7_weights_0+0x245c>
42033af6:	04000593          	li	a1,64
42033afa:	00006097          	auipc	ra,0x6
42033afe:	8f6080e7          	jalr	-1802(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
