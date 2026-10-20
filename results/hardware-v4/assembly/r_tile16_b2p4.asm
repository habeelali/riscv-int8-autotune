
hardware/esp32c3-v4/build/riscv_int8_generalize.elf:     file format elf32-littleriscv


Disassembly of section .iram0.text:

Disassembly of section .flash.text:

4202e17a <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E>:
4202e17a:	7111                	addi	sp,sp,-256
4202e17c:	df86                	sw	ra,252(sp)
4202e17e:	dda2                	sw	s0,248(sp)
4202e180:	dba6                	sw	s1,244(sp)
4202e182:	d9ca                	sw	s2,240(sp)
4202e184:	d7ce                	sw	s3,236(sp)
4202e186:	d5d2                	sw	s4,232(sp)
4202e188:	d3d6                	sw	s5,228(sp)
4202e18a:	d1da                	sw	s6,224(sp)
4202e18c:	cfde                	sw	s7,220(sp)
4202e18e:	cde2                	sw	s8,216(sp)
4202e190:	cbe6                	sw	s9,212(sp)
4202e192:	c9ea                	sw	s10,208(sp)
4202e194:	c7ee                	sw	s11,204(sp)
4202e196:	6809                	lui	a6,0x2
4202e198:	3a080813          	addi	a6,a6,928 # 23a0 <RvExcFrameSize+0x230c>
4202e19c:	41010133          	sub	sp,sp,a6
4202e1a0:	8b3e                	mv	s6,a5
4202e1a2:	d03a                	sw	a4,32(sp)
4202e1a4:	ce36                	sw	a3,28(sp)
4202e1a6:	cc32                	sw	a2,24(sp)
4202e1a8:	842e                	mv	s0,a1
4202e1aa:	84aa                	mv	s1,a0
4202e1ac:	40000cb7          	lui	s9,0x40000
4202e1b0:	4525                	li	a0,9
4202e1b2:	00a51613          	slli	a2,a0,0xa
4202e1b6:	10e8                	addi	a0,sp,108
4202e1b8:	4581                	li	a1,0
4202e1ba:	c0b2                	sw	a2,64(sp)
4202e1bc:	fdfd2097          	auipc	ra,0xfdfd2
4202e1c0:	198080e7          	jalr	408(ra) # 40000354 <memset>
4202e1c4:	4701                	li	a4,0
4202e1c6:	ca22                	sw	s0,20(sp)
4202e1c8:	0405                	addi	s0,s0,1
4202e1ca:	c822                	sw	s0,16(sp)
4202e1cc:	450d                	li	a0,3
4202e1ce:	04000613          	li	a2,64
4202e1d2:	6689                	lui	a3,0x2
4202e1d4:	45ed                	li	a1,27
4202e1d6:	f8068693          	addi	a3,a3,-128 # 1f80 <RvExcFrameSize+0x1eec>
4202e1da:	de36                	sw	a3,60(sp)
4202e1dc:	05a2                	slli	a1,a1,0x8
4202e1de:	dc2e                	sw	a1,56(sp)
4202e1e0:	d426                	sw	s1,40(sp)
4202e1e2:	40e606b3          	sub	a3,a2,a4
4202e1e6:	45c1                	li	a1,16
4202e1e8:	c62a                	sw	a0,12(sp)
4202e1ea:	c8b6                	sw	a3,80(sp)
4202e1ec:	00b6e463          	bltu	a3,a1,4202e1f4 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x7a>
4202e1f0:	4541                	li	a0,16
4202e1f2:	c8aa                	sw	a0,80(sp)
4202e1f4:	d4ba                	sw	a4,104(sp)
4202e1f6:	42c71b63          	bne	a4,a2,4202e62c <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x4b2>
4202e1fa:	da02                	sw	zero,52(sp)
4202e1fc:	4501                	li	a0,0
4202e1fe:	4646                	lw	a2,80(sp)
4202e200:	8209                	srli	a2,a2,0x2
4202e202:	44fd                	li	s1,31
4202e204:	fff60693          	addi	a3,a2,-1
4202e208:	00d63633          	sltu	a2,a2,a3
4202e20c:	167d                	addi	a2,a2,-1
4202e20e:	8e75                	and	a2,a2,a3
4202e210:	d232                	sw	a2,36(sp)
4202e212:	4789                	li	a5,2
4202e214:	49c2                	lw	s3,16(sp)
4202e216:	a005                	j	4202e236 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0xbc>
4202e218:	5642                	lw	a2,48(sp)
4202e21a:	8532                	mv	a0,a2
4202e21c:	00260793          	addi	a5,a2,2
4202e220:	55b2                	lw	a1,44(sp)
4202e222:	fff58493          	addi	s1,a1,-1
4202e226:	48098993          	addi	s3,s3,1152
4202e22a:	56d2                	lw	a3,52(sp)
4202e22c:	0685                	addi	a3,a3,1
4202e22e:	da36                	sw	a3,52(sp)
4202e230:	5726                	lw	a4,104(sp)
4202e232:	74058a63          	beqz	a1,4202e986 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x80c>
4202e236:	24000613          	li	a2,576
4202e23a:	02c506b3          	mul	a3,a0,a2
4202e23e:	b8000593          	li	a1,-1152
4202e242:	02b6f4e3          	bgeu	a3,a1,4202ea6a <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8f0>
4202e246:	d626                	sw	s1,44(sp)
4202e248:	d83e                	sw	a5,48(sp)
4202e24a:	04000593          	li	a1,64
4202e24e:	fcb705e3          	beq	a4,a1,4202e218 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x9e>
4202e252:	03f00593          	li	a1,63
4202e256:	06a5ede3          	bltu	a1,a0,4202ead0 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x956>
4202e25a:	4b81                	li	s7,0
4202e25c:	48000593          	li	a1,1152
4202e260:	5652                	lw	a2,52(sp)
4202e262:	02b605b3          	mul	a1,a2,a1
4202e266:	04050613          	addi	a2,a0,64
4202e26a:	ceb2                	sw	a2,92(sp)
4202e26c:	08050613          	addi	a2,a0,128
4202e270:	c8b2                	sw	a2,80(sp)
4202e272:	0c050613          	addi	a2,a0,192
4202e276:	c6b2                	sw	a2,76(sp)
4202e278:	00251c13          	slli	s8,a0,0x2
4202e27c:	4652                	lw	a2,20(sp)
4202e27e:	95b2                	add	a1,a1,a2
4202e280:	4762                	lw	a4,24(sp)
4202e282:	01870633          	add	a2,a4,s8
4202e286:	d2b2                	sw	a2,100(sp)
4202e288:	004c0f93          	addi	t6,s8,4
4202e28c:	46f2                	lw	a3,28(sp)
4202e28e:	01868633          	add	a2,a3,s8
4202e292:	ccb2                	sw	a2,88(sp)
4202e294:	5602                	lw	a2,32(sp)
4202e296:	9c32                	add	s8,s8,a2
4202e298:	cae2                	sw	s8,84(sp)
4202e29a:	48158f13          	addi	t5,a1,1153
4202e29e:	977e                	add	a4,a4,t6
4202e2a0:	d0ba                	sw	a4,96(sp)
4202e2a2:	96fe                	add	a3,a3,t6
4202e2a4:	c4b6                	sw	a3,72(sp)
4202e2a6:	967e                	add	a2,a2,t6
4202e2a8:	c2b2                	sw	a2,68(sp)
4202e2aa:	4911                	li	s2,4
4202e2ac:	4ec10a13          	addi	s4,sp,1260
4202e2b0:	5592                	lw	a1,36(sp)
4202e2b2:	a829                	j	4202e2cc <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x152>
4202e2b4:	00b380a3          	sb	a1,1(t2)
4202e2b8:	8bca                	mv	s7,s2
4202e2ba:	0911                	addi	s2,s2,4
4202e2bc:	fffd8593          	addi	a1,s11,-1
4202e2c0:	7ffa0613          	addi	a2,s4,2047
4202e2c4:	10160a13          	addi	s4,a2,257
4202e2c8:	f40d88e3          	beqz	s11,4202e218 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x9e>
4202e2cc:	8dae                	mv	s11,a1
4202e2ce:	24000593          	li	a1,576
4202e2d2:	02bb86b3          	mul	a3,s7,a1
4202e2d6:	24068713          	addi	a4,a3,576
4202e2da:	dbf00593          	li	a1,-577
4202e2de:	6cd5ef63          	bltu	a1,a3,4202e9bc <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x842>
4202e2e2:	4586                	lw	a1,64(sp)
4202e2e4:	6ee5e663          	bltu	a1,a4,4202e9d0 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x856>
4202e2e8:	48068593          	addi	a1,a3,1152
4202e2ec:	6ee5ed63          	bltu	a1,a4,4202e9e6 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x86c>
4202e2f0:	5672                	lw	a2,60(sp)
4202e2f2:	70d66363          	bltu	a2,a3,4202e9f8 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x87e>
4202e2f6:	6589                	lui	a1,0x2
4202e2f8:	d4058593          	addi	a1,a1,-704 # 1d40 <RvExcFrameSize+0x1cac>
4202e2fc:	70d5ea63          	bltu	a1,a3,4202ea10 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x896>
4202e300:	55e2                	lw	a1,56(sp)
4202e302:	72d5e363          	bltu	a1,a3,4202ea28 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8ae>
4202e306:	5596                	lw	a1,100(sp)
4202e308:	0005ad03          	lw	s10,0(a1)
4202e30c:	5586                	lw	a1,96(sp)
4202e30e:	0005a883          	lw	a7,0(a1)
4202e312:	86d2                	mv	a3,s4
4202e314:	864e                	mv	a2,s3
4202e316:	8eea                	mv	t4,s10
4202e318:	8fea                	mv	t6,s10
4202e31a:	8c6a                	mv	s8,s10
4202e31c:	82c6                	mv	t0,a7
4202e31e:	8846                	mv	a6,a7
4202e320:	80c6                	mv	ra,a7
4202e322:	b8068583          	lb	a1,-1152(a3)
4202e326:	24068303          	lb	t1,576(a3)
4202e32a:	fff60783          	lb	a5,-1(a2)
4202e32e:	00068483          	lb	s1,0(a3)
4202e332:	dc068403          	lb	s0,-576(a3)
4202e336:	00060703          	lb	a4,0(a2)
4202e33a:	026783b3          	mul	t2,a5,t1
4202e33e:	02978e33          	mul	t3,a5,s1
4202e342:	02878ab3          	mul	s5,a5,s0
4202e346:	02b787b3          	mul	a5,a5,a1
4202e34a:	02b705b3          	mul	a1,a4,a1
4202e34e:	02870433          	mul	s0,a4,s0
4202e352:	029704b3          	mul	s1,a4,s1
4202e356:	02670733          	mul	a4,a4,t1
4202e35a:	9c1e                	add	s8,s8,t2
4202e35c:	9ff2                	add	t6,t6,t3
4202e35e:	0609                	addi	a2,a2,2
4202e360:	9ed6                	add	t4,t4,s5
4202e362:	9d3e                	add	s10,s10,a5
4202e364:	98ae                	add	a7,a7,a1
4202e366:	92a2                	add	t0,t0,s0
4202e368:	9826                	add	a6,a6,s1
4202e36a:	90ba                	add	ra,ra,a4
4202e36c:	0685                	addi	a3,a3,1
4202e36e:	fbe61ae3          	bne	a2,t5,4202e322 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x1a8>
4202e372:	53a6                	lw	t2,104(sp)
4202e374:	93de                	add	t2,t2,s7
4202e376:	039a                	slli	t2,t2,0x6
4202e378:	00a3e6b3          	or	a3,t2,a0
4202e37c:	00c6d613          	srli	a2,a3,0xc
4202e380:	72061463          	bnez	a2,4202eaa8 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x92e>
4202e384:	45d6                	lw	a1,84(sp)
4202e386:	4190                	lw	a2,0(a1)
4202e388:	45e6                	lw	a1,88(sp)
4202e38a:	0005ae03          	lw	t3,0(a1)
4202e38e:	fff64313          	not	t1,a2
4202e392:	40c00633          	neg	a2,a2
4202e396:	03ae1ab3          	mulh	s5,t3,s10
4202e39a:	03ae0433          	mul	s0,t3,s10
4202e39e:	4585                	li	a1,1
4202e3a0:	00659333          	sll	t1,a1,t1
4202e3a4:	01f67d13          	andi	s10,a2,31
4202e3a8:	01940633          	add	a2,s0,s9
4202e3ac:	00863433          	sltu	s0,a2,s0
4202e3b0:	827d                	srli	a2,a2,0x1f
4202e3b2:	9456                	add	s0,s0,s5
4202e3b4:	0406                	slli	s0,s0,0x1
4202e3b6:	8e41                	or	a2,a2,s0
4202e3b8:	847d                	srai	s0,s0,0x1f
4202e3ba:	961a                	add	a2,a2,t1
4202e3bc:	9622                	add	a2,a2,s0
4202e3be:	41a65633          	sra	a2,a2,s10
4202e3c2:	1679                	addi	a2,a2,-2
4202e3c4:	07f00413          	li	s0,127
4202e3c8:	02865163          	bge	a2,s0,4202e3ea <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x270>
4202e3cc:	f8000413          	li	s0,-128
4202e3d0:	02c45363          	bge	s0,a2,4202e3f6 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x27c>
4202e3d4:	00db0bb3          	add	s7,s6,a3
4202e3d8:	45f6                	lw	a1,92(sp)
4202e3da:	00b3e6b3          	or	a3,t2,a1
4202e3de:	00c6d413          	srli	s0,a3,0xc
4202e3e2:	00cb8023          	sb	a2,0(s7)
4202e3e6:	c40d                	beqz	s0,4202e410 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x296>
4202e3e8:	ad61                	j	4202ea80 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x906>
4202e3ea:	07f00613          	li	a2,127
4202e3ee:	f8000413          	li	s0,-128
4202e3f2:	fec441e3          	blt	s0,a2,4202e3d4 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x25a>
4202e3f6:	f8000613          	li	a2,-128
4202e3fa:	00db0bb3          	add	s7,s6,a3
4202e3fe:	45f6                	lw	a1,92(sp)
4202e400:	00b3e6b3          	or	a3,t2,a1
4202e404:	00c6d413          	srli	s0,a3,0xc
4202e408:	00cb8023          	sb	a2,0(s7)
4202e40c:	66041a63          	bnez	s0,4202ea80 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x906>
4202e410:	03de1ab3          	mulh	s5,t3,t4
4202e414:	03de0433          	mul	s0,t3,t4
4202e418:	01940633          	add	a2,s0,s9
4202e41c:	00863433          	sltu	s0,a2,s0
4202e420:	827d                	srli	a2,a2,0x1f
4202e422:	9456                	add	s0,s0,s5
4202e424:	0406                	slli	s0,s0,0x1
4202e426:	8e41                	or	a2,a2,s0
4202e428:	847d                	srai	s0,s0,0x1f
4202e42a:	961a                	add	a2,a2,t1
4202e42c:	9622                	add	a2,a2,s0
4202e42e:	41a65633          	sra	a2,a2,s10
4202e432:	1679                	addi	a2,a2,-2
4202e434:	07f00413          	li	s0,127
4202e438:	02865163          	bge	a2,s0,4202e45a <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x2e0>
4202e43c:	f8000413          	li	s0,-128
4202e440:	02c45363          	bge	s0,a2,4202e466 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x2ec>
4202e444:	00db0eb3          	add	t4,s6,a3
4202e448:	45c6                	lw	a1,80(sp)
4202e44a:	00b3e6b3          	or	a3,t2,a1
4202e44e:	00c6d413          	srli	s0,a3,0xc
4202e452:	00ce8023          	sb	a2,0(t4)
4202e456:	c40d                	beqz	s0,4202e480 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x306>
4202e458:	a595                	j	4202eabc <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x942>
4202e45a:	07f00613          	li	a2,127
4202e45e:	f8000413          	li	s0,-128
4202e462:	fec441e3          	blt	s0,a2,4202e444 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x2ca>
4202e466:	f8000613          	li	a2,-128
4202e46a:	00db0eb3          	add	t4,s6,a3
4202e46e:	45c6                	lw	a1,80(sp)
4202e470:	00b3e6b3          	or	a3,t2,a1
4202e474:	00c6d413          	srli	s0,a3,0xc
4202e478:	00ce8023          	sb	a2,0(t4)
4202e47c:	64041063          	bnez	s0,4202eabc <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x942>
4202e480:	03fe1633          	mulh	a2,t3,t6
4202e484:	03fe0433          	mul	s0,t3,t6
4202e488:	019404b3          	add	s1,s0,s9
4202e48c:	0084b433          	sltu	s0,s1,s0
4202e490:	80fd                	srli	s1,s1,0x1f
4202e492:	9622                	add	a2,a2,s0
4202e494:	0606                	slli	a2,a2,0x1
4202e496:	8cd1                	or	s1,s1,a2
4202e498:	867d                	srai	a2,a2,0x1f
4202e49a:	949a                	add	s1,s1,t1
4202e49c:	9626                	add	a2,a2,s1
4202e49e:	41a65633          	sra	a2,a2,s10
4202e4a2:	1679                	addi	a2,a2,-2
4202e4a4:	07f00413          	li	s0,127
4202e4a8:	02865163          	bge	a2,s0,4202e4ca <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x350>
4202e4ac:	f8000413          	li	s0,-128
4202e4b0:	02c45363          	bge	s0,a2,4202e4d6 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x35c>
4202e4b4:	00db0ab3          	add	s5,s6,a3
4202e4b8:	45b6                	lw	a1,76(sp)
4202e4ba:	00b3e6b3          	or	a3,t2,a1
4202e4be:	00c6d493          	srli	s1,a3,0xc
4202e4c2:	00ca8023          	sb	a2,0(s5)
4202e4c6:	c48d                	beqz	s1,4202e4f0 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x376>
4202e4c8:	a3f1                	j	4202ea94 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x91a>
4202e4ca:	07f00613          	li	a2,127
4202e4ce:	f8000413          	li	s0,-128
4202e4d2:	fec441e3          	blt	s0,a2,4202e4b4 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x33a>
4202e4d6:	f8000613          	li	a2,-128
4202e4da:	00db0ab3          	add	s5,s6,a3
4202e4de:	45b6                	lw	a1,76(sp)
4202e4e0:	00b3e6b3          	or	a3,t2,a1
4202e4e4:	00c6d493          	srli	s1,a3,0xc
4202e4e8:	00ca8023          	sb	a2,0(s5)
4202e4ec:	5a049463          	bnez	s1,4202ea94 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x91a>
4202e4f0:	038e1633          	mulh	a2,t3,s8
4202e4f4:	038e07b3          	mul	a5,t3,s8
4202e4f8:	019784b3          	add	s1,a5,s9
4202e4fc:	00f4b7b3          	sltu	a5,s1,a5
4202e500:	80fd                	srli	s1,s1,0x1f
4202e502:	963e                	add	a2,a2,a5
4202e504:	0606                	slli	a2,a2,0x1
4202e506:	8cd1                	or	s1,s1,a2
4202e508:	867d                	srai	a2,a2,0x1f
4202e50a:	9326                	add	t1,t1,s1
4202e50c:	961a                	add	a2,a2,t1
4202e50e:	41a65633          	sra	a2,a2,s10
4202e512:	1679                	addi	a2,a2,-2
4202e514:	07f00d13          	li	s10,127
4202e518:	01a64463          	blt	a2,s10,4202e520 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x3a6>
4202e51c:	07f00613          	li	a2,127
4202e520:	f8000313          	li	t1,-128
4202e524:	00c34463          	blt	t1,a2,4202e52c <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x3b2>
4202e528:	f8000613          	li	a2,-128
4202e52c:	45a6                	lw	a1,72(sp)
4202e52e:	419c                	lw	a5,0(a1)
4202e530:	00db03b3          	add	t2,s6,a3
4202e534:	4596                	lw	a1,68(sp)
4202e536:	4194                	lw	a3,0(a1)
4202e538:	03179e33          	mulh	t3,a5,a7
4202e53c:	03178433          	mul	s0,a5,a7
4202e540:	019404b3          	add	s1,s0,s9
4202e544:	0084b433          	sltu	s0,s1,s0
4202e548:	9e22                	add	t3,t3,s0
4202e54a:	fff6c413          	not	s0,a3
4202e54e:	40d008b3          	neg	a7,a3
4202e552:	4585                	li	a1,1
4202e554:	00859c33          	sll	s8,a1,s0
4202e558:	80fd                	srli	s1,s1,0x1f
4202e55a:	0e06                	slli	t3,t3,0x1
4202e55c:	01c4e4b3          	or	s1,s1,t3
4202e560:	41fe5693          	srai	a3,t3,0x1f
4202e564:	94e2                	add	s1,s1,s8
4202e566:	96a6                	add	a3,a3,s1
4202e568:	4116d4b3          	sra	s1,a3,a7
4202e56c:	14f9                	addi	s1,s1,-2
4202e56e:	00c38023          	sb	a2,0(t2)
4202e572:	01a4c463          	blt	s1,s10,4202e57a <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x400>
4202e576:	07f00493          	li	s1,127
4202e57a:	00934463          	blt	t1,s1,4202e582 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x408>
4202e57e:	f8000493          	li	s1,-128
4202e582:	02579633          	mulh	a2,a5,t0
4202e586:	025786b3          	mul	a3,a5,t0
4202e58a:	01968433          	add	s0,a3,s9
4202e58e:	00d436b3          	sltu	a3,s0,a3
4202e592:	9636                	add	a2,a2,a3
4202e594:	807d                	srli	s0,s0,0x1f
4202e596:	0606                	slli	a2,a2,0x1
4202e598:	8c51                	or	s0,s0,a2
4202e59a:	867d                	srai	a2,a2,0x1f
4202e59c:	9462                	add	s0,s0,s8
4202e59e:	9622                	add	a2,a2,s0
4202e5a0:	41165633          	sra	a2,a2,a7
4202e5a4:	1679                	addi	a2,a2,-2
4202e5a6:	009b80a3          	sb	s1,1(s7)
4202e5aa:	01a64463          	blt	a2,s10,4202e5b2 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x438>
4202e5ae:	07f00613          	li	a2,127
4202e5b2:	00c34463          	blt	t1,a2,4202e5ba <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x440>
4202e5b6:	f8000613          	li	a2,-128
4202e5ba:	030796b3          	mulh	a3,a5,a6
4202e5be:	03078733          	mul	a4,a5,a6
4202e5c2:	019704b3          	add	s1,a4,s9
4202e5c6:	00e4b733          	sltu	a4,s1,a4
4202e5ca:	96ba                	add	a3,a3,a4
4202e5cc:	80fd                	srli	s1,s1,0x1f
4202e5ce:	0686                	slli	a3,a3,0x1
4202e5d0:	8cd5                	or	s1,s1,a3
4202e5d2:	86fd                	srai	a3,a3,0x1f
4202e5d4:	94e2                	add	s1,s1,s8
4202e5d6:	96a6                	add	a3,a3,s1
4202e5d8:	4116d733          	sra	a4,a3,a7
4202e5dc:	1779                	addi	a4,a4,-2
4202e5de:	00ce80a3          	sb	a2,1(t4)
4202e5e2:	01a74463          	blt	a4,s10,4202e5ea <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x470>
4202e5e6:	07f00713          	li	a4,127
4202e5ea:	00e34463          	blt	t1,a4,4202e5f2 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x478>
4202e5ee:	f8000713          	li	a4,-128
4202e5f2:	02179633          	mulh	a2,a5,ra
4202e5f6:	021785b3          	mul	a1,a5,ra
4202e5fa:	019586b3          	add	a3,a1,s9
4202e5fe:	00b6b5b3          	sltu	a1,a3,a1
4202e602:	95b2                	add	a1,a1,a2
4202e604:	82fd                	srli	a3,a3,0x1f
4202e606:	0586                	slli	a1,a1,0x1
4202e608:	8ecd                	or	a3,a3,a1
4202e60a:	85fd                	srai	a1,a1,0x1f
4202e60c:	96e2                	add	a3,a3,s8
4202e60e:	95b6                	add	a1,a1,a3
4202e610:	4115d5b3          	sra	a1,a1,a7
4202e614:	15f9                	addi	a1,a1,-2
4202e616:	00ea80a3          	sb	a4,1(s5)
4202e61a:	01a5c463          	blt	a1,s10,4202e622 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x4a8>
4202e61e:	07f00593          	li	a1,127
4202e622:	c8b349e3          	blt	t1,a1,4202e2b4 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x13a>
4202e626:	f8000593          	li	a1,-128
4202e62a:	b169                	j	4202e2b4 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x13a>
4202e62c:	4601                	li	a2,0
4202e62e:	24000513          	li	a0,576
4202e632:	45c6                	lw	a1,80(sp)
4202e634:	02a587b3          	mul	a5,a1,a0
4202e638:	06c10a13          	addi	s4,sp,108
4202e63c:	16c10c13          	addi	s8,sp,364
4202e640:	1568                	addi	a0,sp,684
4202e642:	a82d                	j	4202e67c <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x502>
4202e644:	08000593          	li	a1,128
4202e648:	04000613          	li	a2,64
4202e64c:	8536                	mv	a0,a3
4202e64e:	fdfd2097          	auipc	ra,0xfdfd2
4202e652:	d06080e7          	jalr	-762(ra) # 40000354 <memset>
4202e656:	47d6                	lw	a5,84(sp)
4202e658:	dc078793          	addi	a5,a5,-576
4202e65c:	240c0c13          	addi	s8,s8,576
4202e660:	2407b513          	sltiu	a0,a5,576
4202e664:	157d                	addi	a0,a0,-1
4202e666:	24057513          	andi	a0,a0,576
4202e66a:	4a66                	lw	s4,88(sp)
4202e66c:	9552                	add	a0,a0,s4
4202e66e:	4676                	lw	a2,92(sp)
4202e670:	0605                	addi	a2,a2,1
4202e672:	5726                	lw	a4,104(sp)
4202e674:	24000593          	li	a1,576
4202e678:	b8b7e1e3          	bltu	a5,a1,4202e1fa <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x80>
4202e67c:	85aa                	mv	a1,a0
4202e67e:	f00c0693          	addi	a3,s8,-256
4202e682:	9732                	add	a4,a4,a2
4202e684:	00767d13          	andi	s10,a2,7
4202e688:	d0ba                	sw	a4,96(sp)
4202e68a:	00375d93          	srli	s11,a4,0x3
4202e68e:	fffd0513          	addi	a0,s10,-1
4202e692:	fffd8993          	addi	s3,s11,-1
4202e696:	009d9b93          	slli	s7,s11,0x9
4202e69a:	dc0b8a93          	addi	s5,s7,-576
4202e69e:	e00b8493          	addi	s1,s7,-512
4202e6a2:	d2aa                	sw	a0,100(sp)
4202e6a4:	01356533          	or	a0,a0,s3
4202e6a8:	006d1913          	slli	s2,s10,0x6
4202e6ac:	4421                	li	s0,8
4202e6ae:	ceb2                	sw	a2,92(sp)
4202e6b0:	cabe                	sw	a5,84(sp)
4202e6b2:	ccae                	sw	a1,88(sp)
4202e6b4:	02857f63          	bgeu	a0,s0,4202e6f2 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x578>
4202e6b8:	01590533          	add	a0,s2,s5
4202e6bc:	00996733          	or	a4,s2,s1
4202e6c0:	38a76c63          	bltu	a4,a0,4202ea58 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8de>
4202e6c4:	5a22                	lw	s4,40(sp)
4202e6c6:	6585                	lui	a1,0x1
4202e6c8:	36e5ee63          	bltu	a1,a4,4202ea44 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8ca>
4202e6cc:	00aa05b3          	add	a1,s4,a0
4202e6d0:	04000613          	li	a2,64
4202e6d4:	8536                	mv	a0,a3
4202e6d6:	fdfd2097          	auipc	ra,0xfdfd2
4202e6da:	c82080e7          	jalr	-894(ra) # 40000358 <memcpy>
4202e6de:	f40c0693          	addi	a3,s8,-192
4202e6e2:	04090713          	addi	a4,s2,64
4202e6e6:	01570533          	add	a0,a4,s5
4202e6ea:	9726                	add	a4,a4,s1
4202e6ec:	02a77863          	bgeu	a4,a0,4202e71c <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x5a2>
4202e6f0:	a6a5                	j	4202ea58 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8de>
4202e6f2:	08000593          	li	a1,128
4202e6f6:	04000613          	li	a2,64
4202e6fa:	8536                	mv	a0,a3
4202e6fc:	fdfd2097          	auipc	ra,0xfdfd2
4202e700:	c58080e7          	jalr	-936(ra) # 40000354 <memset>
4202e704:	0c89fd63          	bgeu	s3,s0,4202e7de <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x664>
4202e708:	040a0693          	addi	a3,s4,64
4202e70c:	5a22                	lw	s4,40(sp)
4202e70e:	04090713          	addi	a4,s2,64
4202e712:	01570533          	add	a0,a4,s5
4202e716:	9726                	add	a4,a4,s1
4202e718:	34a76063          	bltu	a4,a0,4202ea58 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8de>
4202e71c:	6585                	lui	a1,0x1
4202e71e:	32e5e363          	bltu	a1,a4,4202ea44 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8ca>
4202e722:	00aa05b3          	add	a1,s4,a0
4202e726:	04000613          	li	a2,64
4202e72a:	8536                	mv	a0,a3
4202e72c:	fdfd2097          	auipc	ra,0xfdfd2
4202e730:	c2c080e7          	jalr	-980(ra) # 40000358 <memcpy>
4202e734:	0d05                	addi	s10,s10,1
4202e736:	013d6533          	or	a0,s10,s3
4202e73a:	f80c0693          	addi	a3,s8,-128
4202e73e:	4421                	li	s0,8
4202e740:	0c857263          	bgeu	a0,s0,4202e804 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x68a>
4202e744:	08090713          	addi	a4,s2,128
4202e748:	01570533          	add	a0,a4,s5
4202e74c:	9726                	add	a4,a4,s1
4202e74e:	30a76563          	bltu	a4,a0,4202ea58 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8de>
4202e752:	6585                	lui	a1,0x1
4202e754:	2ee5e863          	bltu	a1,a4,4202ea44 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8ca>
4202e758:	00aa05b3          	add	a1,s4,a0
4202e75c:	04000613          	li	a2,64
4202e760:	8536                	mv	a0,a3
4202e762:	fdfd2097          	auipc	ra,0xfdfd2
4202e766:	bf6080e7          	jalr	-1034(ra) # 40000358 <memcpy>
4202e76a:	4421                	li	s0,8
4202e76c:	fc0b8493          	addi	s1,s7,-64
4202e770:	5516                	lw	a0,100(sp)
4202e772:	01b56533          	or	a0,a0,s11
4202e776:	fc0c0693          	addi	a3,s8,-64
4202e77a:	0a856763          	bltu	a0,s0,4202e828 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x6ae>
4202e77e:	08000593          	li	a1,128
4202e782:	04000613          	li	a2,64
4202e786:	8536                	mv	a0,a3
4202e788:	fdfd2097          	auipc	ra,0xfdfd2
4202e78c:	bcc080e7          	jalr	-1076(ra) # 40000354 <memset>
4202e790:	04000513          	li	a0,64
4202e794:	5586                	lw	a1,96(sp)
4202e796:	0ca5f163          	bgeu	a1,a0,4202e858 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x6de>
4202e79a:	01790533          	add	a0,s2,s7
4202e79e:	04050713          	addi	a4,a0,64
4202e7a2:	6505                	lui	a0,0x1
4202e7a4:	2ae56063          	bltu	a0,a4,4202ea44 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8ca>
4202e7a8:	017965b3          	or	a1,s2,s7
4202e7ac:	95d2                	add	a1,a1,s4
4202e7ae:	04000613          	li	a2,64
4202e7b2:	8562                	mv	a0,s8
4202e7b4:	fdfd2097          	auipc	ra,0xfdfd2
4202e7b8:	ba4080e7          	jalr	-1116(ra) # 40000358 <memcpy>
4202e7bc:	4421                	li	s0,8
4202e7be:	01bd6533          	or	a0,s10,s11
4202e7c2:	040c0693          	addi	a3,s8,64
4202e7c6:	0a856863          	bltu	a0,s0,4202e876 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x6fc>
4202e7ca:	08000593          	li	a1,128
4202e7ce:	04000613          	li	a2,64
4202e7d2:	8536                	mv	a0,a3
4202e7d4:	fdfd2097          	auipc	ra,0xfdfd2
4202e7d8:	b80080e7          	jalr	-1152(ra) # 40000354 <memset>
4202e7dc:	a0c9                	j	4202e89e <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x724>
4202e7de:	f40c0513          	addi	a0,s8,-192
4202e7e2:	08000593          	li	a1,128
4202e7e6:	04000613          	li	a2,64
4202e7ea:	fdfd2097          	auipc	ra,0xfdfd2
4202e7ee:	b6a080e7          	jalr	-1174(ra) # 40000354 <memset>
4202e7f2:	5a22                	lw	s4,40(sp)
4202e7f4:	0d05                	addi	s10,s10,1
4202e7f6:	013d6533          	or	a0,s10,s3
4202e7fa:	f80c0693          	addi	a3,s8,-128
4202e7fe:	4421                	li	s0,8
4202e800:	f48562e3          	bltu	a0,s0,4202e744 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x5ca>
4202e804:	08000593          	li	a1,128
4202e808:	04000613          	li	a2,64
4202e80c:	8536                	mv	a0,a3
4202e80e:	fdfd2097          	auipc	ra,0xfdfd2
4202e812:	b46080e7          	jalr	-1210(ra) # 40000354 <memset>
4202e816:	fc0b8493          	addi	s1,s7,-64
4202e81a:	5516                	lw	a0,100(sp)
4202e81c:	01b56533          	or	a0,a0,s11
4202e820:	fc0c0693          	addi	a3,s8,-64
4202e824:	f4857de3          	bgeu	a0,s0,4202e77e <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x604>
4202e828:	00990533          	add	a0,s2,s1
4202e82c:	01796733          	or	a4,s2,s7
4202e830:	22a76463          	bltu	a4,a0,4202ea58 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8de>
4202e834:	6585                	lui	a1,0x1
4202e836:	20e5e763          	bltu	a1,a4,4202ea44 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8ca>
4202e83a:	00aa05b3          	add	a1,s4,a0
4202e83e:	04000613          	li	a2,64
4202e842:	8536                	mv	a0,a3
4202e844:	fdfd2097          	auipc	ra,0xfdfd2
4202e848:	b14080e7          	jalr	-1260(ra) # 40000358 <memcpy>
4202e84c:	4421                	li	s0,8
4202e84e:	04000513          	li	a0,64
4202e852:	5586                	lw	a1,96(sp)
4202e854:	f4a5e3e3          	bltu	a1,a0,4202e79a <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x620>
4202e858:	08000593          	li	a1,128
4202e85c:	04000613          	li	a2,64
4202e860:	8562                	mv	a0,s8
4202e862:	fdfd2097          	auipc	ra,0xfdfd2
4202e866:	af2080e7          	jalr	-1294(ra) # 40000354 <memset>
4202e86a:	01bd6533          	or	a0,s10,s11
4202e86e:	040c0693          	addi	a3,s8,64
4202e872:	f4857ce3          	bgeu	a0,s0,4202e7ca <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x650>
4202e876:	08090713          	addi	a4,s2,128
4202e87a:	00970533          	add	a0,a4,s1
4202e87e:	975e                	add	a4,a4,s7
4202e880:	1ca76c63          	bltu	a4,a0,4202ea58 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8de>
4202e884:	6585                	lui	a1,0x1
4202e886:	1ae5ef63          	bltu	a1,a4,4202ea44 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8ca>
4202e88a:	00aa05b3          	add	a1,s4,a0
4202e88e:	04000613          	li	a2,64
4202e892:	8536                	mv	a0,a3
4202e894:	fdfd2097          	auipc	ra,0xfdfd2
4202e898:	ac4080e7          	jalr	-1340(ra) # 40000358 <memcpy>
4202e89c:	4421                	li	s0,8
4202e89e:	0d85                	addi	s11,s11,1
4202e8a0:	1c0b8993          	addi	s3,s7,448
4202e8a4:	200b8493          	addi	s1,s7,512
4202e8a8:	5516                	lw	a0,100(sp)
4202e8aa:	01b56533          	or	a0,a0,s11
4202e8ae:	080c0693          	addi	a3,s8,128
4202e8b2:	04857c63          	bgeu	a0,s0,4202e90a <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x790>
4202e8b6:	01390533          	add	a0,s2,s3
4202e8ba:	00996733          	or	a4,s2,s1
4202e8be:	18a76d63          	bltu	a4,a0,4202ea58 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8de>
4202e8c2:	6585                	lui	a1,0x1
4202e8c4:	18e5e063          	bltu	a1,a4,4202ea44 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8ca>
4202e8c8:	00aa05b3          	add	a1,s4,a0
4202e8cc:	04000613          	li	a2,64
4202e8d0:	8536                	mv	a0,a3
4202e8d2:	fdfd2097          	auipc	ra,0xfdfd2
4202e8d6:	a86080e7          	jalr	-1402(ra) # 40000358 <memcpy>
4202e8da:	4421                	li	s0,8
4202e8dc:	0c0c0693          	addi	a3,s8,192
4202e8e0:	03800513          	li	a0,56
4202e8e4:	5586                	lw	a1,96(sp)
4202e8e6:	04a5e263          	bltu	a1,a0,4202e92a <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x7b0>
4202e8ea:	08000593          	li	a1,128
4202e8ee:	04000613          	li	a2,64
4202e8f2:	8536                	mv	a0,a3
4202e8f4:	fdfd2097          	auipc	ra,0xfdfd2
4202e8f8:	a60080e7          	jalr	-1440(ra) # 40000354 <memset>
4202e8fc:	01bd6533          	or	a0,s10,s11
4202e900:	100c0693          	addi	a3,s8,256
4202e904:	d48570e3          	bgeu	a0,s0,4202e644 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x4ca>
4202e908:	a899                	j	4202e95e <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x7e4>
4202e90a:	08000593          	li	a1,128
4202e90e:	04000613          	li	a2,64
4202e912:	8536                	mv	a0,a3
4202e914:	fdfd2097          	auipc	ra,0xfdfd2
4202e918:	a40080e7          	jalr	-1472(ra) # 40000354 <memset>
4202e91c:	0c0c0693          	addi	a3,s8,192
4202e920:	03800513          	li	a0,56
4202e924:	5586                	lw	a1,96(sp)
4202e926:	fca5f2e3          	bgeu	a1,a0,4202e8ea <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x770>
4202e92a:	04090713          	addi	a4,s2,64
4202e92e:	01370533          	add	a0,a4,s3
4202e932:	9726                	add	a4,a4,s1
4202e934:	1334e263          	bltu	s1,s3,4202ea58 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8de>
4202e938:	6585                	lui	a1,0x1
4202e93a:	10e5e563          	bltu	a1,a4,4202ea44 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8ca>
4202e93e:	00aa05b3          	add	a1,s4,a0
4202e942:	04000613          	li	a2,64
4202e946:	8536                	mv	a0,a3
4202e948:	fdfd2097          	auipc	ra,0xfdfd2
4202e94c:	a10080e7          	jalr	-1520(ra) # 40000358 <memcpy>
4202e950:	4421                	li	s0,8
4202e952:	01bd6533          	or	a0,s10,s11
4202e956:	100c0693          	addi	a3,s8,256
4202e95a:	ce8575e3          	bgeu	a0,s0,4202e644 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x4ca>
4202e95e:	08090713          	addi	a4,s2,128
4202e962:	01370533          	add	a0,a4,s3
4202e966:	9726                	add	a4,a4,s1
4202e968:	0ea76863          	bltu	a4,a0,4202ea58 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8de>
4202e96c:	6585                	lui	a1,0x1
4202e96e:	0ce5eb63          	bltu	a1,a4,4202ea44 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x8ca>
4202e972:	00aa05b3          	add	a1,s4,a0
4202e976:	04000613          	li	a2,64
4202e97a:	8536                	mv	a0,a3
4202e97c:	fdfd2097          	auipc	ra,0xfdfd2
4202e980:	9dc080e7          	jalr	-1572(ra) # 40000358 <memcpy>
4202e984:	b9c9                	j	4202e656 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x4dc>
4202e986:	45b2                	lw	a1,12(sp)
4202e988:	fff58513          	addi	a0,a1,-1 # fff <RvExcFrameSize+0xf6b>
4202e98c:	0741                	addi	a4,a4,16
4202e98e:	04000613          	li	a2,64
4202e992:	840598e3          	bnez	a1,4202e1e2 <_ZN7kernels13r_tile16_b2p47layer_93run17hc57b49ce7a02a0e1E+0x68>
4202e996:	6509                	lui	a0,0x2
4202e998:	3a050513          	addi	a0,a0,928 # 23a0 <RvExcFrameSize+0x230c>
4202e99c:	912a                	add	sp,sp,a0
4202e99e:	50fe                	lw	ra,252(sp)
4202e9a0:	546e                	lw	s0,248(sp)
4202e9a2:	54de                	lw	s1,244(sp)
4202e9a4:	594e                	lw	s2,240(sp)
4202e9a6:	59be                	lw	s3,236(sp)
4202e9a8:	5a2e                	lw	s4,232(sp)
4202e9aa:	5a9e                	lw	s5,228(sp)
4202e9ac:	5b0e                	lw	s6,224(sp)
4202e9ae:	4bfe                	lw	s7,220(sp)
4202e9b0:	4c6e                	lw	s8,216(sp)
4202e9b2:	4cde                	lw	s9,212(sp)
4202e9b4:	4d4e                	lw	s10,208(sp)
4202e9b6:	4dbe                	lw	s11,204(sp)
4202e9b8:	6111                	addi	sp,sp,256
4202e9ba:	8082                	ret
4202e9bc:	3c2b7637          	lui	a2,0x3c2b7
4202e9c0:	c0c60613          	addi	a2,a2,-1012 # 3c2b6c0c <c7_weights_0+0x1e0c>
4202e9c4:	8536                	mv	a0,a3
4202e9c6:	85ba                	mv	a1,a4
4202e9c8:	0000b097          	auipc	ra,0xb
4202e9cc:	d42080e7          	jalr	-702(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
4202e9d0:	3c2b7637          	lui	a2,0x3c2b7
4202e9d4:	c0c60613          	addi	a2,a2,-1012 # 3c2b6c0c <c7_weights_0+0x1e0c>
4202e9d8:	45a5                	li	a1,9
4202e9da:	05aa                	slli	a1,a1,0xa
4202e9dc:	853a                	mv	a0,a4
4202e9de:	0000b097          	auipc	ra,0xb
4202e9e2:	d1c080e7          	jalr	-740(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
4202e9e6:	3c2b7637          	lui	a2,0x3c2b7
4202e9ea:	c1c60613          	addi	a2,a2,-996 # 3c2b6c1c <c7_weights_0+0x1e1c>
4202e9ee:	853a                	mv	a0,a4
4202e9f0:	0000b097          	auipc	ra,0xb
4202e9f4:	d1a080e7          	jalr	-742(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
4202e9f8:	3c2b7637          	lui	a2,0x3c2b7
4202e9fc:	c1c60613          	addi	a2,a2,-996 # 3c2b6c1c <c7_weights_0+0x1e1c>
4202ea00:	46a5                	li	a3,9
4202ea02:	06aa                	slli	a3,a3,0xa
4202ea04:	852e                	mv	a0,a1
4202ea06:	85b6                	mv	a1,a3
4202ea08:	0000b097          	auipc	ra,0xb
4202ea0c:	cf2080e7          	jalr	-782(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
4202ea10:	6c068513          	addi	a0,a3,1728
4202ea14:	3c2b7637          	lui	a2,0x3c2b7
4202ea18:	c2c60613          	addi	a2,a2,-980 # 3c2b6c2c <c7_weights_0+0x1e2c>
4202ea1c:	45a5                	li	a1,9
4202ea1e:	05aa                	slli	a1,a1,0xa
4202ea20:	0000b097          	auipc	ra,0xb
4202ea24:	cda080e7          	jalr	-806(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
4202ea28:	7ff68513          	addi	a0,a3,2047
4202ea2c:	10150513          	addi	a0,a0,257
4202ea30:	3c2b7637          	lui	a2,0x3c2b7
4202ea34:	c3c60613          	addi	a2,a2,-964 # 3c2b6c3c <c7_weights_0+0x1e3c>
4202ea38:	45a5                	li	a1,9
4202ea3a:	05aa                	slli	a1,a1,0xa
4202ea3c:	0000b097          	auipc	ra,0xb
4202ea40:	cbe080e7          	jalr	-834(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
4202ea44:	3c2b7637          	lui	a2,0x3c2b7
4202ea48:	c9c60613          	addi	a2,a2,-868 # 3c2b6c9c <c7_weights_0+0x1e9c>
4202ea4c:	6585                	lui	a1,0x1
4202ea4e:	853a                	mv	a0,a4
4202ea50:	0000b097          	auipc	ra,0xb
4202ea54:	caa080e7          	jalr	-854(ra) # 420396fa <_ZN4core5slice5index24slice_end_index_len_fail17h1efda5756204f1e3E>
4202ea58:	3c2b7637          	lui	a2,0x3c2b7
4202ea5c:	c9c60613          	addi	a2,a2,-868 # 3c2b6c9c <c7_weights_0+0x1e9c>
4202ea60:	85ba                	mv	a1,a4
4202ea62:	0000b097          	auipc	ra,0xb
4202ea66:	ca8080e7          	jalr	-856(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
4202ea6a:	48068593          	addi	a1,a3,1152
4202ea6e:	3c2b7637          	lui	a2,0x3c2b7
4202ea72:	bfc60613          	addi	a2,a2,-1028 # 3c2b6bfc <c7_weights_0+0x1dfc>
4202ea76:	8536                	mv	a0,a3
4202ea78:	0000b097          	auipc	ra,0xb
4202ea7c:	c92080e7          	jalr	-878(ra) # 4203970a <_ZN4core5slice5index22slice_index_order_fail17he43fe295c42f4f0cE>
4202ea80:	3c2b7637          	lui	a2,0x3c2b7
4202ea84:	c6c60613          	addi	a2,a2,-916 # 3c2b6c6c <c7_weights_0+0x1e6c>
4202ea88:	6585                	lui	a1,0x1
4202ea8a:	8536                	mv	a0,a3
4202ea8c:	0000b097          	auipc	ra,0xb
4202ea90:	964080e7          	jalr	-1692(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
4202ea94:	3c2b7637          	lui	a2,0x3c2b7
4202ea98:	c8c60613          	addi	a2,a2,-884 # 3c2b6c8c <c7_weights_0+0x1e8c>
4202ea9c:	6585                	lui	a1,0x1
4202ea9e:	8536                	mv	a0,a3
4202eaa0:	0000b097          	auipc	ra,0xb
4202eaa4:	950080e7          	jalr	-1712(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
4202eaa8:	3c2b7637          	lui	a2,0x3c2b7
4202eaac:	c5c60613          	addi	a2,a2,-932 # 3c2b6c5c <c7_weights_0+0x1e5c>
4202eab0:	6585                	lui	a1,0x1
4202eab2:	8536                	mv	a0,a3
4202eab4:	0000b097          	auipc	ra,0xb
4202eab8:	93c080e7          	jalr	-1732(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
4202eabc:	3c2b7637          	lui	a2,0x3c2b7
4202eac0:	c7c60613          	addi	a2,a2,-900 # 3c2b6c7c <c7_weights_0+0x1e7c>
4202eac4:	6585                	lui	a1,0x1
4202eac6:	8536                	mv	a0,a3
4202eac8:	0000b097          	auipc	ra,0xb
4202eacc:	928080e7          	jalr	-1752(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
4202ead0:	3c2b7637          	lui	a2,0x3c2b7
4202ead4:	c4c60613          	addi	a2,a2,-948 # 3c2b6c4c <c7_weights_0+0x1e4c>
4202ead8:	04000593          	li	a1,64
4202eadc:	0000b097          	auipc	ra,0xb
4202eae0:	914080e7          	jalr	-1772(ra) # 420393f0 <_ZN4core9panicking18panic_bounds_check17habb6b324eaf98cfdE>
