
hardware/esp32c3-v4/build/riscv_int8_generalize.elf:     file format elf32-littleriscv


Disassembly of section .iram0.text:

Disassembly of section .flash.text:

4202318a <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE>:
4202318a:	81010113          	addi	sp,sp,-2032
4202318e:	7e112623          	sw	ra,2028(sp)
42023192:	7e812423          	sw	s0,2024(sp)
42023196:	7e912223          	sw	s1,2020(sp)
4202319a:	7f212023          	sw	s2,2016(sp)
4202319e:	7d312e23          	sw	s3,2012(sp)
420231a2:	7d412c23          	sw	s4,2008(sp)
420231a6:	7d512a23          	sw	s5,2004(sp)
420231aa:	7d612823          	sw	s6,2000(sp)
420231ae:	7d712623          	sw	s7,1996(sp)
420231b2:	7d812423          	sw	s8,1992(sp)
420231b6:	7d912223          	sw	s9,1988(sp)
420231ba:	7da12023          	sw	s10,1984(sp)
420231be:	7bb12e23          	sw	s11,1980(sp)
420231c2:	7141                	addi	sp,sp,-496
420231c4:	d33e                	sw	a5,164(sp)
420231c6:	893a                	mv	s2,a4
420231c8:	8db6                	mv	s11,a3
420231ca:	cf32                	sw	a2,156(sp)
420231cc:	84ae                	mv	s1,a1
420231ce:	8d2a                	mv	s10,a0
420231d0:	1168                	addi	a0,sp,172
420231d2:	7ff50413          	addi	s0,a0,2047
420231d6:	00140513          	addi	a0,s0,1
420231da:	d22a                	sw	a0,36(sp)
420231dc:	52c10c13          	addi	s8,sp,1324
420231e0:	4625                	li	a2,9
420231e2:	0622                	slli	a2,a2,0x8
420231e4:	1168                	addi	a0,sp,172
420231e6:	4581                	li	a1,0
420231e8:	fdfdd097          	auipc	ra,0xfdfdd
420231ec:	16c080e7          	jalr	364(ra) # 40000354 <memset>
420231f0:	4881                	li	a7,0
420231f2:	4801                	li	a6,0
420231f4:	08140513          	addi	a0,s0,129
420231f8:	d02a                	sw	a0,32(sp)
420231fa:	dc0d0293          	addi	t0,s10,-576
420231fe:	e00d0313          	addi	t1,s10,-512
42023202:	e40d0393          	addi	t2,s10,-448
42023206:	040d0593          	addi	a1,s10,64
4202320a:	48048513          	addi	a0,s1,1152
4202320e:	dc2a                	sw	a0,56(sp)
42023210:	e4000513          	li	a0,-448
42023214:	ceaa                	sw	a0,92(sp)
42023216:	e0000513          	li	a0,-512
4202321a:	d0aa                	sw	a0,96(sp)
4202321c:	1c000613          	li	a2,448
42023220:	20000713          	li	a4,512
42023224:	24000693          	li	a3,576
42023228:	4505                	li	a0,1
4202322a:	d14a                	sw	s2,160(sp)
4202322c:	a0a9                	j	42023276 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0xec>
4202322e:	4832                	lw	a6,12(sp)
42023230:	00883513          	sltiu	a0,a6,8
42023234:	45f6                	lw	a1,92(sp)
42023236:	20058593          	addi	a1,a1,512
4202323a:	ceae                	sw	a1,92(sp)
4202323c:	5586                	lw	a1,96(sp)
4202323e:	20058593          	addi	a1,a1,512
42023242:	d0ae                	sw	a1,96(sp)
42023244:	e0068693          	addi	a3,a3,-512
42023248:	20028293          	addi	t0,t0,512
4202324c:	e0070713          	addi	a4,a4,-512
42023250:	20030313          	addi	t1,t1,512
42023254:	e0060613          	addi	a2,a2,-512
42023258:	20038393          	addi	t2,t2,512
4202325c:	48c2                	lw	a7,16(sp)
4202325e:	e0088893          	addi	a7,a7,-512
42023262:	4d52                	lw	s10,20(sp)
42023264:	200d0d13          	addi	s10,s10,512
42023268:	9542                	add	a0,a0,a6
4202326a:	45a2                	lw	a1,8(sp)
4202326c:	20058593          	addi	a1,a1,512
42023270:	449d                	li	s1,7
42023272:	0704e1e3          	bltu	s1,a6,42023ad4 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x94a>
42023276:	c0b6                	sw	a3,64(sp)
42023278:	c2ba                	sw	a4,68(sp)
4202327a:	c4b2                	sw	a2,72(sp)
4202327c:	c69e                	sw	t2,76(sp)
4202327e:	c89a                	sw	t1,80(sp)
42023280:	ca96                	sw	t0,84(sp)
42023282:	d882                	sw	zero,112(sp)
42023284:	c62a                	sw	a0,12(sp)
42023286:	fff80513          	addi	a0,a6,-1
4202328a:	c72a                	sw	a0,140(sp)
4202328c:	00981513          	slli	a0,a6,0x9
42023290:	ccc2                	sw	a6,88(sp)
42023292:	0805                	addi	a6,a6,1
42023294:	dcc2                	sw	a6,120(sp)
42023296:	dc050493          	addi	s1,a0,-576
4202329a:	da26                	sw	s1,52(sp)
4202329c:	fc050493          	addi	s1,a0,-64
420232a0:	d826                	sw	s1,48(sp)
420232a2:	1c050493          	addi	s1,a0,448
420232a6:	daa6                	sw	s1,116(sp)
420232a8:	e8050493          	addi	s1,a0,-384
420232ac:	ce26                	sw	s1,28(sp)
420232ae:	08050493          	addi	s1,a0,128
420232b2:	d626                	sw	s1,44(sp)
420232b4:	24050493          	addi	s1,a0,576
420232b8:	d426                	sw	s1,40(sp)
420232ba:	de2a                	sw	a0,60(sp)
420232bc:	28050513          	addi	a0,a0,640
420232c0:	cc2a                	sw	a0,24(sp)
420232c2:	c42e                	sw	a1,8(sp)
420232c4:	c92e                	sw	a1,144(sp)
420232c6:	ca6a                	sw	s10,20(sp)
420232c8:	c846                	sw	a7,16(sp)
420232ca:	859e                	mv	a1,t2
420232cc:	c132                	sw	a2,128(sp)
420232ce:	841a                	mv	s0,t1
420232d0:	c33a                	sw	a4,132(sp)
420232d2:	8496                	mv	s1,t0
420232d4:	deb6                	sw	a3,124(sp)
420232d6:	a891                	j	4202332a <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x1a0>
420232d8:	5576                	lw	a0,124(sp)
420232da:	f8050513          	addi	a0,a0,-128
420232de:	deaa                	sw	a0,124(sp)
420232e0:	54a6                	lw	s1,104(sp)
420232e2:	08048493          	addi	s1,s1,128
420232e6:	451a                	lw	a0,132(sp)
420232e8:	f8050513          	addi	a0,a0,-128
420232ec:	c32a                	sw	a0,132(sp)
420232ee:	442a                	lw	s0,136(sp)
420232f0:	08040413          	addi	s0,s0,128
420232f4:	450a                	lw	a0,128(sp)
420232f6:	f8050513          	addi	a0,a0,-128
420232fa:	c12a                	sw	a0,128(sp)
420232fc:	45da                	lw	a1,148(sp)
420232fe:	08058593          	addi	a1,a1,128
42023302:	58b6                	lw	a7,108(sp)
42023304:	f8088893          	addi	a7,a7,-128
42023308:	4d6a                	lw	s10,152(sp)
4202330a:	080d0d13          	addi	s10,s10,128
4202330e:	454a                	lw	a0,144(sp)
42023310:	08050513          	addi	a0,a0,128
42023314:	c92a                	sw	a0,144(sp)
42023316:	42d6                	lw	t0,84(sp)
42023318:	4346                	lw	t1,80(sp)
4202331a:	43b6                	lw	t2,76(sp)
4202331c:	4626                	lw	a2,72(sp)
4202331e:	4519                	li	a0,6
42023320:	4716                	lw	a4,68(sp)
42023322:	4686                	lw	a3,64(sp)
42023324:	5796                	lw	a5,100(sp)
42023326:	f0a7f4e3          	bgeu	a5,a0,4202322e <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0xa4>
4202332a:	cb2e                	sw	a1,148(sp)
4202332c:	cd6a                	sw	s10,152(sp)
4202332e:	d6c6                	sw	a7,108(sp)
42023330:	5ac6                	lw	s5,112(sp)
42023332:	fffa8993          	addi	s3,s5,-1
42023336:	453a                	lw	a0,140(sp)
42023338:	00a9e533          	or	a0,s3,a0
4202333c:	006a9c93          	slli	s9,s5,0x6
42023340:	45a1                	li	a1,8
42023342:	c522                	sw	s0,136(sp)
42023344:	02b57863          	bgeu	a0,a1,42023374 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x1ea>
42023348:	5652                	lw	a2,52(sp)
4202334a:	9666                	add	a2,a2,s9
4202334c:	1168                	addi	a0,sp,172
4202334e:	04000593          	li	a1,64
42023352:	8231                	srli	a2,a2,0xc
42023354:	8726                	mv	a4,s1
42023356:	56f6                	lw	a3,124(sp)
42023358:	000619e3          	bnez	a2,42023b6a <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x9e0>
4202335c:	00070783          	lb	a5,0(a4)
42023360:	16fd                	addi	a3,a3,-1
42023362:	15fd                	addi	a1,a1,-1
42023364:	08078793          	addi	a5,a5,128
42023368:	00f51023          	sh	a5,0(a0)
4202336c:	0509                	addi	a0,a0,2
4202336e:	0705                	addi	a4,a4,1
42023370:	f5e5                	bnez	a1,42023358 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x1ce>
42023372:	a811                	j	42023386 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x1fc>
42023374:	1168                	addi	a0,sp,172
42023376:	08000613          	li	a2,128
4202337a:	4581                	li	a1,0
4202337c:	fdfdd097          	auipc	ra,0xfdfdd
42023380:	fd8080e7          	jalr	-40(ra) # 40000354 <memset>
42023384:	442a                	lw	s0,136(sp)
42023386:	d4a6                	sw	s1,104(sp)
42023388:	453a                	lw	a0,140(sp)
4202338a:	00aae4b3          	or	s1,s5,a0
4202338e:	5506                	lw	a0,96(sp)
42023390:	00c55b93          	srli	s7,a0,0xc
42023394:	451d                	li	a0,7
42023396:	08956b63          	bltu	a0,s1,4202342c <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x2a2>
4202339a:	04000593          	li	a1,64
4202339e:	8622                	mv	a2,s0
420233a0:	451a                	lw	a0,132(sp)
420233a2:	1274                	addi	a3,sp,300
420233a4:	47da                	lw	a5,148(sp)
420233a6:	780b9b63          	bnez	s7,42023b3c <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x9b2>
420233aa:	00060703          	lb	a4,0(a2)
420233ae:	15fd                	addi	a1,a1,-1
420233b0:	157d                	addi	a0,a0,-1
420233b2:	08070713          	addi	a4,a4,128
420233b6:	00e69023          	sh	a4,0(a3)
420233ba:	0689                	addi	a3,a3,2
420233bc:	0605                	addi	a2,a2,1
420233be:	f5e5                	bnez	a1,420233a6 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x21c>
420233c0:	4576                	lw	a0,92(sp)
420233c2:	00c55a13          	srli	s4,a0,0xc
420233c6:	451d                	li	a0,7
420233c8:	08956163          	bltu	a0,s1,4202344a <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x2c0>
420233cc:	04000593          	li	a1,64
420233d0:	863e                	mv	a2,a5
420233d2:	450a                	lw	a0,128(sp)
420233d4:	1374                	addi	a3,sp,428
420233d6:	760a1363          	bnez	s4,42023b3c <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x9b2>
420233da:	00060703          	lb	a4,0(a2)
420233de:	15fd                	addi	a1,a1,-1
420233e0:	157d                	addi	a0,a0,-1
420233e2:	08070713          	addi	a4,a4,128
420233e6:	00e69023          	sh	a4,0(a3)
420233ea:	0689                	addi	a3,a3,2
420233ec:	0605                	addi	a2,a2,1
420233ee:	f5e5                	bnez	a1,420233d6 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x24c>
420233f0:	4521                	li	a0,8
420233f2:	06a9f763          	bgeu	s3,a0,42023460 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x2d6>
420233f6:	55c2                	lw	a1,48(sp)
420233f8:	95e6                	add	a1,a1,s9
420233fa:	fc000513          	li	a0,-64
420233fe:	81b1                	srli	a1,a1,0xc
42023400:	1470                	addi	a2,sp,556
42023402:	1664                	addi	s1,sp,812
42023404:	476a                	lw	a4,152(sp)
42023406:	70059563          	bnez	a1,42023b10 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x986>
4202340a:	00a706b3          	add	a3,a4,a0
4202340e:	00068683          	lb	a3,0(a3)
42023412:	0505                	addi	a0,a0,1
42023414:	08068693          	addi	a3,a3,128
42023418:	00d61023          	sh	a3,0(a2)
4202341c:	0609                	addi	a2,a2,2
4202341e:	f565                	bnez	a0,42023406 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x27c>
42023420:	853a                	mv	a0,a4
42023422:	156c                	addi	a1,sp,684
42023424:	461d                	li	a2,7
42023426:	05567b63          	bgeu	a2,s5,4202347c <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x2f2>
4202342a:	aa7d                	j	420235e8 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x45e>
4202342c:	08000613          	li	a2,128
42023430:	1268                	addi	a0,sp,300
42023432:	4581                	li	a1,0
42023434:	fdfdd097          	auipc	ra,0xfdfdd
42023438:	f20080e7          	jalr	-224(ra) # 40000354 <memset>
4202343c:	47da                	lw	a5,148(sp)
4202343e:	4576                	lw	a0,92(sp)
42023440:	00c55a13          	srli	s4,a0,0xc
42023444:	451d                	li	a0,7
42023446:	f89573e3          	bgeu	a0,s1,420233cc <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x242>
4202344a:	08000613          	li	a2,128
4202344e:	1368                	addi	a0,sp,428
42023450:	4581                	li	a1,0
42023452:	fdfdd097          	auipc	ra,0xfdfdd
42023456:	f02080e7          	jalr	-254(ra) # 40000354 <memset>
4202345a:	4521                	li	a0,8
4202345c:	f8a9ede3          	bltu	s3,a0,420233f6 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x26c>
42023460:	08000613          	li	a2,128
42023464:	1468                	addi	a0,sp,556
42023466:	4581                	li	a1,0
42023468:	fdfdd097          	auipc	ra,0xfdfdd
4202346c:	eec080e7          	jalr	-276(ra) # 40000354 <memset>
42023470:	1664                	addi	s1,sp,812
42023472:	456a                	lw	a0,152(sp)
42023474:	156c                	addi	a1,sp,684
42023476:	461d                	li	a2,7
42023478:	17566863          	bltu	a2,s5,420235e8 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x45e>
4202347c:	00050603          	lb	a2,0(a0)
42023480:	08060613          	addi	a2,a2,128
42023484:	00c59023          	sh	a2,0(a1)
42023488:	0589                	addi	a1,a1,2
4202348a:	0505                	addi	a0,a0,1
4202348c:	fe9598e3          	bne	a1,s1,4202347c <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x2f2>
42023490:	46ca                	lw	a3,144(sp)
42023492:	1768                	addi	a0,sp,940
42023494:	00068603          	lb	a2,0(a3)
42023498:	08060613          	addi	a2,a2,128
4202349c:	00c49023          	sh	a2,0(s1)
420234a0:	0489                	addi	s1,s1,2
420234a2:	0685                	addi	a3,a3,1
420234a4:	fea498e3          	bne	s1,a0,42023494 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x30a>
420234a8:	55e6                	lw	a1,120(sp)
420234aa:	00b9e5b3          	or	a1,s3,a1
420234ae:	4621                	li	a2,8
420234b0:	28000993          	li	s3,640
420234b4:	16c5f363          	bgeu	a1,a2,4202361a <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x490>
420234b8:	55d6                	lw	a1,116(sp)
420234ba:	95e6                	add	a1,a1,s9
420234bc:	1c000713          	li	a4,448
420234c0:	81b1                	srli	a1,a1,0xc
420234c2:	862a                	mv	a2,a0
420234c4:	20000513          	li	a0,512
420234c8:	44ea                	lw	s1,152(sp)
420234ca:	6a059b63          	bnez	a1,42023b80 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x9f6>
420234ce:	00e486b3          	add	a3,s1,a4
420234d2:	00068683          	lb	a3,0(a3)
420234d6:	0705                	addi	a4,a4,1
420234d8:	08068693          	addi	a3,a3,128
420234dc:	00d61023          	sh	a3,0(a2)
420234e0:	0609                	addi	a2,a2,2
420234e2:	fea714e3          	bne	a4,a0,420234ca <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x340>
420234e6:	5566                	lw	a0,120(sp)
420234e8:	00aae433          	or	s0,s5,a0
420234ec:	4521                	li	a0,8
420234ee:	14a47463          	bgeu	s0,a0,42023636 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x4ac>
420234f2:	040c8513          	addi	a0,s9,64
420234f6:	55d6                	lw	a1,116(sp)
420234f8:	95aa                	add	a1,a1,a0
420234fa:	20000513          	li	a0,512
420234fe:	81b1                	srli	a1,a1,0xc
42023500:	42c10613          	addi	a2,sp,1068
42023504:	24000713          	li	a4,576
42023508:	60059463          	bnez	a1,42023b10 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x986>
4202350c:	00a486b3          	add	a3,s1,a0
42023510:	00068683          	lb	a3,0(a3)
42023514:	0505                	addi	a0,a0,1
42023516:	08068693          	addi	a3,a3,128
4202351a:	00d61023          	sh	a3,0(a2)
4202351e:	0609                	addi	a2,a2,2
42023520:	fee514e3          	bne	a0,a4,42023508 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x37e>
42023524:	002a8b13          	addi	s6,s5,2
42023528:	4521                	li	a0,8
4202352a:	12a47563          	bgeu	s0,a0,42023654 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x4ca>
4202352e:	006b1513          	slli	a0,s6,0x6
42023532:	55d6                	lw	a1,116(sp)
42023534:	95aa                	add	a1,a1,a0
42023536:	24000513          	li	a0,576
4202353a:	81b1                	srli	a1,a1,0xc
4202353c:	4ac10613          	addi	a2,sp,1196
42023540:	04000713          	li	a4,64
42023544:	47da                	lw	a5,148(sp)
42023546:	5c059563          	bnez	a1,42023b10 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x986>
4202354a:	00a486b3          	add	a3,s1,a0
4202354e:	00068683          	lb	a3,0(a3)
42023552:	0505                	addi	a0,a0,1
42023554:	08068693          	addi	a3,a3,128
42023558:	00d61023          	sh	a3,0(a2)
4202355c:	0609                	addi	a2,a2,2
4202355e:	ff3514e3          	bne	a0,s3,42023546 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x3bc>
42023562:	4521                	li	a0,8
42023564:	45ba                	lw	a1,140(sp)
42023566:	10a5f863          	bgeu	a1,a0,42023676 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x4ec>
4202356a:	4501                	li	a0,0
4202356c:	85e2                	mv	a1,s8
4202356e:	46aa                	lw	a3,136(sp)
42023570:	620b9463          	bnez	s7,42023b98 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0xa0e>
42023574:	00a68633          	add	a2,a3,a0
42023578:	00060603          	lb	a2,0(a2)
4202357c:	0505                	addi	a0,a0,1
4202357e:	08060613          	addi	a2,a2,128
42023582:	00c59023          	sh	a2,0(a1)
42023586:	0589                	addi	a1,a1,2
42023588:	fee514e3          	bne	a0,a4,42023570 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x3e6>
4202358c:	4501                	li	a0,0
4202358e:	5ac10593          	addi	a1,sp,1452
42023592:	600a1e63          	bnez	s4,42023bae <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0xa24>
42023596:	00a78633          	add	a2,a5,a0
4202359a:	00060603          	lb	a2,0(a2)
4202359e:	0505                	addi	a0,a0,1
420235a0:	08060613          	addi	a2,a2,128
420235a4:	00c59023          	sh	a2,0(a1)
420235a8:	0589                	addi	a1,a1,2
420235aa:	fee514e3          	bne	a0,a4,42023592 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x408>
420235ae:	4519                	li	a0,6
420235b0:	0caaf763          	bgeu	s5,a0,4202367e <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x4f4>
420235b4:	45f2                	lw	a1,28(sp)
420235b6:	95e6                	add	a1,a1,s9
420235b8:	e8000713          	li	a4,-384
420235bc:	81b1                	srli	a1,a1,0xc
420235be:	62c10613          	addi	a2,sp,1580
420235c2:	7ac10513          	addi	a0,sp,1964
420235c6:	ec000793          	li	a5,-320
420235ca:	58059463          	bnez	a1,42023b52 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x9c8>
420235ce:	00e486b3          	add	a3,s1,a4
420235d2:	00068683          	lb	a3,0(a3)
420235d6:	0705                	addi	a4,a4,1
420235d8:	08068693          	addi	a3,a3,128
420235dc:	00d61023          	sh	a3,0(a2)
420235e0:	0609                	addi	a2,a2,2
420235e2:	fef714e3          	bne	a4,a5,420235ca <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x440>
420235e6:	a845                	j	42023696 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x50c>
420235e8:	08000613          	li	a2,128
420235ec:	1568                	addi	a0,sp,684
420235ee:	4581                	li	a1,0
420235f0:	fdfdd097          	auipc	ra,0xfdfdd
420235f4:	d64080e7          	jalr	-668(ra) # 40000354 <memset>
420235f8:	08000613          	li	a2,128
420235fc:	8526                	mv	a0,s1
420235fe:	4581                	li	a1,0
42023600:	fdfdd097          	auipc	ra,0xfdfdd
42023604:	d54080e7          	jalr	-684(ra) # 40000354 <memset>
42023608:	1768                	addi	a0,sp,940
4202360a:	55e6                	lw	a1,120(sp)
4202360c:	00b9e5b3          	or	a1,s3,a1
42023610:	4621                	li	a2,8
42023612:	28000993          	li	s3,640
42023616:	eac5e1e3          	bltu	a1,a2,420234b8 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x32e>
4202361a:	08000613          	li	a2,128
4202361e:	4581                	li	a1,0
42023620:	fdfdd097          	auipc	ra,0xfdfdd
42023624:	d34080e7          	jalr	-716(ra) # 40000354 <memset>
42023628:	44ea                	lw	s1,152(sp)
4202362a:	5566                	lw	a0,120(sp)
4202362c:	00aae433          	or	s0,s5,a0
42023630:	4521                	li	a0,8
42023632:	eca460e3          	bltu	s0,a0,420234f2 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x368>
42023636:	08000613          	li	a2,128
4202363a:	42c10513          	addi	a0,sp,1068
4202363e:	4581                	li	a1,0
42023640:	fdfdd097          	auipc	ra,0xfdfdd
42023644:	d14080e7          	jalr	-748(ra) # 40000354 <memset>
42023648:	44ea                	lw	s1,152(sp)
4202364a:	002a8b13          	addi	s6,s5,2
4202364e:	4521                	li	a0,8
42023650:	eca46fe3          	bltu	s0,a0,4202352e <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x3a4>
42023654:	08000613          	li	a2,128
42023658:	4ac10513          	addi	a0,sp,1196
4202365c:	4581                	li	a1,0
4202365e:	fdfdd097          	auipc	ra,0xfdfdd
42023662:	cf6080e7          	jalr	-778(ra) # 40000354 <memset>
42023666:	44ea                	lw	s1,152(sp)
42023668:	04000713          	li	a4,64
4202366c:	47da                	lw	a5,148(sp)
4202366e:	4521                	li	a0,8
42023670:	45ba                	lw	a1,140(sp)
42023672:	eea5ece3          	bltu	a1,a0,4202356a <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x3e0>
42023676:	18000613          	li	a2,384
4202367a:	8562                	mv	a0,s8
4202367c:	a029                	j	42023686 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x4fc>
4202367e:	08000613          	li	a2,128
42023682:	62c10513          	addi	a0,sp,1580
42023686:	4581                	li	a1,0
42023688:	fdfdd097          	auipc	ra,0xfdfdd
4202368c:	ccc080e7          	jalr	-820(ra) # 40000354 <memset>
42023690:	44ea                	lw	s1,152(sp)
42023692:	7ac10513          	addi	a0,sp,1964
42023696:	001a8413          	addi	s0,s5,1
4202369a:	86a6                	mv	a3,s1
4202369c:	6ac10593          	addi	a1,sp,1708
420236a0:	72c10713          	addi	a4,sp,1836
420236a4:	00068603          	lb	a2,0(a3)
420236a8:	08060613          	addi	a2,a2,128
420236ac:	00c59023          	sh	a2,0(a1)
420236b0:	0589                	addi	a1,a1,2
420236b2:	0685                	addi	a3,a3,1
420236b4:	fee598e3          	bne	a1,a4,420236a4 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x51a>
420236b8:	46ca                	lw	a3,144(sp)
420236ba:	85ba                	mv	a1,a4
420236bc:	00068603          	lb	a2,0(a3)
420236c0:	08060613          	addi	a2,a2,128
420236c4:	00c59023          	sh	a2,0(a1)
420236c8:	0589                	addi	a1,a1,2
420236ca:	0685                	addi	a3,a3,1
420236cc:	fea598e3          	bne	a1,a0,420236bc <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x532>
420236d0:	4599                	li	a1,6
420236d2:	04baf363          	bgeu	s5,a1,42023718 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x58e>
420236d6:	55b2                	lw	a1,44(sp)
420236d8:	95e6                	add	a1,a1,s9
420236da:	08000713          	li	a4,128
420236de:	81b1                	srli	a1,a1,0xc
420236e0:	862a                	mv	a2,a0
420236e2:	0c000513          	li	a0,192
420236e6:	46059663          	bnez	a1,42023b52 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x9c8>
420236ea:	00e486b3          	add	a3,s1,a4
420236ee:	00068683          	lb	a3,0(a3)
420236f2:	0705                	addi	a4,a4,1
420236f4:	08068693          	addi	a3,a3,128
420236f8:	00d61023          	sh	a3,0(a2)
420236fc:	0609                	addi	a2,a2,2
420236fe:	fea714e3          	bne	a4,a0,420236e6 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x55c>
42023702:	451d                	li	a0,7
42023704:	45e6                	lw	a1,88(sp)
42023706:	02a59563          	bne	a1,a0,42023730 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x5a6>
4202370a:	18000613          	li	a2,384
4202370e:	7ff10513          	addi	a0,sp,2047
42023712:	02d50513          	addi	a0,a0,45
42023716:	a845                	j	420237c6 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x63c>
42023718:	08000613          	li	a2,128
4202371c:	4581                	li	a1,0
4202371e:	fdfdd097          	auipc	ra,0xfdfdd
42023722:	c36080e7          	jalr	-970(ra) # 40000354 <memset>
42023726:	44ea                	lw	s1,152(sp)
42023728:	451d                	li	a0,7
4202372a:	45e6                	lw	a1,88(sp)
4202372c:	fca58fe3          	beq	a1,a0,4202370a <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x580>
42023730:	041a                	slli	s0,s0,0x6
42023732:	55d6                	lw	a1,116(sp)
42023734:	95a2                	add	a1,a1,s0
42023736:	20000513          	li	a0,512
4202373a:	81b1                	srli	a1,a1,0xc
4202373c:	7ff10613          	addi	a2,sp,2047
42023740:	02d60613          	addi	a2,a2,45
42023744:	24000713          	li	a4,576
42023748:	3c059f63          	bnez	a1,42023b26 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x99c>
4202374c:	00a486b3          	add	a3,s1,a0
42023750:	00068683          	lb	a3,0(a3)
42023754:	0505                	addi	a0,a0,1
42023756:	08068693          	addi	a3,a3,128
4202375a:	00d61023          	sh	a3,0(a2)
4202375e:	0609                	addi	a2,a2,2
42023760:	fee514e3          	bne	a0,a4,42023748 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x5be>
42023764:	55a2                	lw	a1,40(sp)
42023766:	95e6                	add	a1,a1,s9
42023768:	24000513          	li	a0,576
4202376c:	81b1                	srli	a1,a1,0xc
4202376e:	5612                	lw	a2,36(sp)
42023770:	3a059b63          	bnez	a1,42023b26 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x99c>
42023774:	00a486b3          	add	a3,s1,a0
42023778:	00068683          	lb	a3,0(a3)
4202377c:	0505                	addi	a0,a0,1
4202377e:	08068693          	addi	a3,a3,128
42023782:	00d61023          	sh	a3,0(a2)
42023786:	0609                	addi	a2,a2,2
42023788:	ff3514e3          	bne	a0,s3,42023770 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x5e6>
4202378c:	4519                	li	a0,6
4202378e:	02aaf963          	bgeu	s5,a0,420237c0 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x636>
42023792:	45e2                	lw	a1,24(sp)
42023794:	95e6                	add	a1,a1,s9
42023796:	28000513          	li	a0,640
4202379a:	81b1                	srli	a1,a1,0xc
4202379c:	5602                	lw	a2,32(sp)
4202379e:	2c000713          	li	a4,704
420237a2:	38059263          	bnez	a1,42023b26 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x99c>
420237a6:	00a486b3          	add	a3,s1,a0
420237aa:	00068683          	lb	a3,0(a3)
420237ae:	0505                	addi	a0,a0,1
420237b0:	08068693          	addi	a3,a3,128
420237b4:	00d61023          	sh	a3,0(a2)
420237b8:	0609                	addi	a2,a2,2
420237ba:	fee514e3          	bne	a0,a4,420237a2 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x618>
420237be:	a809                	j	420237d0 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x646>
420237c0:	08000613          	li	a2,128
420237c4:	5502                	lw	a0,32(sp)
420237c6:	4581                	li	a1,0
420237c8:	fdfdd097          	auipc	ra,0xfdfdd
420237cc:	b8c080e7          	jalr	-1140(ra) # 40000354 <memset>
420237d0:	d8da                	sw	s6,112(sp)
420237d2:	d2d6                	sw	s5,100(sp)
420237d4:	4501                	li	a0,0
420237d6:	55f2                	lw	a1,60(sp)
420237d8:	95e6                	add	a1,a1,s9
420237da:	d52e                	sw	a1,168(sp)
420237dc:	52e2                	lw	t0,56(sp)
420237de:	a829                	j	420237f8 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x66e>
420237e0:	04a081a3          	sb	a0,67(ra)
420237e4:	004f0513          	addi	a0,t5,4
420237e8:	7ff28593          	addi	a1,t0,2047
420237ec:	10158293          	addi	t0,a1,257
420237f0:	03c00593          	li	a1,60
420237f4:	aebf72e3          	bgeu	t5,a1,420232d8 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x14e>
420237f8:	40000d37          	lui	s10,0x40000
420237fc:	8f2a                	mv	t5,a0
420237fe:	00251393          	slli	t2,a0,0x2
42023802:	45fa                	lw	a1,156(sp)
42023804:	959e                	add	a1,a1,t2
42023806:	0005ae03          	lw	t3,0(a1)
4202380a:	0045a803          	lw	a6,4(a1)
4202380e:	0085a983          	lw	s3,8(a1)
42023812:	00c5aa03          	lw	s4,12(a1)
42023816:	00150e93          	addi	t4,a0,1
4202381a:	00250313          	addi	t1,a0,2
4202381e:	00350893          	addi	a7,a0,3
42023822:	1174                	addi	a3,sp,172
42023824:	8716                	mv	a4,t0
42023826:	8b72                	mv	s6,t3
42023828:	8942                	mv	s2,a6
4202382a:	8ace                	mv	s5,s3
4202382c:	8fd2                	mv	t6,s4
4202382e:	00069583          	lh	a1,0(a3)
42023832:	48069503          	lh	a0,1152(a3)
42023836:	b8070783          	lb	a5,-1152(a4)
4202383a:	dc070403          	lb	s0,-576(a4)
4202383e:	00070603          	lb	a2,0(a4)
42023842:	24070483          	lb	s1,576(a4)
42023846:	02b78bb3          	mul	s7,a5,a1
4202384a:	02a787b3          	mul	a5,a5,a0
4202384e:	02b400b3          	mul	ra,s0,a1
42023852:	02a40433          	mul	s0,s0,a0
42023856:	02a60cb3          	mul	s9,a2,a0
4202385a:	02b60633          	mul	a2,a2,a1
4202385e:	02b485b3          	mul	a1,s1,a1
42023862:	02a48533          	mul	a0,s1,a0
42023866:	9e5e                	add	t3,t3,s7
42023868:	9b3e                	add	s6,s6,a5
4202386a:	9806                	add	a6,a6,ra
4202386c:	9922                	add	s2,s2,s0
4202386e:	0689                	addi	a3,a3,2
42023870:	9ae6                	add	s5,s5,s9
42023872:	99b2                	add	s3,s3,a2
42023874:	9a2e                	add	s4,s4,a1
42023876:	9faa                	add	t6,t6,a0
42023878:	0705                	addi	a4,a4,1
4202387a:	fb869ae3          	bne	a3,s8,4202382e <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x6a4>
4202387e:	007d86b3          	add	a3,s11,t2
42023882:	0006a083          	lw	ra,0(a3)
42023886:	5c8a                	lw	s9,160(sp)
42023888:	93e6                	add	t2,t2,s9
4202388a:	0003a703          	lw	a4,0(t2)
4202388e:	03c093b3          	mulh	t2,ra,t3
42023892:	03c084b3          	mul	s1,ra,t3
42023896:	01a486b3          	add	a3,s1,s10
4202389a:	0096b4b3          	sltu	s1,a3,s1
4202389e:	93a6                	add	t2,t2,s1
420238a0:	fff74493          	not	s1,a4
420238a4:	40e00bb3          	neg	s7,a4
420238a8:	4505                	li	a0,1
420238aa:	00951733          	sll	a4,a0,s1
420238ae:	82fd                	srli	a3,a3,0x1f
420238b0:	0386                	slli	t2,t2,0x1
420238b2:	0076e6b3          	or	a3,a3,t2
420238b6:	41f3d493          	srai	s1,t2,0x1f
420238ba:	96ba                	add	a3,a3,a4
420238bc:	96a6                	add	a3,a3,s1
420238be:	4176d4b3          	sra	s1,a3,s7
420238c2:	14f9                	addi	s1,s1,-2
420238c4:	07f00393          	li	t2,127
420238c8:	0074c463          	blt	s1,t2,420238d0 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x746>
420238cc:	07f00493          	li	s1,127
420238d0:	f8000e13          	li	t3,-128
420238d4:	009e4463          	blt	t3,s1,420238dc <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x752>
420238d8:	f8000493          	li	s1,-128
420238dc:	036096b3          	mulh	a3,ra,s6
420238e0:	03608633          	mul	a2,ra,s6
420238e4:	01a60b33          	add	s6,a2,s10
420238e8:	00cb3633          	sltu	a2,s6,a2
420238ec:	9636                	add	a2,a2,a3
420238ee:	50aa                	lw	ra,168(sp)
420238f0:	90fa                	add	ra,ra,t5
420238f2:	551a                	lw	a0,164(sp)
420238f4:	90aa                	add	ra,ra,a0
420238f6:	01fb5693          	srli	a3,s6,0x1f
420238fa:	0606                	slli	a2,a2,0x1
420238fc:	8ed1                	or	a3,a3,a2
420238fe:	867d                	srai	a2,a2,0x1f
42023900:	96ba                	add	a3,a3,a4
42023902:	9636                	add	a2,a2,a3
42023904:	41765733          	sra	a4,a2,s7
42023908:	1779                	addi	a4,a4,-2
4202390a:	00908023          	sb	s1,0(ra)
4202390e:	00774463          	blt	a4,t2,42023916 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x78c>
42023912:	07f00713          	li	a4,127
42023916:	00ee4463          	blt	t3,a4,4202391e <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x794>
4202391a:	f8000713          	li	a4,-128
4202391e:	0e8a                	slli	t4,t4,0x2
42023920:	01dd8633          	add	a2,s11,t4
42023924:	4214                	lw	a3,0(a2)
42023926:	9ee6                	add	t4,t4,s9
42023928:	000ea603          	lw	a2,0(t4)
4202392c:	03069eb3          	mulh	t4,a3,a6
42023930:	03068433          	mul	s0,a3,a6
42023934:	40000837          	lui	a6,0x40000
42023938:	010404b3          	add	s1,s0,a6
4202393c:	0084b433          	sltu	s0,s1,s0
42023940:	008e8bb3          	add	s7,t4,s0
42023944:	fff64413          	not	s0,a2
42023948:	40c00eb3          	neg	t4,a2
4202394c:	4505                	li	a0,1
4202394e:	00851b33          	sll	s6,a0,s0
42023952:	80fd                	srli	s1,s1,0x1f
42023954:	0b86                	slli	s7,s7,0x1
42023956:	0174e633          	or	a2,s1,s7
4202395a:	41fbd413          	srai	s0,s7,0x1f
4202395e:	965a                	add	a2,a2,s6
42023960:	9622                	add	a2,a2,s0
42023962:	41d65633          	sra	a2,a2,t4
42023966:	1679                	addi	a2,a2,-2
42023968:	04e08023          	sb	a4,64(ra)
4202396c:	00764463          	blt	a2,t2,42023974 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x7ea>
42023970:	07f00613          	li	a2,127
42023974:	00ce4463          	blt	t3,a2,4202397c <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x7f2>
42023978:	f8000613          	li	a2,-128
4202397c:	03269733          	mulh	a4,a3,s2
42023980:	032686b3          	mul	a3,a3,s2
42023984:	010687b3          	add	a5,a3,a6
42023988:	00d7b6b3          	sltu	a3,a5,a3
4202398c:	96ba                	add	a3,a3,a4
4202398e:	83fd                	srli	a5,a5,0x1f
42023990:	0686                	slli	a3,a3,0x1
42023992:	8fd5                	or	a5,a5,a3
42023994:	86fd                	srai	a3,a3,0x1f
42023996:	97da                	add	a5,a5,s6
42023998:	96be                	add	a3,a3,a5
4202399a:	41d6d733          	sra	a4,a3,t4
4202399e:	1779                	addi	a4,a4,-2
420239a0:	00c080a3          	sb	a2,1(ra)
420239a4:	00774463          	blt	a4,t2,420239ac <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x822>
420239a8:	07f00713          	li	a4,127
420239ac:	00ee4463          	blt	t3,a4,420239b4 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x82a>
420239b0:	f8000713          	li	a4,-128
420239b4:	030a                	slli	t1,t1,0x2
420239b6:	006d8633          	add	a2,s11,t1
420239ba:	4214                	lw	a3,0(a2)
420239bc:	9366                	add	t1,t1,s9
420239be:	00032603          	lw	a2,0(t1)
420239c2:	033697b3          	mulh	a5,a3,s3
420239c6:	03368533          	mul	a0,a3,s3
420239ca:	010504b3          	add	s1,a0,a6
420239ce:	00a4b533          	sltu	a0,s1,a0
420239d2:	953e                	add	a0,a0,a5
420239d4:	fff64793          	not	a5,a2
420239d8:	40c00433          	neg	s0,a2
420239dc:	4585                	li	a1,1
420239de:	00f597b3          	sll	a5,a1,a5
420239e2:	80fd                	srli	s1,s1,0x1f
420239e4:	0506                	slli	a0,a0,0x1
420239e6:	8cc9                	or	s1,s1,a0
420239e8:	857d                	srai	a0,a0,0x1f
420239ea:	94be                	add	s1,s1,a5
420239ec:	9526                	add	a0,a0,s1
420239ee:	40855633          	sra	a2,a0,s0
420239f2:	1679                	addi	a2,a2,-2
420239f4:	04e080a3          	sb	a4,65(ra)
420239f8:	00764463          	blt	a2,t2,42023a00 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x876>
420239fc:	07f00613          	li	a2,127
42023a00:	00ce4463          	blt	t3,a2,42023a08 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x87e>
42023a04:	f8000613          	li	a2,-128
42023a08:	03569533          	mulh	a0,a3,s5
42023a0c:	035685b3          	mul	a1,a3,s5
42023a10:	010586b3          	add	a3,a1,a6
42023a14:	00b6b5b3          	sltu	a1,a3,a1
42023a18:	952e                	add	a0,a0,a1
42023a1a:	82fd                	srli	a3,a3,0x1f
42023a1c:	0506                	slli	a0,a0,0x1
42023a1e:	8ec9                	or	a3,a3,a0
42023a20:	857d                	srai	a0,a0,0x1f
42023a22:	96be                	add	a3,a3,a5
42023a24:	9536                	add	a0,a0,a3
42023a26:	408555b3          	sra	a1,a0,s0
42023a2a:	15f9                	addi	a1,a1,-2
42023a2c:	00c08123          	sb	a2,2(ra)
42023a30:	0075c463          	blt	a1,t2,42023a38 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x8ae>
42023a34:	07f00593          	li	a1,127
42023a38:	00be4463          	blt	t3,a1,42023a40 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x8b6>
42023a3c:	f8000593          	li	a1,-128
42023a40:	088a                	slli	a7,a7,0x2
42023a42:	011d8533          	add	a0,s11,a7
42023a46:	4108                	lw	a0,0(a0)
42023a48:	98e6                	add	a7,a7,s9
42023a4a:	0008a603          	lw	a2,0(a7)
42023a4e:	034516b3          	mulh	a3,a0,s4
42023a52:	03450733          	mul	a4,a0,s4
42023a56:	010707b3          	add	a5,a4,a6
42023a5a:	00e7b733          	sltu	a4,a5,a4
42023a5e:	96ba                	add	a3,a3,a4
42023a60:	fff64713          	not	a4,a2
42023a64:	40c004b3          	neg	s1,a2
42023a68:	4605                	li	a2,1
42023a6a:	00e61733          	sll	a4,a2,a4
42023a6e:	83fd                	srli	a5,a5,0x1f
42023a70:	0686                	slli	a3,a3,0x1
42023a72:	8fd5                	or	a5,a5,a3
42023a74:	86fd                	srai	a3,a3,0x1f
42023a76:	97ba                	add	a5,a5,a4
42023a78:	96be                	add	a3,a3,a5
42023a7a:	4096d633          	sra	a2,a3,s1
42023a7e:	1679                	addi	a2,a2,-2
42023a80:	04b08123          	sb	a1,66(ra)
42023a84:	00764463          	blt	a2,t2,42023a8c <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x902>
42023a88:	07f00613          	li	a2,127
42023a8c:	00ce4463          	blt	t3,a2,42023a94 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x90a>
42023a90:	f8000613          	li	a2,-128
42023a94:	03f515b3          	mulh	a1,a0,t6
42023a98:	03f50533          	mul	a0,a0,t6
42023a9c:	010506b3          	add	a3,a0,a6
42023aa0:	00a6b533          	sltu	a0,a3,a0
42023aa4:	952e                	add	a0,a0,a1
42023aa6:	82fd                	srli	a3,a3,0x1f
42023aa8:	0506                	slli	a0,a0,0x1
42023aaa:	8ec9                	or	a3,a3,a0
42023aac:	857d                	srai	a0,a0,0x1f
42023aae:	96ba                	add	a3,a3,a4
42023ab0:	9536                	add	a0,a0,a3
42023ab2:	40955533          	sra	a0,a0,s1
42023ab6:	1579                	addi	a0,a0,-2
42023ab8:	00c081a3          	sb	a2,3(ra)
42023abc:	00755563          	bge	a0,t2,42023ac6 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x93c>
42023ac0:	d2ae40e3          	blt	t3,a0,420237e0 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x656>
42023ac4:	a029                	j	42023ace <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x944>
42023ac6:	07f00513          	li	a0,127
42023aca:	d0ae4be3          	blt	t3,a0,420237e0 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x656>
42023ace:	f8000513          	li	a0,-128
42023ad2:	b339                	j	420237e0 <_ZN7kernels6r_rust7layer_93run17haa6283f55b0d0ceaE+0x656>
42023ad4:	617d                	addi	sp,sp,496
42023ad6:	7ec12083          	lw	ra,2028(sp)
42023ada:	7e812403          	lw	s0,2024(sp)
42023ade:	7e412483          	lw	s1,2020(sp)
42023ae2:	7e012903          	lw	s2,2016(sp)
42023ae6:	7dc12983          	lw	s3,2012(sp)
42023aea:	7d812a03          	lw	s4,2008(sp)
42023aee:	7d412a83          	lw	s5,2004(sp)
42023af2:	7d012b03          	lw	s6,2000(sp)
42023af6:	7cc12b83          	lw	s7,1996(sp)
42023afa:	7c812c03          	lw	s8,1992(sp)
42023afe:	7c412c83          	lw	s9,1988(sp)
42023b02:	7c012d03          	lw	s10,1984(sp)
42023b06:	7bc12d83          	lw	s11,1980(sp)
42023b0a:	7f010113          	addi	sp,sp,2032
42023b0e:	8082                	ret
42023b10:	55b6                	lw	a1,108(sp)
42023b12:	8d0d                	sub	a0,a0,a1
42023b14:	3c2b6637          	lui	a2,0x3c2b6
42023b18:	12c60613          	addi	a2,a2,300 # 3c2b612c <c7_weights_0+0x132c>
42023b1c:	6585                	lui	a1,0x1
42023b1e:	00016097          	auipc	ra,0x16
42023b22:	8d2080e7          	jalr	-1838(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42023b26:	55b6                	lw	a1,108(sp)
42023b28:	8d0d                	sub	a0,a0,a1
42023b2a:	3c2b6637          	lui	a2,0x3c2b6
42023b2e:	11c60613          	addi	a2,a2,284 # 3c2b611c <c7_weights_0+0x131c>
42023b32:	6585                	lui	a1,0x1
42023b34:	00016097          	auipc	ra,0x16
42023b38:	8bc080e7          	jalr	-1860(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42023b3c:	40a00533          	neg	a0,a0
42023b40:	3c2b6637          	lui	a2,0x3c2b6
42023b44:	12c60613          	addi	a2,a2,300 # 3c2b612c <c7_weights_0+0x132c>
42023b48:	6585                	lui	a1,0x1
42023b4a:	00016097          	auipc	ra,0x16
42023b4e:	8a6080e7          	jalr	-1882(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42023b52:	5536                	lw	a0,108(sp)
42023b54:	40a70533          	sub	a0,a4,a0
42023b58:	3c2b6637          	lui	a2,0x3c2b6
42023b5c:	11c60613          	addi	a2,a2,284 # 3c2b611c <c7_weights_0+0x131c>
42023b60:	6585                	lui	a1,0x1
42023b62:	00016097          	auipc	ra,0x16
42023b66:	88e080e7          	jalr	-1906(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42023b6a:	40d00533          	neg	a0,a3
42023b6e:	3c2b6637          	lui	a2,0x3c2b6
42023b72:	12c60613          	addi	a2,a2,300 # 3c2b612c <c7_weights_0+0x132c>
42023b76:	6585                	lui	a1,0x1
42023b78:	00016097          	auipc	ra,0x16
42023b7c:	878080e7          	jalr	-1928(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42023b80:	5536                	lw	a0,108(sp)
42023b82:	40a70533          	sub	a0,a4,a0
42023b86:	3c2b6637          	lui	a2,0x3c2b6
42023b8a:	12c60613          	addi	a2,a2,300 # 3c2b612c <c7_weights_0+0x132c>
42023b8e:	6585                	lui	a1,0x1
42023b90:	00016097          	auipc	ra,0x16
42023b94:	860080e7          	jalr	-1952(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42023b98:	459a                	lw	a1,132(sp)
42023b9a:	8d0d                	sub	a0,a0,a1
42023b9c:	3c2b6637          	lui	a2,0x3c2b6
42023ba0:	11c60613          	addi	a2,a2,284 # 3c2b611c <c7_weights_0+0x131c>
42023ba4:	6585                	lui	a1,0x1
42023ba6:	00016097          	auipc	ra,0x16
42023baa:	84a080e7          	jalr	-1974(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
42023bae:	458a                	lw	a1,128(sp)
42023bb0:	8d0d                	sub	a0,a0,a1
42023bb2:	3c2b6637          	lui	a2,0x3c2b6
42023bb6:	11c60613          	addi	a2,a2,284 # 3c2b611c <c7_weights_0+0x131c>
42023bba:	6585                	lui	a1,0x1
42023bbc:	00016097          	auipc	ra,0x16
42023bc0:	834080e7          	jalr	-1996(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
