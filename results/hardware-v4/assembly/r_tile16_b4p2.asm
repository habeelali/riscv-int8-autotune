
hardware/esp32c3-v4/build/riscv_int8_generalize.elf:     file format elf32-littleriscv


Disassembly of section .iram0.text:

Disassembly of section .flash.text:

42038230 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E>:
42038230:	7111                	addi	sp,sp,-256
42038232:	df86                	sw	ra,252(sp)
42038234:	dda2                	sw	s0,248(sp)
42038236:	dba6                	sw	s1,244(sp)
42038238:	d9ca                	sw	s2,240(sp)
4203823a:	d7ce                	sw	s3,236(sp)
4203823c:	d5d2                	sw	s4,232(sp)
4203823e:	d3d6                	sw	s5,228(sp)
42038240:	d1da                	sw	s6,224(sp)
42038242:	cfde                	sw	s7,220(sp)
42038244:	cde2                	sw	s8,216(sp)
42038246:	cbe6                	sw	s9,212(sp)
42038248:	c9ea                	sw	s10,208(sp)
4203824a:	c7ee                	sw	s11,204(sp)
4203824c:	6809                	lui	a6,0x2
4203824e:	3b080813          	addi	a6,a6,944 # 23b0 <RvExcFrameSize+0x231c>
42038252:	41010133          	sub	sp,sp,a6
42038256:	8d3e                	mv	s10,a5
42038258:	d03a                	sw	a4,32(sp)
4203825a:	ce36                	sw	a3,28(sp)
4203825c:	cc32                	sw	a2,24(sp)
4203825e:	8bae                	mv	s7,a1
42038260:	da2a                	sw	a0,52(sp)
42038262:	40000c37          	lui	s8,0x40000
42038266:	4525                	li	a0,9
42038268:	00a51613          	slli	a2,a0,0xa
4203826c:	18e8                	addi	a0,sp,124
4203826e:	4581                	li	a1,0
42038270:	de32                	sw	a2,60(sp)
42038272:	fdfc8097          	auipc	ra,0xfdfc8
42038276:	0e2080e7          	jalr	226(ra) # 40000354 <memset>
4203827a:	4301                	li	t1,0
4203827c:	0b8d                	addi	s7,s7,3
4203827e:	ca5e                	sw	s7,20(sp)
42038280:	450d                	li	a0,3
42038282:	04000693          	li	a3,64
42038286:	55dd                	li	a1,-9
42038288:	6609                	lui	a2,0x2
4203828a:	05a2                	slli	a1,a1,0x8
4203828c:	d42e                	sw	a1,40(sp)
4203828e:	f8060593          	addi	a1,a2,-128 # 1f80 <RvExcFrameSize+0x1eec>
42038292:	dc2e                	sw	a1,56(sp)
42038294:	40668633          	sub	a2,a3,t1
42038298:	45c1                	li	a1,16
4203829a:	c82a                	sw	a0,16(sp)
4203829c:	d0b2                	sw	a2,96(sp)
4203829e:	00b66463          	bltu	a2,a1,420382a6 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x76>
420382a2:	4541                	li	a0,16
420382a4:	d0aa                	sw	a0,96(sp)
420382a6:	da9a                	sw	t1,116(sp)
420382a8:	46d31c63          	bne	t1,a3,42038720 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x4f0>
420382ac:	4081                	li	ra,0
420382ae:	5586                	lw	a1,96(sp)
420382b0:	8185                	srli	a1,a1,0x1
420382b2:	453d                	li	a0,15
420382b4:	fff58613          	addi	a2,a1,-1
420382b8:	00c5b5b3          	sltu	a1,a1,a2
420382bc:	15fd                	addi	a1,a1,-1
420382be:	8df1                	and	a1,a1,a2
420382c0:	d22e                	sw	a1,36(sp)
420382c2:	4611                	li	a2,4
420382c4:	45d2                	lw	a1,20(sp)
420382c6:	dcae                	sw	a1,120(sp)
420382c8:	a005                	j	420382e8 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0xb8>
420382ca:	55c2                	lw	a1,48(sp)
420382cc:	80ae                	mv	ra,a1
420382ce:	0591                	addi	a1,a1,4
420382d0:	56b2                	lw	a3,44(sp)
420382d2:	fff68513          	addi	a0,a3,-1
420382d6:	5666                	lw	a2,120(sp)
420382d8:	7ff60613          	addi	a2,a2,2047
420382dc:	10160613          	addi	a2,a2,257
420382e0:	dcb2                	sw	a2,120(sp)
420382e2:	862e                	mv	a2,a1
420382e4:	78068363          	beqz	a3,42038a6a <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x83a>
420382e8:	24000593          	li	a1,576
420382ec:	02b086b3          	mul	a3,ra,a1
420382f0:	85aa                	mv	a1,a0
420382f2:	5522                	lw	a0,40(sp)
420382f4:	02a6f3e3          	bgeu	a3,a0,42038b1a <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8ea>
420382f8:	d62e                	sw	a1,44(sp)
420382fa:	d832                	sw	a2,48(sp)
420382fc:	04000513          	li	a0,64
42038300:	fca305e3          	beq	t1,a0,420382ca <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x9a>
42038304:	03f00513          	li	a0,63
42038308:	081569e3          	bltu	a0,ra,42038b9a <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x96a>
4203830c:	03d00513          	li	a0,61
42038310:	061569e3          	bltu	a0,ra,42038b82 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x952>
42038314:	4b81                	li	s7,0
42038316:	4f81                	li	t6,0
42038318:	04008513          	addi	a0,ra,64
4203831c:	d0aa                	sw	a0,96(sp)
4203831e:	00209513          	slli	a0,ra,0x2
42038322:	4f09                	li	t5,2
42038324:	4762                	lw	a4,24(sp)
42038326:	00a705b3          	add	a1,a4,a0
4203832a:	d8ae                	sw	a1,112(sp)
4203832c:	00450593          	addi	a1,a0,4
42038330:	00850913          	addi	s2,a0,8
42038334:	00c50893          	addi	a7,a0,12
42038338:	46f2                	lw	a3,28(sp)
4203833a:	00a68633          	add	a2,a3,a0
4203833e:	ceb2                	sw	a2,92(sp)
42038340:	5602                	lw	a2,32(sp)
42038342:	9532                	add	a0,a0,a2
42038344:	ccaa                	sw	a0,88(sp)
42038346:	00b70533          	add	a0,a4,a1
4203834a:	d6aa                	sw	a0,108(sp)
4203834c:	01270533          	add	a0,a4,s2
42038350:	d4aa                	sw	a0,104(sp)
42038352:	9746                	add	a4,a4,a7
42038354:	d2ba                	sw	a4,100(sp)
42038356:	00b68533          	add	a0,a3,a1
4203835a:	caaa                	sw	a0,84(sp)
4203835c:	95b2                	add	a1,a1,a2
4203835e:	c8ae                	sw	a1,80(sp)
42038360:	01268533          	add	a0,a3,s2
42038364:	c6aa                	sw	a0,76(sp)
42038366:	9932                	add	s2,s2,a2
42038368:	c4ca                	sw	s2,72(sp)
4203836a:	96c6                	add	a3,a3,a7
4203836c:	c2b6                	sw	a3,68(sp)
4203836e:	9646                	add	a2,a2,a7
42038370:	c0b2                	sw	a2,64(sp)
42038372:	07c10293          	addi	t0,sp,124
42038376:	5512                	lw	a0,36(sp)
42038378:	a821                	j	42038390 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x160>
4203837a:	00ad81a3          	sb	a0,3(s11)
4203837e:	8ffa                	mv	t6,t5
42038380:	0f09                	addi	t5,t5,2
42038382:	fffb0513          	addi	a0,s6,-1
42038386:	48028293          	addi	t0,t0,1152
4203838a:	0b85                	addi	s7,s7,1
4203838c:	f20b0fe3          	beqz	s6,420382ca <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x9a>
42038390:	8b2a                	mv	s6,a0
42038392:	24000513          	li	a0,576
42038396:	02af8733          	mul	a4,t6,a0
4203839a:	24070693          	addi	a3,a4,576
4203839e:	dbf00513          	li	a0,-577
420383a2:	6ee56f63          	bltu	a0,a4,42038aa0 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x870>
420383a6:	5572                	lw	a0,60(sp)
420383a8:	70d56663          	bltu	a0,a3,42038ab4 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x884>
420383ac:	48070593          	addi	a1,a4,1152
420383b0:	70d5ed63          	bltu	a1,a3,42038aca <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x89a>
420383b4:	5562                	lw	a0,56(sp)
420383b6:	72e56363          	bltu	a0,a4,42038adc <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8ac>
420383ba:	5546                	lw	a0,112(sp)
420383bc:	00052e03          	lw	t3,0(a0)
420383c0:	5536                	lw	a0,108(sp)
420383c2:	00052d83          	lw	s11,0(a0)
420383c6:	5526                	lw	a0,104(sp)
420383c8:	00052a83          	lw	s5,0(a0)
420383cc:	5516                	lw	a0,100(sp)
420383ce:	00052a03          	lw	s4,0(a0)
420383d2:	48000513          	li	a0,1152
420383d6:	02ab8533          	mul	a0,s7,a0
420383da:	18ec                	addi	a1,sp,124
420383dc:	952e                	add	a0,a0,a1
420383de:	24050813          	addi	a6,a0,576
420383e2:	5666                	lw	a2,120(sp)
420383e4:	8416                	mv	s0,t0
420383e6:	88f2                	mv	a7,t3
420383e8:	83ee                	mv	t2,s11
420383ea:	8ed6                	mv	t4,s5
420383ec:	8952                	mv	s2,s4
420383ee:	00040503          	lb	a0,0(s0)
420383f2:	24040683          	lb	a3,576(s0)
420383f6:	ffd60583          	lb	a1,-3(a2)
420383fa:	ffe60483          	lb	s1,-2(a2)
420383fe:	fff60783          	lb	a5,-1(a2)
42038402:	00060703          	lb	a4,0(a2)
42038406:	02a589b3          	mul	s3,a1,a0
4203840a:	02d585b3          	mul	a1,a1,a3
4203840e:	02a48cb3          	mul	s9,s1,a0
42038412:	02d484b3          	mul	s1,s1,a3
42038416:	02d78333          	mul	t1,a5,a3
4203841a:	02a787b3          	mul	a5,a5,a0
4203841e:	02a70533          	mul	a0,a4,a0
42038422:	02d706b3          	mul	a3,a4,a3
42038426:	9e4e                	add	t3,t3,s3
42038428:	98ae                	add	a7,a7,a1
4203842a:	9de6                	add	s11,s11,s9
4203842c:	93a6                	add	t2,t2,s1
4203842e:	0405                	addi	s0,s0,1
42038430:	9e9a                	add	t4,t4,t1
42038432:	9abe                	add	s5,s5,a5
42038434:	9a2a                	add	s4,s4,a0
42038436:	9936                	add	s2,s2,a3
42038438:	0611                	addi	a2,a2,4
4203843a:	fb041ae3          	bne	s0,a6,420383ee <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x1be>
4203843e:	5356                	lw	t1,116(sp)
42038440:	006f86b3          	add	a3,t6,t1
42038444:	00669c93          	slli	s9,a3,0x6
42038448:	001ce533          	or	a0,s9,ra
4203844c:	00c55613          	srli	a2,a0,0xc
42038450:	6e061c63          	bnez	a2,42038b48 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x918>
42038454:	45f6                	lw	a1,92(sp)
42038456:	0005af83          	lw	t6,0(a1)
4203845a:	45e6                	lw	a1,88(sp)
4203845c:	4180                	lw	s0,0(a1)
4203845e:	03cf9633          	mulh	a2,t6,t3
42038462:	03cf8733          	mul	a4,t6,t3
42038466:	018706b3          	add	a3,a4,s8
4203846a:	00e6b733          	sltu	a4,a3,a4
4203846e:	963a                	add	a2,a2,a4
42038470:	fff44713          	not	a4,s0
42038474:	40800833          	neg	a6,s0
42038478:	4585                	li	a1,1
4203847a:	00e59e33          	sll	t3,a1,a4
4203847e:	82fd                	srli	a3,a3,0x1f
42038480:	0606                	slli	a2,a2,0x1
42038482:	8ed1                	or	a3,a3,a2
42038484:	867d                	srai	a2,a2,0x1f
42038486:	96f2                	add	a3,a3,t3
42038488:	9636                	add	a2,a2,a3
4203848a:	41065433          	sra	s0,a2,a6
4203848e:	1479                	addi	s0,s0,-2
42038490:	07f00613          	li	a2,127
42038494:	02c45163          	bge	s0,a2,420384b6 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x286>
42038498:	f8000613          	li	a2,-128
4203849c:	02865363          	bge	a2,s0,420384c2 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x292>
420384a0:	00ad09b3          	add	s3,s10,a0
420384a4:	5606                	lw	a2,96(sp)
420384a6:	00ccecb3          	or	s9,s9,a2
420384aa:	00ccd613          	srli	a2,s9,0xc
420384ae:	00898023          	sb	s0,0(s3)
420384b2:	c60d                	beqz	a2,420384dc <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x2ac>
420384b4:	a541                	j	42038b34 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x904>
420384b6:	07f00413          	li	s0,127
420384ba:	f8000613          	li	a2,-128
420384be:	fe8641e3          	blt	a2,s0,420384a0 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x270>
420384c2:	f8000413          	li	s0,-128
420384c6:	00ad09b3          	add	s3,s10,a0
420384ca:	5606                	lw	a2,96(sp)
420384cc:	00ccecb3          	or	s9,s9,a2
420384d0:	00ccd613          	srli	a2,s9,0xc
420384d4:	00898023          	sb	s0,0(s3)
420384d8:	64061e63          	bnez	a2,42038b34 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x904>
420384dc:	01f87613          	andi	a2,a6,31
420384e0:	031f96b3          	mulh	a3,t6,a7
420384e4:	031f8733          	mul	a4,t6,a7
420384e8:	018707b3          	add	a5,a4,s8
420384ec:	00e7b733          	sltu	a4,a5,a4
420384f0:	83fd                	srli	a5,a5,0x1f
420384f2:	96ba                	add	a3,a3,a4
420384f4:	0686                	slli	a3,a3,0x1
420384f6:	8fd5                	or	a5,a5,a3
420384f8:	86fd                	srai	a3,a3,0x1f
420384fa:	97f2                	add	a5,a5,t3
420384fc:	96be                	add	a3,a3,a5
420384fe:	40c6d6b3          	sra	a3,a3,a2
42038502:	16f9                	addi	a3,a3,-2
42038504:	07f00813          	li	a6,127
42038508:	0106c463          	blt	a3,a6,42038510 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x2e0>
4203850c:	07f00693          	li	a3,127
42038510:	f8000f93          	li	t6,-128
42038514:	00dfc463          	blt	t6,a3,4203851c <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x2ec>
42038518:	f8000693          	li	a3,-128
4203851c:	4656                	lw	a2,84(sp)
4203851e:	4210                	lw	a2,0(a2)
42038520:	4746                	lw	a4,80(sp)
42038522:	4318                	lw	a4,0(a4)
42038524:	03b61433          	mulh	s0,a2,s11
42038528:	03b604b3          	mul	s1,a2,s11
4203852c:	018487b3          	add	a5,s1,s8
42038530:	0097b4b3          	sltu	s1,a5,s1
42038534:	94a2                	add	s1,s1,s0
42038536:	fff74413          	not	s0,a4
4203853a:	00859433          	sll	s0,a1,s0
4203853e:	83fd                	srli	a5,a5,0x1f
42038540:	0486                	slli	s1,s1,0x1
42038542:	8fc5                	or	a5,a5,s1
42038544:	84fd                	srai	s1,s1,0x1f
42038546:	97a2                	add	a5,a5,s0
42038548:	97a6                	add	a5,a5,s1
4203854a:	019d0db3          	add	s11,s10,s9
4203854e:	40e00733          	neg	a4,a4
42038552:	40e7de33          	sra	t3,a5,a4
42038556:	1e79                	addi	t3,t3,-2
42038558:	00dd8023          	sb	a3,0(s11)
4203855c:	010e4463          	blt	t3,a6,42038564 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x334>
42038560:	07f00e13          	li	t3,127
42038564:	01cfc463          	blt	t6,t3,4203856c <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x33c>
42038568:	f8000e13          	li	t3,-128
4203856c:	027616b3          	mulh	a3,a2,t2
42038570:	02760633          	mul	a2,a2,t2
42038574:	018607b3          	add	a5,a2,s8
42038578:	00c7b633          	sltu	a2,a5,a2
4203857c:	9636                	add	a2,a2,a3
4203857e:	83fd                	srli	a5,a5,0x1f
42038580:	0606                	slli	a2,a2,0x1
42038582:	8fd1                	or	a5,a5,a2
42038584:	867d                	srai	a2,a2,0x1f
42038586:	97a2                	add	a5,a5,s0
42038588:	963e                	add	a2,a2,a5
4203858a:	40e65633          	sra	a2,a2,a4
4203858e:	1679                	addi	a2,a2,-2
42038590:	01c980a3          	sb	t3,1(s3)
42038594:	01065f63          	bge	a2,a6,420385b2 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x382>
42038598:	02cfd163          	bge	t6,a2,420385ba <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x38a>
4203859c:	00cd80a3          	sb	a2,1(s11)
420385a0:	00155613          	srli	a2,a0,0x1
420385a4:	00250693          	addi	a3,a0,2
420385a8:	7fe00513          	li	a0,2046
420385ac:	02c57363          	bgeu	a0,a2,420385d2 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x3a2>
420385b0:	ab7d                	j	42038b6e <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x93e>
420385b2:	07f00613          	li	a2,127
420385b6:	fecfc3e3          	blt	t6,a2,4203859c <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x36c>
420385ba:	f8000613          	li	a2,-128
420385be:	00cd80a3          	sb	a2,1(s11)
420385c2:	00155613          	srli	a2,a0,0x1
420385c6:	00250693          	addi	a3,a0,2
420385ca:	7fe00513          	li	a0,2046
420385ce:	5ac56063          	bltu	a0,a2,42038b6e <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x93e>
420385d2:	4536                	lw	a0,76(sp)
420385d4:	4110                	lw	a2,0(a0)
420385d6:	4526                	lw	a0,72(sp)
420385d8:	4108                	lw	a0,0(a0)
420385da:	03561733          	mulh	a4,a2,s5
420385de:	035607b3          	mul	a5,a2,s5
420385e2:	01878433          	add	s0,a5,s8
420385e6:	00f437b3          	sltu	a5,s0,a5
420385ea:	973e                	add	a4,a4,a5
420385ec:	fff54493          	not	s1,a0
420385f0:	40a007b3          	neg	a5,a0
420385f4:	00959533          	sll	a0,a1,s1
420385f8:	807d                	srli	s0,s0,0x1f
420385fa:	0706                	slli	a4,a4,0x1
420385fc:	8c59                	or	s0,s0,a4
420385fe:	877d                	srai	a4,a4,0x1f
42038600:	942a                	add	s0,s0,a0
42038602:	9722                	add	a4,a4,s0
42038604:	40f75733          	sra	a4,a4,a5
42038608:	1779                	addi	a4,a4,-2
4203860a:	07f00493          	li	s1,127
4203860e:	02975263          	bge	a4,s1,42038632 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x402>
42038612:	f8000493          	li	s1,-128
42038616:	02e4d463          	bge	s1,a4,4203863e <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x40e>
4203861a:	96ea                	add	a3,a3,s10
4203861c:	00e68023          	sb	a4,0(a3)
42038620:	001cd693          	srli	a3,s9,0x1
42038624:	002c8713          	addi	a4,s9,2
42038628:	7fe00593          	li	a1,2046
4203862c:	02d5f663          	bgeu	a1,a3,42038658 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x428>
42038630:	a32d                	j	42038b5a <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x92a>
42038632:	07f00713          	li	a4,127
42038636:	f8000493          	li	s1,-128
4203863a:	fee4c0e3          	blt	s1,a4,4203861a <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x3ea>
4203863e:	f8000713          	li	a4,-128
42038642:	96ea                	add	a3,a3,s10
42038644:	00e68023          	sb	a4,0(a3)
42038648:	001cd693          	srli	a3,s9,0x1
4203864c:	002c8713          	addi	a4,s9,2
42038650:	7fe00593          	li	a1,2046
42038654:	50d5e363          	bltu	a1,a3,42038b5a <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x92a>
42038658:	8bfd                	andi	a5,a5,31
4203865a:	03d616b3          	mulh	a3,a2,t4
4203865e:	03d60633          	mul	a2,a2,t4
42038662:	018604b3          	add	s1,a2,s8
42038666:	00c4b633          	sltu	a2,s1,a2
4203866a:	80fd                	srli	s1,s1,0x1f
4203866c:	9636                	add	a2,a2,a3
4203866e:	0606                	slli	a2,a2,0x1
42038670:	8cd1                	or	s1,s1,a2
42038672:	867d                	srai	a2,a2,0x1f
42038674:	9526                	add	a0,a0,s1
42038676:	9532                	add	a0,a0,a2
42038678:	40f55633          	sra	a2,a0,a5
4203867c:	1679                	addi	a2,a2,-2
4203867e:	07f00813          	li	a6,127
42038682:	01064463          	blt	a2,a6,4203868a <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x45a>
42038686:	07f00613          	li	a2,127
4203868a:	4885                	li	a7,1
4203868c:	f8000793          	li	a5,-128
42038690:	00c7c463          	blt	a5,a2,42038698 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x468>
42038694:	f8000613          	li	a2,-128
42038698:	4516                	lw	a0,68(sp)
4203869a:	4108                	lw	a0,0(a0)
4203869c:	4586                	lw	a1,64(sp)
4203869e:	4184                	lw	s1,0(a1)
420386a0:	03451433          	mulh	s0,a0,s4
420386a4:	034505b3          	mul	a1,a0,s4
420386a8:	018586b3          	add	a3,a1,s8
420386ac:	00b6b5b3          	sltu	a1,a3,a1
420386b0:	942e                	add	s0,s0,a1
420386b2:	fff4c593          	not	a1,s1
420386b6:	00b895b3          	sll	a1,a7,a1
420386ba:	82fd                	srli	a3,a3,0x1f
420386bc:	0406                	slli	s0,s0,0x1
420386be:	8ec1                	or	a3,a3,s0
420386c0:	847d                	srai	s0,s0,0x1f
420386c2:	96ae                	add	a3,a3,a1
420386c4:	96a2                	add	a3,a3,s0
420386c6:	976a                	add	a4,a4,s10
420386c8:	409004b3          	neg	s1,s1
420386cc:	4096d433          	sra	s0,a3,s1
420386d0:	1479                	addi	s0,s0,-2
420386d2:	00c70023          	sb	a2,0(a4)
420386d6:	01044463          	blt	s0,a6,420386de <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x4ae>
420386da:	07f00413          	li	s0,127
420386de:	0087c463          	blt	a5,s0,420386e6 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x4b6>
420386e2:	f8000413          	li	s0,-128
420386e6:	03251633          	mulh	a2,a0,s2
420386ea:	03250533          	mul	a0,a0,s2
420386ee:	018506b3          	add	a3,a0,s8
420386f2:	00a6b533          	sltu	a0,a3,a0
420386f6:	9532                	add	a0,a0,a2
420386f8:	82fd                	srli	a3,a3,0x1f
420386fa:	0506                	slli	a0,a0,0x1
420386fc:	8ec9                	or	a3,a3,a0
420386fe:	857d                	srai	a0,a0,0x1f
42038700:	95b6                	add	a1,a1,a3
42038702:	952e                	add	a0,a0,a1
42038704:	40955533          	sra	a0,a0,s1
42038708:	1579                	addi	a0,a0,-2
4203870a:	008981a3          	sb	s0,3(s3)
4203870e:	01054463          	blt	a0,a6,42038716 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x4e6>
42038712:	07f00513          	li	a0,127
42038716:	c6a7c2e3          	blt	a5,a0,4203837a <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x14a>
4203871a:	f8000513          	li	a0,-128
4203871e:	b9b1                	j	4203837a <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x14a>
42038720:	4601                	li	a2,0
42038722:	24000513          	li	a0,576
42038726:	5586                	lw	a1,96(sp)
42038728:	02a587b3          	mul	a5,a1,a0
4203872c:	18e4                	addi	s1,sp,124
4203872e:	17c10b93          	addi	s7,sp,380
42038732:	1d68                	addi	a0,sp,700
42038734:	a82d                	j	4203876e <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x53e>
42038736:	08000593          	li	a1,128
4203873a:	04000613          	li	a2,64
4203873e:	8536                	mv	a0,a3
42038740:	fdfc8097          	auipc	ra,0xfdfc8
42038744:	c14080e7          	jalr	-1004(ra) # 40000354 <memset>
42038748:	5796                	lw	a5,100(sp)
4203874a:	dc078793          	addi	a5,a5,-576
4203874e:	240b8b93          	addi	s7,s7,576
42038752:	2407b513          	sltiu	a0,a5,576
42038756:	157d                	addi	a0,a0,-1
42038758:	24057513          	andi	a0,a0,576
4203875c:	54a6                	lw	s1,104(sp)
4203875e:	9526                	add	a0,a0,s1
42038760:	5636                	lw	a2,108(sp)
42038762:	0605                	addi	a2,a2,1
42038764:	5356                	lw	t1,116(sp)
42038766:	24000593          	li	a1,576
4203876a:	b4b7e1e3          	bltu	a5,a1,420382ac <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x7c>
4203876e:	85aa                	mv	a1,a0
42038770:	f00b8693          	addi	a3,s7,-256
42038774:	9332                	add	t1,t1,a2
42038776:	00767913          	andi	s2,a2,7
4203877a:	d89a                	sw	t1,112(sp)
4203877c:	00335993          	srli	s3,t1,0x3
42038780:	fff90513          	addi	a0,s2,-1
42038784:	fff98c93          	addi	s9,s3,-1
42038788:	00999413          	slli	s0,s3,0x9
4203878c:	dc040a13          	addi	s4,s0,-576
42038790:	e0040d93          	addi	s11,s0,-512
42038794:	dcaa                	sw	a0,120(sp)
42038796:	01956533          	or	a0,a0,s9
4203879a:	00691a93          	slli	s5,s2,0x6
4203879e:	4b21                	li	s6,8
420387a0:	d6b2                	sw	a2,108(sp)
420387a2:	d2be                	sw	a5,100(sp)
420387a4:	d4ae                	sw	a1,104(sp)
420387a6:	03657e63          	bgeu	a0,s6,420387e2 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x5b2>
420387aa:	014a8533          	add	a0,s5,s4
420387ae:	01bae733          	or	a4,s5,s11
420387b2:	34a76b63          	bltu	a4,a0,42038b08 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8d8>
420387b6:	6b05                	lui	s6,0x1
420387b8:	32eb6e63          	bltu	s6,a4,42038af4 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8c4>
420387bc:	55d2                	lw	a1,52(sp)
420387be:	95aa                	add	a1,a1,a0
420387c0:	04000613          	li	a2,64
420387c4:	8536                	mv	a0,a3
420387c6:	fdfc8097          	auipc	ra,0xfdfc8
420387ca:	b92080e7          	jalr	-1134(ra) # 40000358 <memcpy>
420387ce:	f40b8693          	addi	a3,s7,-192
420387d2:	040a8713          	addi	a4,s5,64
420387d6:	01470533          	add	a0,a4,s4
420387da:	976e                	add	a4,a4,s11
420387dc:	02a77863          	bgeu	a4,a0,4203880c <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x5dc>
420387e0:	a625                	j	42038b08 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8d8>
420387e2:	08000593          	li	a1,128
420387e6:	04000613          	li	a2,64
420387ea:	8536                	mv	a0,a3
420387ec:	fdfc8097          	auipc	ra,0xfdfc8
420387f0:	b68080e7          	jalr	-1176(ra) # 40000354 <memset>
420387f4:	0d6cfb63          	bgeu	s9,s6,420388ca <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x69a>
420387f8:	04048693          	addi	a3,s1,64
420387fc:	6b05                	lui	s6,0x1
420387fe:	040a8713          	addi	a4,s5,64
42038802:	01470533          	add	a0,a4,s4
42038806:	976e                	add	a4,a4,s11
42038808:	30a76063          	bltu	a4,a0,42038b08 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8d8>
4203880c:	2eeb6463          	bltu	s6,a4,42038af4 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8c4>
42038810:	55d2                	lw	a1,52(sp)
42038812:	95aa                	add	a1,a1,a0
42038814:	04000613          	li	a2,64
42038818:	8536                	mv	a0,a3
4203881a:	fdfc8097          	auipc	ra,0xfdfc8
4203881e:	b3e080e7          	jalr	-1218(ra) # 40000358 <memcpy>
42038822:	0905                	addi	s2,s2,1
42038824:	01996533          	or	a0,s2,s9
42038828:	f80b8693          	addi	a3,s7,-128
4203882c:	45a1                	li	a1,8
4203882e:	0cb57163          	bgeu	a0,a1,420388f0 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x6c0>
42038832:	080a8713          	addi	a4,s5,128
42038836:	01470533          	add	a0,a4,s4
4203883a:	976e                	add	a4,a4,s11
4203883c:	2ca76663          	bltu	a4,a0,42038b08 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8d8>
42038840:	2aeb6a63          	bltu	s6,a4,42038af4 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8c4>
42038844:	55d2                	lw	a1,52(sp)
42038846:	95aa                	add	a1,a1,a0
42038848:	04000613          	li	a2,64
4203884c:	8536                	mv	a0,a3
4203884e:	fdfc8097          	auipc	ra,0xfdfc8
42038852:	b0a080e7          	jalr	-1270(ra) # 40000358 <memcpy>
42038856:	fc040493          	addi	s1,s0,-64
4203885a:	5566                	lw	a0,120(sp)
4203885c:	01356533          	or	a0,a0,s3
42038860:	fc0b8693          	addi	a3,s7,-64
42038864:	45a1                	li	a1,8
42038866:	0ab56863          	bltu	a0,a1,42038916 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x6e6>
4203886a:	08000593          	li	a1,128
4203886e:	04000613          	li	a2,64
42038872:	8536                	mv	a0,a3
42038874:	fdfc8097          	auipc	ra,0xfdfc8
42038878:	ae0080e7          	jalr	-1312(ra) # 40000354 <memset>
4203887c:	04000513          	li	a0,64
42038880:	55c6                	lw	a1,112(sp)
42038882:	0ca5f063          	bgeu	a1,a0,42038942 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x712>
42038886:	008a8533          	add	a0,s5,s0
4203888a:	04050713          	addi	a4,a0,64
4203888e:	26eb6363          	bltu	s6,a4,42038af4 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8c4>
42038892:	008ae5b3          	or	a1,s5,s0
42038896:	5552                	lw	a0,52(sp)
42038898:	95aa                	add	a1,a1,a0
4203889a:	04000613          	li	a2,64
4203889e:	855e                	mv	a0,s7
420388a0:	fdfc8097          	auipc	ra,0xfdfc8
420388a4:	ab8080e7          	jalr	-1352(ra) # 40000358 <memcpy>
420388a8:	01396533          	or	a0,s2,s3
420388ac:	040b8693          	addi	a3,s7,64
420388b0:	45a1                	li	a1,8
420388b2:	0ab56863          	bltu	a0,a1,42038962 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x732>
420388b6:	08000593          	li	a1,128
420388ba:	04000613          	li	a2,64
420388be:	8536                	mv	a0,a3
420388c0:	fdfc8097          	auipc	ra,0xfdfc8
420388c4:	a94080e7          	jalr	-1388(ra) # 40000354 <memset>
420388c8:	a87d                	j	42038986 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x756>
420388ca:	f40b8513          	addi	a0,s7,-192
420388ce:	08000593          	li	a1,128
420388d2:	04000613          	li	a2,64
420388d6:	fdfc8097          	auipc	ra,0xfdfc8
420388da:	a7e080e7          	jalr	-1410(ra) # 40000354 <memset>
420388de:	6b05                	lui	s6,0x1
420388e0:	0905                	addi	s2,s2,1
420388e2:	01996533          	or	a0,s2,s9
420388e6:	f80b8693          	addi	a3,s7,-128
420388ea:	45a1                	li	a1,8
420388ec:	f4b563e3          	bltu	a0,a1,42038832 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x602>
420388f0:	08000593          	li	a1,128
420388f4:	04000613          	li	a2,64
420388f8:	8536                	mv	a0,a3
420388fa:	fdfc8097          	auipc	ra,0xfdfc8
420388fe:	a5a080e7          	jalr	-1446(ra) # 40000354 <memset>
42038902:	fc040493          	addi	s1,s0,-64
42038906:	5566                	lw	a0,120(sp)
42038908:	01356533          	or	a0,a0,s3
4203890c:	fc0b8693          	addi	a3,s7,-64
42038910:	45a1                	li	a1,8
42038912:	f4b57ce3          	bgeu	a0,a1,4203886a <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x63a>
42038916:	009a8533          	add	a0,s5,s1
4203891a:	008ae733          	or	a4,s5,s0
4203891e:	1ea76563          	bltu	a4,a0,42038b08 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8d8>
42038922:	1ceb6963          	bltu	s6,a4,42038af4 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8c4>
42038926:	55d2                	lw	a1,52(sp)
42038928:	95aa                	add	a1,a1,a0
4203892a:	04000613          	li	a2,64
4203892e:	8536                	mv	a0,a3
42038930:	fdfc8097          	auipc	ra,0xfdfc8
42038934:	a28080e7          	jalr	-1496(ra) # 40000358 <memcpy>
42038938:	04000513          	li	a0,64
4203893c:	55c6                	lw	a1,112(sp)
4203893e:	f4a5e4e3          	bltu	a1,a0,42038886 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x656>
42038942:	08000593          	li	a1,128
42038946:	04000613          	li	a2,64
4203894a:	855e                	mv	a0,s7
4203894c:	fdfc8097          	auipc	ra,0xfdfc8
42038950:	a08080e7          	jalr	-1528(ra) # 40000354 <memset>
42038954:	01396533          	or	a0,s2,s3
42038958:	040b8693          	addi	a3,s7,64
4203895c:	45a1                	li	a1,8
4203895e:	f4b57ce3          	bgeu	a0,a1,420388b6 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x686>
42038962:	080a8713          	addi	a4,s5,128
42038966:	00970533          	add	a0,a4,s1
4203896a:	9722                	add	a4,a4,s0
4203896c:	18a76e63          	bltu	a4,a0,42038b08 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8d8>
42038970:	18eb6263          	bltu	s6,a4,42038af4 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8c4>
42038974:	55d2                	lw	a1,52(sp)
42038976:	95aa                	add	a1,a1,a0
42038978:	04000613          	li	a2,64
4203897c:	8536                	mv	a0,a3
4203897e:	fdfc8097          	auipc	ra,0xfdfc8
42038982:	9da080e7          	jalr	-1574(ra) # 40000358 <memcpy>
42038986:	00198493          	addi	s1,s3,1
4203898a:	1c040993          	addi	s3,s0,448
4203898e:	20040413          	addi	s0,s0,512
42038992:	5566                	lw	a0,120(sp)
42038994:	8d45                	or	a0,a0,s1
42038996:	080b8693          	addi	a3,s7,128
4203899a:	45a1                	li	a1,8
4203899c:	04b57b63          	bgeu	a0,a1,420389f2 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x7c2>
420389a0:	013a8533          	add	a0,s5,s3
420389a4:	008ae733          	or	a4,s5,s0
420389a8:	16a76063          	bltu	a4,a0,42038b08 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8d8>
420389ac:	14eb6463          	bltu	s6,a4,42038af4 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8c4>
420389b0:	55d2                	lw	a1,52(sp)
420389b2:	95aa                	add	a1,a1,a0
420389b4:	04000613          	li	a2,64
420389b8:	8536                	mv	a0,a3
420389ba:	fdfc8097          	auipc	ra,0xfdfc8
420389be:	99e080e7          	jalr	-1634(ra) # 40000358 <memcpy>
420389c2:	0c0b8693          	addi	a3,s7,192
420389c6:	03800513          	li	a0,56
420389ca:	55c6                	lw	a1,112(sp)
420389cc:	04a5e363          	bltu	a1,a0,42038a12 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x7e2>
420389d0:	08000593          	li	a1,128
420389d4:	04000613          	li	a2,64
420389d8:	8536                	mv	a0,a3
420389da:	fdfc8097          	auipc	ra,0xfdfc8
420389de:	97a080e7          	jalr	-1670(ra) # 40000354 <memset>
420389e2:	00996533          	or	a0,s2,s1
420389e6:	100b8693          	addi	a3,s7,256
420389ea:	45a1                	li	a1,8
420389ec:	d4b575e3          	bgeu	a0,a1,42038736 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x506>
420389f0:	a891                	j	42038a44 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x814>
420389f2:	08000593          	li	a1,128
420389f6:	04000613          	li	a2,64
420389fa:	8536                	mv	a0,a3
420389fc:	fdfc8097          	auipc	ra,0xfdfc8
42038a00:	958080e7          	jalr	-1704(ra) # 40000354 <memset>
42038a04:	0c0b8693          	addi	a3,s7,192
42038a08:	03800513          	li	a0,56
42038a0c:	55c6                	lw	a1,112(sp)
42038a0e:	fca5f1e3          	bgeu	a1,a0,420389d0 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x7a0>
42038a12:	040a8713          	addi	a4,s5,64
42038a16:	01370533          	add	a0,a4,s3
42038a1a:	9722                	add	a4,a4,s0
42038a1c:	0f346663          	bltu	s0,s3,42038b08 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8d8>
42038a20:	0ceb6a63          	bltu	s6,a4,42038af4 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8c4>
42038a24:	55d2                	lw	a1,52(sp)
42038a26:	95aa                	add	a1,a1,a0
42038a28:	04000613          	li	a2,64
42038a2c:	8536                	mv	a0,a3
42038a2e:	fdfc8097          	auipc	ra,0xfdfc8
42038a32:	92a080e7          	jalr	-1750(ra) # 40000358 <memcpy>
42038a36:	00996533          	or	a0,s2,s1
42038a3a:	100b8693          	addi	a3,s7,256
42038a3e:	45a1                	li	a1,8
42038a40:	ceb57be3          	bgeu	a0,a1,42038736 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x506>
42038a44:	080a8713          	addi	a4,s5,128
42038a48:	01370533          	add	a0,a4,s3
42038a4c:	9722                	add	a4,a4,s0
42038a4e:	0aa76d63          	bltu	a4,a0,42038b08 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8d8>
42038a52:	0aeb6163          	bltu	s6,a4,42038af4 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x8c4>
42038a56:	55d2                	lw	a1,52(sp)
42038a58:	95aa                	add	a1,a1,a0
42038a5a:	04000613          	li	a2,64
42038a5e:	8536                	mv	a0,a3
42038a60:	fdfc8097          	auipc	ra,0xfdfc8
42038a64:	8f8080e7          	jalr	-1800(ra) # 40000358 <memcpy>
42038a68:	b1c5                	j	42038748 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x518>
42038a6a:	45c2                	lw	a1,16(sp)
42038a6c:	fff58513          	addi	a0,a1,-1
42038a70:	0341                	addi	t1,t1,16
42038a72:	04000693          	li	a3,64
42038a76:	80059fe3          	bnez	a1,42038294 <_ZN7kernels13r_tile16_b4p27layer_93run17h9dee09d145e06bc1E+0x64>
42038a7a:	6509                	lui	a0,0x2
42038a7c:	3b050513          	addi	a0,a0,944 # 23b0 <RvExcFrameSize+0x231c>
42038a80:	912a                	add	sp,sp,a0
42038a82:	50fe                	lw	ra,252(sp)
42038a84:	546e                	lw	s0,248(sp)
42038a86:	54de                	lw	s1,244(sp)
42038a88:	594e                	lw	s2,240(sp)
42038a8a:	59be                	lw	s3,236(sp)
42038a8c:	5a2e                	lw	s4,232(sp)
42038a8e:	5a9e                	lw	s5,228(sp)
42038a90:	5b0e                	lw	s6,224(sp)
42038a92:	4bfe                	lw	s7,220(sp)
42038a94:	4c6e                	lw	s8,216(sp)
42038a96:	4cde                	lw	s9,212(sp)
42038a98:	4d4e                	lw	s10,208(sp)
42038a9a:	4dbe                	lw	s11,204(sp)
42038a9c:	6111                	addi	sp,sp,256
42038a9e:	8082                	ret
42038aa0:	3c2b7637          	lui	a2,0x3c2b7
42038aa4:	7bc60613          	addi	a2,a2,1980 # 3c2b77bc <c7_weights_0+0x29bc>
42038aa8:	853a                	mv	a0,a4
42038aaa:	85b6                	mv	a1,a3
42038aac:	00001097          	auipc	ra,0x1
42038ab0:	c5e080e7          	jalr	-930(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
42038ab4:	3c2b7637          	lui	a2,0x3c2b7
42038ab8:	7bc60613          	addi	a2,a2,1980 # 3c2b77bc <c7_weights_0+0x29bc>
42038abc:	45a5                	li	a1,9
42038abe:	05aa                	slli	a1,a1,0xa
42038ac0:	8536                	mv	a0,a3
42038ac2:	00001097          	auipc	ra,0x1
42038ac6:	c38080e7          	jalr	-968(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
42038aca:	3c2b7637          	lui	a2,0x3c2b7
42038ace:	7cc60613          	addi	a2,a2,1996 # 3c2b77cc <c7_weights_0+0x29cc>
42038ad2:	8536                	mv	a0,a3
42038ad4:	00001097          	auipc	ra,0x1
42038ad8:	c36080e7          	jalr	-970(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
42038adc:	3c2b7637          	lui	a2,0x3c2b7
42038ae0:	7cc60613          	addi	a2,a2,1996 # 3c2b77cc <c7_weights_0+0x29cc>
42038ae4:	46a5                	li	a3,9
42038ae6:	06aa                	slli	a3,a3,0xa
42038ae8:	852e                	mv	a0,a1
42038aea:	85b6                	mv	a1,a3
42038aec:	00001097          	auipc	ra,0x1
42038af0:	c0e080e7          	jalr	-1010(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
42038af4:	3c2b8637          	lui	a2,0x3c2b8
42038af8:	83c60613          	addi	a2,a2,-1988 # 3c2b783c <c7_weights_0+0x2a3c>
42038afc:	6585                	lui	a1,0x1
42038afe:	853a                	mv	a0,a4
42038b00:	00001097          	auipc	ra,0x1
42038b04:	bfa080e7          	jalr	-1030(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
42038b08:	3c2b8637          	lui	a2,0x3c2b8
42038b0c:	83c60613          	addi	a2,a2,-1988 # 3c2b783c <c7_weights_0+0x2a3c>
42038b10:	85ba                	mv	a1,a4
42038b12:	00001097          	auipc	ra,0x1
42038b16:	bf8080e7          	jalr	-1032(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
42038b1a:	7ff68513          	addi	a0,a3,2047
42038b1e:	10150593          	addi	a1,a0,257
42038b22:	3c2b7637          	lui	a2,0x3c2b7
42038b26:	7ac60613          	addi	a2,a2,1964 # 3c2b77ac <c7_weights_0+0x29ac>
42038b2a:	8536                	mv	a0,a3
42038b2c:	00001097          	auipc	ra,0x1
42038b30:	bde080e7          	jalr	-1058(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
42038b34:	3c2b8637          	lui	a2,0x3c2b8
42038b38:	80c60613          	addi	a2,a2,-2036 # 3c2b780c <c7_weights_0+0x2a0c>
42038b3c:	6585                	lui	a1,0x1
42038b3e:	8566                	mv	a0,s9
42038b40:	00001097          	auipc	ra,0x1
42038b44:	8b0080e7          	jalr	-1872(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42038b48:	3c2b7637          	lui	a2,0x3c2b7
42038b4c:	7fc60613          	addi	a2,a2,2044 # 3c2b77fc <c7_weights_0+0x29fc>
42038b50:	6585                	lui	a1,0x1
42038b52:	00001097          	auipc	ra,0x1
42038b56:	89e080e7          	jalr	-1890(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42038b5a:	3c2b8637          	lui	a2,0x3c2b8
42038b5e:	82c60613          	addi	a2,a2,-2004 # 3c2b782c <c7_weights_0+0x2a2c>
42038b62:	6585                	lui	a1,0x1
42038b64:	853a                	mv	a0,a4
42038b66:	00001097          	auipc	ra,0x1
42038b6a:	88a080e7          	jalr	-1910(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42038b6e:	3c2b8637          	lui	a2,0x3c2b8
42038b72:	81c60613          	addi	a2,a2,-2020 # 3c2b781c <c7_weights_0+0x2a1c>
42038b76:	6585                	lui	a1,0x1
42038b78:	8536                	mv	a0,a3
42038b7a:	00001097          	auipc	ra,0x1
42038b7e:	876080e7          	jalr	-1930(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42038b82:	00208513          	addi	a0,ra,2
42038b86:	3c2b7637          	lui	a2,0x3c2b7
42038b8a:	7ec60613          	addi	a2,a2,2028 # 3c2b77ec <c7_weights_0+0x29ec>
42038b8e:	04000593          	li	a1,64
42038b92:	00001097          	auipc	ra,0x1
42038b96:	85e080e7          	jalr	-1954(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42038b9a:	3c2b7637          	lui	a2,0x3c2b7
42038b9e:	7dc60613          	addi	a2,a2,2012 # 3c2b77dc <c7_weights_0+0x29dc>
42038ba2:	04000593          	li	a1,64
42038ba6:	8506                	mv	a0,ra
42038ba8:	00001097          	auipc	ra,0x1
42038bac:	848080e7          	jalr	-1976(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
