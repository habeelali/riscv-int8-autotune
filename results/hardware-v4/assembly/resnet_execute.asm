
hardware/esp32c3-v4/build/riscv_int8_generalize.elf:     file format elf32-littleriscv


Disassembly of section .iram0.text:

Disassembly of section .flash.text:

4200c418 <resnet_execute>:
4200c418:	7159                	addi	sp,sp,-112
4200c41a:	d4a2                	sw	s0,104(sp)
4200c41c:	842a                	mv	s0,a0
4200c41e:	4501                	li	a0,0
4200c420:	d686                	sw	ra,108(sp)
4200c422:	d0ca                	sw	s2,96(sp)
4200c424:	ccd2                	sw	s4,88(sp)
4200c426:	892e                	mv	s2,a1
4200c428:	cad6                	sw	s5,84(sp)
4200c42a:	c6de                	sw	s7,76(sp)
4200c42c:	8ab2                	mv	s5,a2
4200c42e:	d2a6                	sw	s1,100(sp)
4200c430:	cece                	sw	s3,92(sp)
4200c432:	c8da                	sw	s6,80(sp)
4200c434:	c4e2                	sw	s8,72(sp)
4200c436:	c2e6                	sw	s9,68(sp)
4200c438:	c0ea                	sw	s10,64(sp)
4200c43a:	de6e                	sw	s11,60(sp)
4200c43c:	9f1ff0ef          	jal	4200be2c <profile_begin>
4200c440:	85a2                	mv	a1,s0
4200c442:	4601                	li	a2,0
4200c444:	4505                	li	a0,1
4200c446:	9adff0ef          	jal	4200bdf2 <choose_backend>
4200c44a:	3c1de737          	lui	a4,0x3c1de
4200c44e:	00291b93          	slli	s7,s2,0x2
4200c452:	6b870713          	addi	a4,a4,1720 # 3c1de6b8 <r_images>
4200c456:	975e                	add	a4,a4,s7
4200c458:	4699                	li	a3,6
4200c45a:	00072a03          	lw	s4,0(a4)
4200c45e:	06d50263          	beq	a0,a3,4200c4c2 <resnet_execute+0xaa>
4200c462:	3c1ca737          	lui	a4,0x3c1ca
4200c466:	00651693          	slli	a3,a0,0x6
4200c46a:	64070713          	addi	a4,a4,1600 # 3c1ca640 <r_kernels>
4200c46e:	9736                	add	a4,a4,a3
4200c470:	469d                	li	a3,7
4200c472:	00072803          	lw	a6,0(a4)
4200c476:	10d50e63          	beq	a0,a3,4200c592 <resnet_execute+0x17a>
4200c47a:	4715                	li	a4,5
4200c47c:	00e51463          	bne	a0,a4,4200c484 <resnet_execute+0x6c>
4200c480:	47c0106f          	j	4200d8fc <resnet_execute+0x14e4>
4200c484:	4711                	li	a4,4
4200c486:	00e51463          	bne	a0,a4,4200c48e <resnet_execute+0x76>
4200c48a:	4220106f          	j	4200d8ac <resnet_execute+0x1494>
4200c48e:	470d                	li	a4,3
4200c490:	00e51463          	bne	a0,a4,4200c498 <resnet_execute+0x80>
4200c494:	4940106f          	j	4200d928 <resnet_execute+0x1510>
4200c498:	3c2656b7          	lui	a3,0x3c265
4200c49c:	3c265637          	lui	a2,0x3c265
4200c4a0:	3c2655b7          	lui	a1,0x3c265
4200c4a4:	3c265737          	lui	a4,0x3c265
4200c4a8:	3fc8fb37          	lui	s6,0x3fc8f
4200c4ac:	66068693          	addi	a3,a3,1632 # 3c265660 <r_mult_0>
4200c4b0:	6a060613          	addi	a2,a2,1696 # 3c2656a0 <r_bias_0>
4200c4b4:	6e058593          	addi	a1,a1,1760 # 3c2656e0 <r_weights_0>
4200c4b8:	62070713          	addi	a4,a4,1568 # 3c265620 <r_shift_0>
4200c4bc:	accb0993          	addi	s3,s6,-1332 # 3fc8eacc <padded>
4200c4c0:	a0b9                	j	4200c50e <resnet_execute+0xf6>
4200c4c2:	3fc8fb37          	lui	s6,0x3fc8f
4200c4c6:	6525                	lui	a0,0x9
4200c4c8:	accb0993          	addi	s3,s6,-1332 # 3fc8eacc <padded>
4200c4cc:	46050513          	addi	a0,a0,1120 # 9460 <RvExcFrameSize+0x93cc>
4200c4d0:	3c2a15b7          	lui	a1,0x3c2a1
4200c4d4:	f9858593          	addi	a1,a1,-104 # 3c2a0f98 <c6_weights_0>
4200c4d8:	1b000613          	li	a2,432
4200c4dc:	954e                	add	a0,a0,s3
4200c4de:	fdff4097          	auipc	ra,0xfdff4
4200c4e2:	e7a080e7          	jalr	-390(ra) # 40000358 <memcpy>
4200c4e6:	3c2a16b7          	lui	a3,0x3c2a1
4200c4ea:	3c2a1637          	lui	a2,0x3c2a1
4200c4ee:	3fc985b7          	lui	a1,0x3fc98
4200c4f2:	42030837          	lui	a6,0x42030
4200c4f6:	3c2a1737          	lui	a4,0x3c2a1
4200c4fa:	f1868693          	addi	a3,a3,-232 # 3c2a0f18 <c6_mult_0>
4200c4fe:	f5860613          	addi	a2,a2,-168 # 3c2a0f58 <c6_bias_0>
4200c502:	f2c58593          	addi	a1,a1,-212 # 3fc97f2c <padded+0x9460>
4200c506:	d5680813          	addi	a6,a6,-682 # 4202fd56 <r_tile16_b2p4_sram_layer_0>
4200c50a:	ed870713          	addi	a4,a4,-296 # 3c2a0ed8 <c6_shift_0>
4200c50e:	3fc984b7          	lui	s1,0x3fc98
4200c512:	8552                	mv	a0,s4
4200c514:	0dc48793          	addi	a5,s1,220 # 3fc980dc <activations>
4200c518:	9802                	jalr	a6
4200c51a:	4501                	li	a0,0
4200c51c:	95bff0ef          	jal	4200be76 <profile_end>
4200c520:	180a9563          	bnez	s5,4200c6aa <resnet_execute+0x292>
4200c524:	4505                	li	a0,1
4200c526:	907ff0ef          	jal	4200be2c <profile_begin>
4200c52a:	4605                	li	a2,1
4200c52c:	85a2                	mv	a1,s0
4200c52e:	8532                	mv	a0,a2
4200c530:	8c3ff0ef          	jal	4200bdf2 <choose_backend>
4200c534:	4719                	li	a4,6
4200c536:	12e50663          	beq	a0,a4,4200c662 <resnet_execute+0x24a>
4200c53a:	3c1ca737          	lui	a4,0x3c1ca
4200c53e:	00651693          	slli	a3,a0,0x6
4200c542:	64070713          	addi	a4,a4,1600 # 3c1ca640 <r_kernels>
4200c546:	9736                	add	a4,a4,a3
4200c548:	469d                	li	a3,7
4200c54a:	00472803          	lw	a6,4(a4)
4200c54e:	06d50763          	beq	a0,a3,4200c5bc <resnet_execute+0x1a4>
4200c552:	4715                	li	a4,5
4200c554:	00e51463          	bne	a0,a4,4200c55c <resnet_execute+0x144>
4200c558:	0020106f          	j	4200d55a <resnet_execute+0x1142>
4200c55c:	4711                	li	a4,4
4200c55e:	00e51463          	bne	a0,a4,4200c566 <resnet_execute+0x14e>
4200c562:	3f20106f          	j	4200d954 <resnet_execute+0x153c>
4200c566:	470d                	li	a4,3
4200c568:	00e51463          	bne	a0,a4,4200c570 <resnet_execute+0x158>
4200c56c:	40c0106f          	j	4200d978 <resnet_execute+0x1560>
4200c570:	3c2656b7          	lui	a3,0x3c265
4200c574:	3c265637          	lui	a2,0x3c265
4200c578:	3c2655b7          	lui	a1,0x3c265
4200c57c:	3c265737          	lui	a4,0x3c265
4200c580:	ca068693          	addi	a3,a3,-864 # 3c264ca0 <r_mult_1>
4200c584:	ce060613          	addi	a2,a2,-800 # 3c264ce0 <r_bias_1>
4200c588:	d2058593          	addi	a1,a1,-736 # 3c264d20 <r_weights_1>
4200c58c:	c6070713          	addi	a4,a4,-928 # 3c264c60 <r_shift_1>
4200c590:	a0b1                	j	4200c5dc <resnet_execute+0x1c4>
4200c592:	3c2b56b7          	lui	a3,0x3c2b5
4200c596:	3c2b5637          	lui	a2,0x3c2b5
4200c59a:	3c2b55b7          	lui	a1,0x3c2b5
4200c59e:	3c2b5737          	lui	a4,0x3c2b5
4200c5a2:	3fc8fb37          	lui	s6,0x3fc8f
4200c5a6:	d8068693          	addi	a3,a3,-640 # 3c2b4d80 <c7_mult_0>
4200c5aa:	dc060613          	addi	a2,a2,-576 # 3c2b4dc0 <c7_bias_0>
4200c5ae:	e0058593          	addi	a1,a1,-512 # 3c2b4e00 <c7_weights_0>
4200c5b2:	d4070713          	addi	a4,a4,-704 # 3c2b4d40 <c7_shift_0>
4200c5b6:	accb0993          	addi	s3,s6,-1332 # 3fc8eacc <padded>
4200c5ba:	bf91                	j	4200c50e <resnet_execute+0xf6>
4200c5bc:	3c2b46b7          	lui	a3,0x3c2b4
4200c5c0:	3c2b4637          	lui	a2,0x3c2b4
4200c5c4:	3c2b45b7          	lui	a1,0x3c2b4
4200c5c8:	3c2b4737          	lui	a4,0x3c2b4
4200c5cc:	3c068693          	addi	a3,a3,960 # 3c2b43c0 <c7_mult_1>
4200c5d0:	40060613          	addi	a2,a2,1024 # 3c2b4400 <c7_bias_1>
4200c5d4:	44058593          	addi	a1,a1,1088 # 3c2b4440 <c7_weights_1>
4200c5d8:	38070713          	addi	a4,a4,896 # 3c2b4380 <c7_shift_1>
4200c5dc:	3fc9cc37          	lui	s8,0x3fc9c
4200c5e0:	0dc48513          	addi	a0,s1,220
4200c5e4:	0dcc0793          	addi	a5,s8,220 # 3fc9c0dc <activations+0x4000>
4200c5e8:	9802                	jalr	a6
4200c5ea:	4505                	li	a0,1
4200c5ec:	88bff0ef          	jal	4200be76 <profile_end>
4200c5f0:	6c0a9363          	bnez	s5,4200ccb6 <resnet_execute+0x89e>
4200c5f4:	4509                	li	a0,2
4200c5f6:	837ff0ef          	jal	4200be2c <profile_begin>
4200c5fa:	85a2                	mv	a1,s0
4200c5fc:	4609                	li	a2,2
4200c5fe:	4505                	li	a0,1
4200c600:	ff2ff0ef          	jal	4200bdf2 <choose_backend>
4200c604:	4719                	li	a4,6
4200c606:	66e50463          	beq	a0,a4,4200cc6e <resnet_execute+0x856>
4200c60a:	3c1ca737          	lui	a4,0x3c1ca
4200c60e:	00651693          	slli	a3,a0,0x6
4200c612:	64070713          	addi	a4,a4,1600 # 3c1ca640 <r_kernels>
4200c616:	9736                	add	a4,a4,a3
4200c618:	469d                	li	a3,7
4200c61a:	00872803          	lw	a6,8(a4)
4200c61e:	0ad50763          	beq	a0,a3,4200c6cc <resnet_execute+0x2b4>
4200c622:	4715                	li	a4,5
4200c624:	00e51463          	bne	a0,a4,4200c62c <resnet_execute+0x214>
4200c628:	2600106f          	j	4200d888 <resnet_execute+0x1470>
4200c62c:	4711                	li	a4,4
4200c62e:	00e51463          	bne	a0,a4,4200c636 <resnet_execute+0x21e>
4200c632:	2320106f          	j	4200d864 <resnet_execute+0x144c>
4200c636:	470d                	li	a4,3
4200c638:	00e51463          	bne	a0,a4,4200c640 <resnet_execute+0x228>
4200c63c:	29c0106f          	j	4200d8d8 <resnet_execute+0x14c0>
4200c640:	3c2646b7          	lui	a3,0x3c264
4200c644:	3c264637          	lui	a2,0x3c264
4200c648:	3c2645b7          	lui	a1,0x3c264
4200c64c:	3c264737          	lui	a4,0x3c264
4200c650:	2e068693          	addi	a3,a3,736 # 3c2642e0 <r_mult_2>
4200c654:	32060613          	addi	a2,a2,800 # 3c264320 <r_bias_2>
4200c658:	36058593          	addi	a1,a1,864 # 3c264360 <r_weights_2>
4200c65c:	2a070713          	addi	a4,a4,672 # 3c2642a0 <r_shift_2>
4200c660:	a071                	j	4200c6ec <resnet_execute+0x2d4>
4200c662:	6525                	lui	a0,0x9
4200c664:	d1050513          	addi	a0,a0,-752 # 8d10 <RvExcFrameSize+0x8c7c>
4200c668:	3c2a05b7          	lui	a1,0x3c2a0
4200c66c:	6605                	lui	a2,0x1
4200c66e:	5d858593          	addi	a1,a1,1496 # 3c2a05d8 <c6_weights_1>
4200c672:	90060613          	addi	a2,a2,-1792 # 900 <RvExcFrameSize+0x86c>
4200c676:	954e                	add	a0,a0,s3
4200c678:	fdff4097          	auipc	ra,0xfdff4
4200c67c:	ce0080e7          	jalr	-800(ra) # 40000358 <memcpy>
4200c680:	3c2a06b7          	lui	a3,0x3c2a0
4200c684:	3c2a0637          	lui	a2,0x3c2a0
4200c688:	3fc975b7          	lui	a1,0x3fc97
4200c68c:	42031837          	lui	a6,0x42031
4200c690:	3c2a0737          	lui	a4,0x3c2a0
4200c694:	55868693          	addi	a3,a3,1368 # 3c2a0558 <c6_mult_1>
4200c698:	59860613          	addi	a2,a2,1432 # 3c2a0598 <c6_bias_1>
4200c69c:	7dc58593          	addi	a1,a1,2012 # 3fc977dc <padded+0x8d10>
4200c6a0:	80480813          	addi	a6,a6,-2044 # 42030804 <r_tile16_b2p4_sram_layer_1>
4200c6a4:	51870713          	addi	a4,a4,1304 # 3c2a0518 <c6_shift_1>
4200c6a8:	bf15                	j	4200c5dc <resnet_execute+0x1c4>
4200c6aa:	3c1de7b7          	lui	a5,0x3c1de
4200c6ae:	6a878793          	addi	a5,a5,1704 # 3c1de6a8 <r_oracles>
4200c6b2:	97de                	add	a5,a5,s7
4200c6b4:	439c                	lw	a5,0(a5)
4200c6b6:	6811                	lui	a6,0x4
4200c6b8:	0dc48713          	addi	a4,s1,220
4200c6bc:	439c                	lw	a5,0(a5)
4200c6be:	4681                	li	a3,0
4200c6c0:	864a                	mv	a2,s2
4200c6c2:	85a2                	mv	a1,s0
4200c6c4:	4505                	li	a0,1
4200c6c6:	84bff0ef          	jal	4200bf10 <record_check>
4200c6ca:	bda9                	j	4200c524 <resnet_execute+0x10c>
4200c6cc:	3c2b46b7          	lui	a3,0x3c2b4
4200c6d0:	3c2b4637          	lui	a2,0x3c2b4
4200c6d4:	3c2b45b7          	lui	a1,0x3c2b4
4200c6d8:	3c2b4737          	lui	a4,0x3c2b4
4200c6dc:	a0068693          	addi	a3,a3,-1536 # 3c2b3a00 <c7_mult_2>
4200c6e0:	a4060613          	addi	a2,a2,-1472 # 3c2b3a40 <c7_bias_2>
4200c6e4:	a8058593          	addi	a1,a1,-1408 # 3c2b3a80 <c7_weights_2>
4200c6e8:	9c070713          	addi	a4,a4,-1600 # 3c2b39c0 <c7_shift_2>
4200c6ec:	3fca0a37          	lui	s4,0x3fca0
4200c6f0:	0dcc0513          	addi	a0,s8,220
4200c6f4:	0dca0793          	addi	a5,s4,220 # 3fca00dc <activations+0x8000>
4200c6f8:	9802                	jalr	a6
4200c6fa:	4509                	li	a0,2
4200c6fc:	f7aff0ef          	jal	4200be76 <profile_end>
4200c700:	260a9263          	bnez	s5,4200c964 <resnet_execute+0x54c>
4200c704:	450d                	li	a0,3
4200c706:	f26ff0ef          	jal	4200be2c <profile_begin>
4200c70a:	85a2                	mv	a1,s0
4200c70c:	460d                	li	a2,3
4200c70e:	4505                	li	a0,1
4200c710:	ee2ff0ef          	jal	4200bdf2 <choose_backend>
4200c714:	417277b7          	lui	a5,0x41727
4200c718:	f8000713          	li	a4,-128
4200c71c:	6891                	lui	a7,0x4
4200c71e:	07f00813          	li	a6,127
4200c722:	553d                	li	a0,-17
4200c724:	b1e78793          	addi	a5,a5,-1250 # 41726b1e <_coredump_iram_end+0x139a31e>
4200c728:	4651                	li	a2,20
4200c72a:	0dcc0d13          	addi	s10,s8,220
4200c72e:	60c996b7          	lui	a3,0x60c99
4200c732:	ce46                	sw	a7,28(sp)
4200c734:	cc42                	sw	a6,24(sp)
4200c736:	c82a                	sw	a0,16(sp)
4200c738:	c63e                	sw	a5,12(sp)
4200c73a:	c032                	sw	a2,0(sp)
4200c73c:	ca3a                	sw	a4,20(sp)
4200c73e:	c43a                	sw	a4,8(sp)
4200c740:	0dca0593          	addi	a1,s4,220
4200c744:	c26a                	sw	s10,4(sp)
4200c746:	0dc48513          	addi	a0,s1,220
4200c74a:	ca368693          	addi	a3,a3,-861 # 60c98ca3 <SYSTEM+0xbd8ca3>
4200c74e:	4881                	li	a7,0
4200c750:	40000837          	lui	a6,0x40000
4200c754:	57f1                	li	a5,-4
4200c756:	5779                	li	a4,-2
4200c758:	08000613          	li	a2,128
4200c75c:	72d020ef          	jal	4200f688 <muriscv_nn_elementwise_add_s8>
4200c760:	450d                	li	a0,3
4200c762:	f14ff0ef          	jal	4200be76 <profile_end>
4200c766:	4511                	li	a0,4
4200c768:	ec4ff0ef          	jal	4200be2c <profile_begin>
4200c76c:	4611                	li	a2,4
4200c76e:	85a2                	mv	a1,s0
4200c770:	4505                	li	a0,1
4200c772:	e80ff0ef          	jal	4200bdf2 <choose_backend>
4200c776:	6711                	lui	a4,0x4
4200c778:	3fc8f6b7          	lui	a3,0x3fc8f
4200c77c:	40070713          	addi	a4,a4,1024 # 4400 <RvExcFrameSize+0x436c>
4200c780:	87aa                	mv	a5,a0
4200c782:	ccc68a13          	addi	s4,a3,-820 # 3fc8eccc <padded+0x200>
4200c786:	d46a                	sw	s10,40(sp)
4200c788:	21098293          	addi	t0,s3,528
4200c78c:	00e98cb3          	add	s9,s3,a4
4200c790:	856a                	mv	a0,s10
4200c792:	ccc68693          	addi	a3,a3,-820
4200c796:	f8000613          	li	a2,-128
4200c79a:	e0068713          	addi	a4,a3,-512
4200c79e:	85aa                	mv	a1,a0
4200c7a0:	0005cf83          	lbu	t6,0(a1)
4200c7a4:	0015cf03          	lbu	t5,1(a1)
4200c7a8:	0025ce83          	lbu	t4,2(a1)
4200c7ac:	0035ce03          	lbu	t3,3(a1)
4200c7b0:	0045c303          	lbu	t1,4(a1)
4200c7b4:	0055c883          	lbu	a7,5(a1)
4200c7b8:	0065c803          	lbu	a6,6(a1)
4200c7bc:	0075cd83          	lbu	s11,7(a1)
4200c7c0:	0085c383          	lbu	t2,8(a1)
4200c7c4:	01f70023          	sb	t6,0(a4)
4200c7c8:	01e700a3          	sb	t5,1(a4)
4200c7cc:	0095cf83          	lbu	t6,9(a1)
4200c7d0:	00a5cf03          	lbu	t5,10(a1)
4200c7d4:	01d70123          	sb	t4,2(a4)
4200c7d8:	01c701a3          	sb	t3,3(a4)
4200c7dc:	00b5ce83          	lbu	t4,11(a1)
4200c7e0:	00c5ce03          	lbu	t3,12(a1)
4200c7e4:	00670223          	sb	t1,4(a4)
4200c7e8:	011702a3          	sb	a7,5(a4)
4200c7ec:	00d5c303          	lbu	t1,13(a1)
4200c7f0:	00e5c883          	lbu	a7,14(a1)
4200c7f4:	01070323          	sb	a6,6(a4)
4200c7f8:	00f5c803          	lbu	a6,15(a1)
4200c7fc:	01b703a3          	sb	s11,7(a4)
4200c800:	00770423          	sb	t2,8(a4)
4200c804:	01f704a3          	sb	t6,9(a4)
4200c808:	01e70523          	sb	t5,10(a4)
4200c80c:	01d705a3          	sb	t4,11(a4)
4200c810:	01c70623          	sb	t3,12(a4)
4200c814:	006706a3          	sb	t1,13(a4)
4200c818:	01170723          	sb	a7,14(a4)
4200c81c:	010707a3          	sb	a6,15(a4)
4200c820:	0741                	addi	a4,a4,16
4200c822:	05c1                	addi	a1,a1,16
4200c824:	04570463          	beq	a4,t0,4200c86c <resnet_execute+0x454>
4200c828:	f6d71ce3          	bne	a4,a3,4200c7a0 <resnet_execute+0x388>
4200c82c:	00c68023          	sb	a2,0(a3)
4200c830:	00c680a3          	sb	a2,1(a3)
4200c834:	00c68123          	sb	a2,2(a3)
4200c838:	00c681a3          	sb	a2,3(a3)
4200c83c:	00c68223          	sb	a2,4(a3)
4200c840:	00c682a3          	sb	a2,5(a3)
4200c844:	00c68323          	sb	a2,6(a3)
4200c848:	00c683a3          	sb	a2,7(a3)
4200c84c:	00c68423          	sb	a2,8(a3)
4200c850:	00c684a3          	sb	a2,9(a3)
4200c854:	00c68523          	sb	a2,10(a3)
4200c858:	00c685a3          	sb	a2,11(a3)
4200c85c:	00c68623          	sb	a2,12(a3)
4200c860:	00c686a3          	sb	a2,13(a3)
4200c864:	00c68723          	sb	a2,14(a3)
4200c868:	00c687a3          	sb	a2,15(a3)
4200c86c:	21068693          	addi	a3,a3,528
4200c870:	20050513          	addi	a0,a0,512
4200c874:	21028293          	addi	t0,t0,528
4200c878:	f2dc91e3          	bne	s9,a3,4200c79a <resnet_execute+0x382>
4200c87c:	3fc93737          	lui	a4,0x3fc93
4200c880:	ccc70713          	addi	a4,a4,-820 # 3fc92ccc <padded+0x4200>
4200c884:	21070613          	addi	a2,a4,528
4200c888:	f8000693          	li	a3,-128
4200c88c:	00d70023          	sb	a3,0(a4)
4200c890:	00d700a3          	sb	a3,1(a4)
4200c894:	00d70123          	sb	a3,2(a4)
4200c898:	00d701a3          	sb	a3,3(a4)
4200c89c:	00d70223          	sb	a3,4(a4)
4200c8a0:	00d702a3          	sb	a3,5(a4)
4200c8a4:	00d70323          	sb	a3,6(a4)
4200c8a8:	00d703a3          	sb	a3,7(a4)
4200c8ac:	00d70423          	sb	a3,8(a4)
4200c8b0:	00d704a3          	sb	a3,9(a4)
4200c8b4:	00d70523          	sb	a3,10(a4)
4200c8b8:	00d705a3          	sb	a3,11(a4)
4200c8bc:	00d70623          	sb	a3,12(a4)
4200c8c0:	00d706a3          	sb	a3,13(a4)
4200c8c4:	00d70723          	sb	a3,14(a4)
4200c8c8:	00d707a3          	sb	a3,15(a4)
4200c8cc:	0741                	addi	a4,a4,16
4200c8ce:	fae61fe3          	bne	a2,a4,4200c88c <resnet_execute+0x474>
4200c8d2:	4719                	li	a4,6
4200c8d4:	1ce78263          	beq	a5,a4,4200ca98 <resnet_execute+0x680>
4200c8d8:	3c1ca737          	lui	a4,0x3c1ca
4200c8dc:	00679693          	slli	a3,a5,0x6
4200c8e0:	64070713          	addi	a4,a4,1600 # 3c1ca640 <r_kernels>
4200c8e4:	9736                	add	a4,a4,a3
4200c8e6:	469d                	li	a3,7
4200c8e8:	01072803          	lw	a6,16(a4)
4200c8ec:	10d78b63          	beq	a5,a3,4200ca02 <resnet_execute+0x5ea>
4200c8f0:	4715                	li	a4,5
4200c8f2:	00e79463          	bne	a5,a4,4200c8fa <resnet_execute+0x4e2>
4200c8f6:	6df0006f          	j	4200d7d4 <resnet_execute+0x13bc>
4200c8fa:	4711                	li	a4,4
4200c8fc:	00e79463          	bne	a5,a4,4200c904 <resnet_execute+0x4ec>
4200c900:	71d0006f          	j	4200d81c <resnet_execute+0x1404>
4200c904:	470d                	li	a4,3
4200c906:	00e79463          	bne	a5,a4,4200c90e <resnet_execute+0x4f6>
4200c90a:	6ef0006f          	j	4200d7f8 <resnet_execute+0x13e0>
4200c90e:	3c2636b7          	lui	a3,0x3c263
4200c912:	3c263637          	lui	a2,0x3c263
4200c916:	3c2635b7          	lui	a1,0x3c263
4200c91a:	3c263737          	lui	a4,0x3c263
4200c91e:	accb0513          	addi	a0,s6,-1332
4200c922:	fa068693          	addi	a3,a3,-96 # 3c262fa0 <r_mult_4>
4200c926:	02060613          	addi	a2,a2,32 # 3c263020 <r_bias_4>
4200c92a:	0a058593          	addi	a1,a1,160 # 3c2630a0 <r_weights_4>
4200c92e:	f2070713          	addi	a4,a4,-224 # 3c262f20 <r_shift_4>
4200c932:	0dc48793          	addi	a5,s1,220
4200c936:	9802                	jalr	a6
4200c938:	4511                	li	a0,4
4200c93a:	d3cff0ef          	jal	4200be76 <profile_end>
4200c93e:	0e0a8c63          	beqz	s5,4200ca36 <resnet_execute+0x61e>
4200c942:	3c1de7b7          	lui	a5,0x3c1de
4200c946:	6a878793          	addi	a5,a5,1704 # 3c1de6a8 <r_oracles>
4200c94a:	97de                	add	a5,a5,s7
4200c94c:	439c                	lw	a5,0(a5)
4200c94e:	6809                	lui	a6,0x2
4200c950:	0dc48713          	addi	a4,s1,220
4200c954:	4b9c                	lw	a5,16(a5)
4200c956:	4691                	li	a3,4
4200c958:	864a                	mv	a2,s2
4200c95a:	85a2                	mv	a1,s0
4200c95c:	4505                	li	a0,1
4200c95e:	db2ff0ef          	jal	4200bf10 <record_check>
4200c962:	a8d1                	j	4200ca36 <resnet_execute+0x61e>
4200c964:	3c1de7b7          	lui	a5,0x3c1de
4200c968:	6a878793          	addi	a5,a5,1704 # 3c1de6a8 <r_oracles>
4200c96c:	97de                	add	a5,a5,s7
4200c96e:	0007ac83          	lw	s9,0(a5)
4200c972:	0dca0713          	addi	a4,s4,220
4200c976:	6811                	lui	a6,0x4
4200c978:	008ca783          	lw	a5,8(s9)
4200c97c:	4689                	li	a3,2
4200c97e:	864a                	mv	a2,s2
4200c980:	85a2                	mv	a1,s0
4200c982:	4505                	li	a0,1
4200c984:	d8cff0ef          	jal	4200bf10 <record_check>
4200c988:	450d                	li	a0,3
4200c98a:	ca2ff0ef          	jal	4200be2c <profile_begin>
4200c98e:	85a2                	mv	a1,s0
4200c990:	460d                	li	a2,3
4200c992:	4505                	li	a0,1
4200c994:	c5eff0ef          	jal	4200bdf2 <choose_backend>
4200c998:	417277b7          	lui	a5,0x41727
4200c99c:	6d91                	lui	s11,0x4
4200c99e:	f8000713          	li	a4,-128
4200c9a2:	07f00813          	li	a6,127
4200c9a6:	553d                	li	a0,-17
4200c9a8:	b1e78793          	addi	a5,a5,-1250 # 41726b1e <_coredump_iram_end+0x139a31e>
4200c9ac:	4651                	li	a2,20
4200c9ae:	0dcc0d13          	addi	s10,s8,220
4200c9b2:	60c996b7          	lui	a3,0x60c99
4200c9b6:	0dca0593          	addi	a1,s4,220
4200c9ba:	cc42                	sw	a6,24(sp)
4200c9bc:	c82a                	sw	a0,16(sp)
4200c9be:	c63e                	sw	a5,12(sp)
4200c9c0:	c032                	sw	a2,0(sp)
4200c9c2:	ca3a                	sw	a4,20(sp)
4200c9c4:	c43a                	sw	a4,8(sp)
4200c9c6:	ca368693          	addi	a3,a3,-861 # 60c98ca3 <SYSTEM+0xbd8ca3>
4200c9ca:	4881                	li	a7,0
4200c9cc:	40000837          	lui	a6,0x40000
4200c9d0:	57f1                	li	a5,-4
4200c9d2:	5779                	li	a4,-2
4200c9d4:	08000613          	li	a2,128
4200c9d8:	c26a                	sw	s10,4(sp)
4200c9da:	ce6e                	sw	s11,28(sp)
4200c9dc:	0dc48513          	addi	a0,s1,220
4200c9e0:	4a9020ef          	jal	4200f688 <muriscv_nn_elementwise_add_s8>
4200c9e4:	450d                	li	a0,3
4200c9e6:	c90ff0ef          	jal	4200be76 <profile_end>
4200c9ea:	00cca783          	lw	a5,12(s9)
4200c9ee:	886e                	mv	a6,s11
4200c9f0:	0dcc0713          	addi	a4,s8,220
4200c9f4:	864a                	mv	a2,s2
4200c9f6:	85a2                	mv	a1,s0
4200c9f8:	468d                	li	a3,3
4200c9fa:	4505                	li	a0,1
4200c9fc:	d14ff0ef          	jal	4200bf10 <record_check>
4200ca00:	b39d                	j	4200c766 <resnet_execute+0x34e>
4200ca02:	3c2b26b7          	lui	a3,0x3c2b2
4200ca06:	3c2b2637          	lui	a2,0x3c2b2
4200ca0a:	3c2b25b7          	lui	a1,0x3c2b2
4200ca0e:	3c2b2737          	lui	a4,0x3c2b2
4200ca12:	6c068693          	addi	a3,a3,1728 # 3c2b26c0 <c7_mult_4>
4200ca16:	74060613          	addi	a2,a2,1856 # 3c2b2740 <c7_bias_4>
4200ca1a:	7c058593          	addi	a1,a1,1984 # 3c2b27c0 <c7_weights_4>
4200ca1e:	64070713          	addi	a4,a4,1600 # 3c2b2640 <c7_shift_4>
4200ca22:	accb0513          	addi	a0,s6,-1332
4200ca26:	0dc48793          	addi	a5,s1,220
4200ca2a:	9802                	jalr	a6
4200ca2c:	4511                	li	a0,4
4200ca2e:	c48ff0ef          	jal	4200be76 <profile_end>
4200ca32:	f00a98e3          	bnez	s5,4200c942 <resnet_execute+0x52a>
4200ca36:	4515                	li	a0,5
4200ca38:	bf4ff0ef          	jal	4200be2c <profile_begin>
4200ca3c:	85a2                	mv	a1,s0
4200ca3e:	4615                	li	a2,5
4200ca40:	4505                	li	a0,1
4200ca42:	bb0ff0ef          	jal	4200bdf2 <choose_backend>
4200ca46:	4719                	li	a4,6
4200ca48:	1ce50f63          	beq	a0,a4,4200cc26 <resnet_execute+0x80e>
4200ca4c:	3c1ca737          	lui	a4,0x3c1ca
4200ca50:	00651693          	slli	a3,a0,0x6
4200ca54:	64070713          	addi	a4,a4,1600 # 3c1ca640 <r_kernels>
4200ca58:	9736                	add	a4,a4,a3
4200ca5a:	469d                	li	a3,7
4200ca5c:	01472803          	lw	a6,20(a4)
4200ca60:	08d50063          	beq	a0,a3,4200cae0 <resnet_execute+0x6c8>
4200ca64:	4715                	li	a4,5
4200ca66:	54e505e3          	beq	a0,a4,4200d7b0 <resnet_execute+0x1398>
4200ca6a:	4711                	li	a4,4
4200ca6c:	52e500e3          	beq	a0,a4,4200d78c <resnet_execute+0x1374>
4200ca70:	470d                	li	a4,3
4200ca72:	5ce507e3          	beq	a0,a4,4200d840 <resnet_execute+0x1428>
4200ca76:	3c2616b7          	lui	a3,0x3c261
4200ca7a:	3c261637          	lui	a2,0x3c261
4200ca7e:	3c2615b7          	lui	a1,0x3c261
4200ca82:	3c261737          	lui	a4,0x3c261
4200ca86:	a2068693          	addi	a3,a3,-1504 # 3c260a20 <r_mult_5>
4200ca8a:	aa060613          	addi	a2,a2,-1376 # 3c260aa0 <r_bias_5>
4200ca8e:	b2058593          	addi	a1,a1,-1248 # 3c260b20 <r_weights_5>
4200ca92:	9a070713          	addi	a4,a4,-1632 # 3c2609a0 <r_shift_5>
4200ca96:	a0ad                	j	4200cb00 <resnet_execute+0x6e8>
4200ca98:	6521                	lui	a0,0x8
4200ca9a:	41050513          	addi	a0,a0,1040 # 8410 <RvExcFrameSize+0x837c>
4200ca9e:	3c29f5b7          	lui	a1,0x3c29f
4200caa2:	6605                	lui	a2,0x1
4200caa4:	95858593          	addi	a1,a1,-1704 # 3c29e958 <c6_weights_4>
4200caa8:	20060613          	addi	a2,a2,512 # 1200 <RvExcFrameSize+0x116c>
4200caac:	954e                	add	a0,a0,s3
4200caae:	fdff4097          	auipc	ra,0xfdff4
4200cab2:	8aa080e7          	jalr	-1878(ra) # 40000358 <memcpy>
4200cab6:	3c29f6b7          	lui	a3,0x3c29f
4200caba:	3c29f637          	lui	a2,0x3c29f
4200cabe:	3fc975b7          	lui	a1,0x3fc97
4200cac2:	42032837          	lui	a6,0x42032
4200cac6:	3c29e737          	lui	a4,0x3c29e
4200caca:	85868693          	addi	a3,a3,-1960 # 3c29e858 <c6_mult_4>
4200cace:	8d860613          	addi	a2,a2,-1832 # 3c29e8d8 <c6_bias_4>
4200cad2:	edc58593          	addi	a1,a1,-292 # 3fc96edc <padded+0x8410>
4200cad6:	9b280813          	addi	a6,a6,-1614 # 420319b2 <r_tile16_b2p4_sram_layer_4>
4200cada:	7d870713          	addi	a4,a4,2008 # 3c29e7d8 <c6_shift_4>
4200cade:	b791                	j	4200ca22 <resnet_execute+0x60a>
4200cae0:	3c2b06b7          	lui	a3,0x3c2b0
4200cae4:	3c2b0637          	lui	a2,0x3c2b0
4200cae8:	3c2b05b7          	lui	a1,0x3c2b0
4200caec:	3c2b0737          	lui	a4,0x3c2b0
4200caf0:	14068693          	addi	a3,a3,320 # 3c2b0140 <c7_mult_5>
4200caf4:	1c060613          	addi	a2,a2,448 # 3c2b01c0 <c7_bias_5>
4200caf8:	24058593          	addi	a1,a1,576 # 3c2b0240 <c7_weights_5>
4200cafc:	0c070713          	addi	a4,a4,192 # 3c2b00c0 <c7_shift_5>
4200cb00:	3fc9acb7          	lui	s9,0x3fc9a
4200cb04:	0dc48513          	addi	a0,s1,220
4200cb08:	0dcc8793          	addi	a5,s9,220 # 3fc9a0dc <activations+0x2000>
4200cb0c:	9802                	jalr	a6
4200cb0e:	4515                	li	a0,5
4200cb10:	b66ff0ef          	jal	4200be76 <profile_end>
4200cb14:	640a9163          	bnez	s5,4200d156 <resnet_execute+0xd3e>
4200cb18:	4519                	li	a0,6
4200cb1a:	b12ff0ef          	jal	4200be2c <profile_begin>
4200cb1e:	85a2                	mv	a1,s0
4200cb20:	4619                	li	a2,6
4200cb22:	4505                	li	a0,1
4200cb24:	aceff0ef          	jal	4200bdf2 <choose_backend>
4200cb28:	4719                	li	a4,6
4200cb2a:	5ee50363          	beq	a0,a4,4200d110 <resnet_execute+0xcf8>
4200cb2e:	3c1ca737          	lui	a4,0x3c1ca
4200cb32:	00651693          	slli	a3,a0,0x6
4200cb36:	64070713          	addi	a4,a4,1600 # 3c1ca640 <r_kernels>
4200cb3a:	9736                	add	a4,a4,a3
4200cb3c:	469d                	li	a3,7
4200cb3e:	01872803          	lw	a6,24(a4)
4200cb42:	18d50b63          	beq	a0,a3,4200ccd8 <resnet_execute+0x8c0>
4200cb46:	4715                	li	a4,5
4200cb48:	3ee50ee3          	beq	a0,a4,4200d744 <resnet_execute+0x132c>
4200cb4c:	4711                	li	a4,4
4200cb4e:	32e501e3          	beq	a0,a4,4200d670 <resnet_execute+0x1258>
4200cb52:	470d                	li	a4,3
4200cb54:	40e50ae3          	beq	a0,a4,4200d768 <resnet_execute+0x1350>
4200cb58:	3c2606b7          	lui	a3,0x3c260
4200cb5c:	3c260637          	lui	a2,0x3c260
4200cb60:	3c2605b7          	lui	a1,0x3c260
4200cb64:	3c260737          	lui	a4,0x3c260
4200cb68:	0dcc0513          	addi	a0,s8,220
4200cb6c:	6a068693          	addi	a3,a3,1696 # 3c2606a0 <r_mult_6>
4200cb70:	72060613          	addi	a2,a2,1824 # 3c260720 <r_bias_6>
4200cb74:	7a058593          	addi	a1,a1,1952 # 3c2607a0 <r_weights_6>
4200cb78:	62070713          	addi	a4,a4,1568 # 3c260620 <r_shift_6>
4200cb7c:	0dc48793          	addi	a5,s1,220
4200cb80:	9802                	jalr	a6
4200cb82:	4519                	li	a0,6
4200cb84:	af2ff0ef          	jal	4200be76 <profile_end>
4200cb88:	180a8263          	beqz	s5,4200cd0c <resnet_execute+0x8f4>
4200cb8c:	3c1de7b7          	lui	a5,0x3c1de
4200cb90:	6a878793          	addi	a5,a5,1704 # 3c1de6a8 <r_oracles>
4200cb94:	97de                	add	a5,a5,s7
4200cb96:	0007ad83          	lw	s11,0(a5)
4200cb9a:	0dc48713          	addi	a4,s1,220
4200cb9e:	6809                	lui	a6,0x2
4200cba0:	018da783          	lw	a5,24(s11) # 4018 <RvExcFrameSize+0x3f84>
4200cba4:	4699                	li	a3,6
4200cba6:	864a                	mv	a2,s2
4200cba8:	85a2                	mv	a1,s0
4200cbaa:	4505                	li	a0,1
4200cbac:	b64ff0ef          	jal	4200bf10 <record_check>
4200cbb0:	451d                	li	a0,7
4200cbb2:	a7aff0ef          	jal	4200be2c <profile_begin>
4200cbb6:	85a2                	mv	a1,s0
4200cbb8:	461d                	li	a2,7
4200cbba:	4505                	li	a0,1
4200cbbc:	a36ff0ef          	jal	4200bdf2 <choose_backend>
4200cbc0:	43fec7b7          	lui	a5,0x43fec
4200cbc4:	6309                	lui	t1,0x2
4200cbc6:	f8000713          	li	a4,-128
4200cbca:	07f00513          	li	a0,127
4200cbce:	55bd                	li	a1,-17
4200cbd0:	03a78793          	addi	a5,a5,58 # 43fec03a <_etext+0x1f945b6>
4200cbd4:	4651                	li	a2,20
4200cbd6:	654cc6b7          	lui	a3,0x654cc
4200cbda:	ce1a                	sw	t1,28(sp)
4200cbdc:	cc2a                	sw	a0,24(sp)
4200cbde:	c82e                	sw	a1,16(sp)
4200cbe0:	c63e                	sw	a5,12(sp)
4200cbe2:	0dcc8593          	addi	a1,s9,220
4200cbe6:	c032                	sw	a2,0(sp)
4200cbe8:	ca3a                	sw	a4,20(sp)
4200cbea:	c43a                	sw	a4,8(sp)
4200cbec:	4ff68693          	addi	a3,a3,1279 # 654cc4ff <SYSTEM+0x540c4ff>
4200cbf0:	4881                	li	a7,0
4200cbf2:	40000837          	lui	a6,0x40000
4200cbf6:	57f1                	li	a5,-4
4200cbf8:	5779                	li	a4,-2
4200cbfa:	4645                	li	a2,17
4200cbfc:	c26a                	sw	s10,4(sp)
4200cbfe:	0dc48513          	addi	a0,s1,220
4200cc02:	287020ef          	jal	4200f688 <muriscv_nn_elementwise_add_s8>
4200cc06:	451d                	li	a0,7
4200cc08:	a6eff0ef          	jal	4200be76 <profile_end>
4200cc0c:	01cda783          	lw	a5,28(s11)
4200cc10:	6309                	lui	t1,0x2
4200cc12:	881a                	mv	a6,t1
4200cc14:	0dcc0713          	addi	a4,s8,220
4200cc18:	864a                	mv	a2,s2
4200cc1a:	85a2                	mv	a1,s0
4200cc1c:	469d                	li	a3,7
4200cc1e:	4505                	li	a0,1
4200cc20:	af0ff0ef          	jal	4200bf10 <record_check>
4200cc24:	a291                	j	4200cd68 <resnet_execute+0x950>
4200cc26:	651d                	lui	a0,0x7
4200cc28:	21050513          	addi	a0,a0,528 # 7210 <RvExcFrameSize+0x717c>
4200cc2c:	3c29c5b7          	lui	a1,0x3c29c
4200cc30:	6609                	lui	a2,0x2
4200cc32:	3d858593          	addi	a1,a1,984 # 3c29c3d8 <c6_weights_5>
4200cc36:	40060613          	addi	a2,a2,1024 # 2400 <RvExcFrameSize+0x236c>
4200cc3a:	954e                	add	a0,a0,s3
4200cc3c:	fdff3097          	auipc	ra,0xfdff3
4200cc40:	71c080e7          	jalr	1820(ra) # 40000358 <memcpy>
4200cc44:	3c29c6b7          	lui	a3,0x3c29c
4200cc48:	3c29c637          	lui	a2,0x3c29c
4200cc4c:	3fc965b7          	lui	a1,0x3fc96
4200cc50:	42032837          	lui	a6,0x42032
4200cc54:	3c29c737          	lui	a4,0x3c29c
4200cc58:	2d868693          	addi	a3,a3,728 # 3c29c2d8 <c6_mult_5>
4200cc5c:	35860613          	addi	a2,a2,856 # 3c29c358 <c6_bias_5>
4200cc60:	cdc58593          	addi	a1,a1,-804 # 3fc95cdc <padded+0x7210>
4200cc64:	45080813          	addi	a6,a6,1104 # 42032450 <r_tile16_b2p4_sram_layer_5>
4200cc68:	25870713          	addi	a4,a4,600 # 3c29c258 <c6_shift_5>
4200cc6c:	bd51                	j	4200cb00 <resnet_execute+0x6e8>
4200cc6e:	6525                	lui	a0,0x9
4200cc70:	d1050513          	addi	a0,a0,-752 # 8d10 <RvExcFrameSize+0x8c7c>
4200cc74:	3c2a05b7          	lui	a1,0x3c2a0
4200cc78:	6605                	lui	a2,0x1
4200cc7a:	c1858593          	addi	a1,a1,-1000 # 3c29fc18 <c6_weights_2>
4200cc7e:	90060613          	addi	a2,a2,-1792 # 900 <RvExcFrameSize+0x86c>
4200cc82:	954e                	add	a0,a0,s3
4200cc84:	fdff3097          	auipc	ra,0xfdff3
4200cc88:	6d4080e7          	jalr	1748(ra) # 40000358 <memcpy>
4200cc8c:	3c2a06b7          	lui	a3,0x3c2a0
4200cc90:	3c2a0637          	lui	a2,0x3c2a0
4200cc94:	3fc975b7          	lui	a1,0x3fc97
4200cc98:	42031837          	lui	a6,0x42031
4200cc9c:	3c2a0737          	lui	a4,0x3c2a0
4200cca0:	b9868693          	addi	a3,a3,-1128 # 3c29fb98 <c6_mult_2>
4200cca4:	bd860613          	addi	a2,a2,-1064 # 3c29fbd8 <c6_bias_2>
4200cca8:	7dc58593          	addi	a1,a1,2012 # 3fc977dc <padded+0x8d10>
4200ccac:	2a280813          	addi	a6,a6,674 # 420312a2 <r_tile16_b2p4_sram_layer_2>
4200ccb0:	b5870713          	addi	a4,a4,-1192 # 3c29fb58 <c6_shift_2>
4200ccb4:	bc25                	j	4200c6ec <resnet_execute+0x2d4>
4200ccb6:	3c1de7b7          	lui	a5,0x3c1de
4200ccba:	6a878793          	addi	a5,a5,1704 # 3c1de6a8 <r_oracles>
4200ccbe:	97de                	add	a5,a5,s7
4200ccc0:	439c                	lw	a5,0(a5)
4200ccc2:	4685                	li	a3,1
4200ccc4:	8536                	mv	a0,a3
4200ccc6:	43dc                	lw	a5,4(a5)
4200ccc8:	6811                	lui	a6,0x4
4200ccca:	0dcc0713          	addi	a4,s8,220
4200ccce:	864a                	mv	a2,s2
4200ccd0:	85a2                	mv	a1,s0
4200ccd2:	a3eff0ef          	jal	4200bf10 <record_check>
4200ccd6:	ba39                	j	4200c5f4 <resnet_execute+0x1dc>
4200ccd8:	3c2b06b7          	lui	a3,0x3c2b0
4200ccdc:	3c2b0637          	lui	a2,0x3c2b0
4200cce0:	3c2b05b7          	lui	a1,0x3c2b0
4200cce4:	3c2b0737          	lui	a4,0x3c2b0
4200cce8:	dc068693          	addi	a3,a3,-576 # 3c2afdc0 <c7_mult_6>
4200ccec:	e4060613          	addi	a2,a2,-448 # 3c2afe40 <c7_bias_6>
4200ccf0:	ec058593          	addi	a1,a1,-320 # 3c2afec0 <c7_weights_6>
4200ccf4:	d4070713          	addi	a4,a4,-704 # 3c2afd40 <c7_shift_6>
4200ccf8:	0dcc0513          	addi	a0,s8,220
4200ccfc:	0dc48793          	addi	a5,s1,220
4200cd00:	9802                	jalr	a6
4200cd02:	4519                	li	a0,6
4200cd04:	972ff0ef          	jal	4200be76 <profile_end>
4200cd08:	e80a92e3          	bnez	s5,4200cb8c <resnet_execute+0x774>
4200cd0c:	451d                	li	a0,7
4200cd0e:	91eff0ef          	jal	4200be2c <profile_begin>
4200cd12:	85a2                	mv	a1,s0
4200cd14:	461d                	li	a2,7
4200cd16:	4505                	li	a0,1
4200cd18:	8daff0ef          	jal	4200bdf2 <choose_backend>
4200cd1c:	43fec7b7          	lui	a5,0x43fec
4200cd20:	f8000713          	li	a4,-128
4200cd24:	6809                	lui	a6,0x2
4200cd26:	07f00513          	li	a0,127
4200cd2a:	55bd                	li	a1,-17
4200cd2c:	03a78793          	addi	a5,a5,58 # 43fec03a <_etext+0x1f945b6>
4200cd30:	4651                	li	a2,20
4200cd32:	654cc6b7          	lui	a3,0x654cc
4200cd36:	ce42                	sw	a6,28(sp)
4200cd38:	cc2a                	sw	a0,24(sp)
4200cd3a:	c82e                	sw	a1,16(sp)
4200cd3c:	c63e                	sw	a5,12(sp)
4200cd3e:	c032                	sw	a2,0(sp)
4200cd40:	ca3a                	sw	a4,20(sp)
4200cd42:	c43a                	sw	a4,8(sp)
4200cd44:	c26a                	sw	s10,4(sp)
4200cd46:	0dcc8593          	addi	a1,s9,220
4200cd4a:	0dc48513          	addi	a0,s1,220
4200cd4e:	4ff68693          	addi	a3,a3,1279 # 654cc4ff <SYSTEM+0x540c4ff>
4200cd52:	4881                	li	a7,0
4200cd54:	40000837          	lui	a6,0x40000
4200cd58:	57f1                	li	a5,-4
4200cd5a:	5779                	li	a4,-2
4200cd5c:	4645                	li	a2,17
4200cd5e:	12b020ef          	jal	4200f688 <muriscv_nn_elementwise_add_s8>
4200cd62:	451d                	li	a0,7
4200cd64:	912ff0ef          	jal	4200be76 <profile_end>
4200cd68:	4521                	li	a0,8
4200cd6a:	8c2ff0ef          	jal	4200be2c <profile_begin>
4200cd6e:	85a2                	mv	a1,s0
4200cd70:	4621                	li	a2,8
4200cd72:	4505                	li	a0,1
4200cd74:	87eff0ef          	jal	4200bdf2 <choose_backend>
4200cd78:	4d01                	li	s10,0
4200cd7a:	d622                	sw	s0,44(sp)
4200cd7c:	4741                	li	a4,16
4200cd7e:	5422                	lw	s0,40(sp)
4200cd80:	8daa                	mv	s11,a0
4200cd82:	e00a0793          	addi	a5,s4,-512
4200cd86:	04ed0963          	beq	s10,a4,4200cdd8 <resnet_execute+0x9c0>
4200cd8a:	85a2                	mv	a1,s0
4200cd8c:	853e                	mv	a0,a5
4200cd8e:	02000613          	li	a2,32
4200cd92:	fdff3097          	auipc	ra,0xfdff3
4200cd96:	5c6080e7          	jalr	1478(ra) # 40000358 <memcpy>
4200cd9a:	02050793          	addi	a5,a0,32
4200cd9e:	02040413          	addi	s0,s0,32
4200cda2:	ff4794e3          	bne	a5,s4,4200cd8a <resnet_execute+0x972>
4200cda6:	47c1                	li	a5,16
4200cda8:	06fd0363          	beq	s10,a5,4200ce0e <resnet_execute+0x9f6>
4200cdac:	8552                	mv	a0,s4
4200cdae:	02000613          	li	a2,32
4200cdb2:	08000593          	li	a1,128
4200cdb6:	fdff3097          	auipc	ra,0xfdff3
4200cdba:	59e080e7          	jalr	1438(ra) # 40000354 <memset>
4200cdbe:	57a2                	lw	a5,40(sp)
4200cdc0:	220a0a13          	addi	s4,s4,544
4200cdc4:	0d05                	addi	s10,s10,1
4200cdc6:	20078793          	addi	a5,a5,512
4200cdca:	d43e                	sw	a5,40(sp)
4200cdcc:	4741                	li	a4,16
4200cdce:	843e                	mv	s0,a5
4200cdd0:	e00a0793          	addi	a5,s4,-512
4200cdd4:	faed1be3          	bne	s10,a4,4200cd8a <resnet_execute+0x972>
4200cdd8:	853e                	mv	a0,a5
4200cdda:	02000613          	li	a2,32
4200cdde:	08000593          	li	a1,128
4200cde2:	fdff3097          	auipc	ra,0xfdff3
4200cde6:	572080e7          	jalr	1394(ra) # 40000354 <memset>
4200cdea:	02050793          	addi	a5,a0,32
4200cdee:	fb478ce3          	beq	a5,s4,4200cda6 <resnet_execute+0x98e>
4200cdf2:	853e                	mv	a0,a5
4200cdf4:	02000613          	li	a2,32
4200cdf8:	08000593          	li	a1,128
4200cdfc:	fdff3097          	auipc	ra,0xfdff3
4200ce00:	558080e7          	jalr	1368(ra) # 40000354 <memset>
4200ce04:	02050793          	addi	a5,a0,32
4200ce08:	fd4798e3          	bne	a5,s4,4200cdd8 <resnet_execute+0x9c0>
4200ce0c:	bf69                	j	4200cda6 <resnet_execute+0x98e>
4200ce0e:	8552                	mv	a0,s4
4200ce10:	02000613          	li	a2,32
4200ce14:	08000593          	li	a1,128
4200ce18:	5432                	lw	s0,44(sp)
4200ce1a:	fdff3097          	auipc	ra,0xfdff3
4200ce1e:	53a080e7          	jalr	1338(ra) # 40000354 <memset>
4200ce22:	4799                	li	a5,6
4200ce24:	10fd8d63          	beq	s11,a5,4200cf3e <resnet_execute+0xb26>
4200ce28:	3c1ca7b7          	lui	a5,0x3c1ca
4200ce2c:	006d9693          	slli	a3,s11,0x6
4200ce30:	64078793          	addi	a5,a5,1600 # 3c1ca640 <r_kernels>
4200ce34:	97b6                	add	a5,a5,a3
4200ce36:	469d                	li	a3,7
4200ce38:	0207a803          	lw	a6,32(a5)
4200ce3c:	06dd8663          	beq	s11,a3,4200cea8 <resnet_execute+0xa90>
4200ce40:	4795                	li	a5,5
4200ce42:	00fd85e3          	beq	s11,a5,4200d64c <resnet_execute+0x1234>
4200ce46:	4791                	li	a5,4
4200ce48:	7efd8063          	beq	s11,a5,4200d628 <resnet_execute+0x1210>
4200ce4c:	478d                	li	a5,3
4200ce4e:	04fd83e3          	beq	s11,a5,4200d694 <resnet_execute+0x127c>
4200ce52:	3c25c6b7          	lui	a3,0x3c25c
4200ce56:	3c25c637          	lui	a2,0x3c25c
4200ce5a:	3c25c5b7          	lui	a1,0x3c25c
4200ce5e:	3c25c737          	lui	a4,0x3c25c
4200ce62:	accb0513          	addi	a0,s6,-1332
4200ce66:	c2068693          	addi	a3,a3,-992 # 3c25bc20 <r_mult_8>
4200ce6a:	d2060613          	addi	a2,a2,-736 # 3c25bd20 <r_bias_8>
4200ce6e:	e2058593          	addi	a1,a1,-480 # 3c25be20 <r_weights_8>
4200ce72:	b2070713          	addi	a4,a4,-1248 # 3c25bb20 <r_shift_8>
4200ce76:	0dc48793          	addi	a5,s1,220
4200ce7a:	9802                	jalr	a6
4200ce7c:	4521                	li	a0,8
4200ce7e:	ff9fe0ef          	jal	4200be76 <profile_end>
4200ce82:	040a8d63          	beqz	s5,4200cedc <resnet_execute+0xac4>
4200ce86:	3c1de7b7          	lui	a5,0x3c1de
4200ce8a:	6a878793          	addi	a5,a5,1704 # 3c1de6a8 <r_oracles>
4200ce8e:	97de                	add	a5,a5,s7
4200ce90:	439c                	lw	a5,0(a5)
4200ce92:	6805                	lui	a6,0x1
4200ce94:	0dc48713          	addi	a4,s1,220
4200ce98:	539c                	lw	a5,32(a5)
4200ce9a:	46a1                	li	a3,8
4200ce9c:	864a                	mv	a2,s2
4200ce9e:	85a2                	mv	a1,s0
4200cea0:	4505                	li	a0,1
4200cea2:	86eff0ef          	jal	4200bf10 <record_check>
4200cea6:	a81d                	j	4200cedc <resnet_execute+0xac4>
4200cea8:	3c2ab6b7          	lui	a3,0x3c2ab
4200ceac:	3c2ab637          	lui	a2,0x3c2ab
4200ceb0:	3c2ab5b7          	lui	a1,0x3c2ab
4200ceb4:	3c2ab737          	lui	a4,0x3c2ab
4200ceb8:	34068693          	addi	a3,a3,832 # 3c2ab340 <c7_mult_8>
4200cebc:	44060613          	addi	a2,a2,1088 # 3c2ab440 <c7_bias_8>
4200cec0:	54058593          	addi	a1,a1,1344 # 3c2ab540 <c7_weights_8>
4200cec4:	24070713          	addi	a4,a4,576 # 3c2ab240 <c7_shift_8>
4200cec8:	accb0513          	addi	a0,s6,-1332
4200cecc:	0dc48793          	addi	a5,s1,220
4200ced0:	9802                	jalr	a6
4200ced2:	4521                	li	a0,8
4200ced4:	fa3fe0ef          	jal	4200be76 <profile_end>
4200ced8:	fa0a97e3          	bnez	s5,4200ce86 <resnet_execute+0xa6e>
4200cedc:	4525                	li	a0,9
4200cede:	f4ffe0ef          	jal	4200be2c <profile_begin>
4200cee2:	85a2                	mv	a1,s0
4200cee4:	4625                	li	a2,9
4200cee6:	4505                	li	a0,1
4200cee8:	f0bfe0ef          	jal	4200bdf2 <choose_backend>
4200ceec:	4719                	li	a4,6
4200ceee:	1ee50163          	beq	a0,a4,4200d0d0 <resnet_execute+0xcb8>
4200cef2:	3c1ca737          	lui	a4,0x3c1ca
4200cef6:	00651693          	slli	a3,a0,0x6
4200cefa:	64070713          	addi	a4,a4,1600 # 3c1ca640 <r_kernels>
4200cefe:	9736                	add	a4,a4,a3
4200cf00:	469d                	li	a3,7
4200cf02:	02472803          	lw	a6,36(a4)
4200cf06:	08d50063          	beq	a0,a3,4200cf86 <resnet_execute+0xb6e>
4200cf0a:	4715                	li	a4,5
4200cf0c:	7ae50663          	beq	a0,a4,4200d6b8 <resnet_execute+0x12a0>
4200cf10:	4711                	li	a4,4
4200cf12:	7ee50663          	beq	a0,a4,4200d6fe <resnet_execute+0x12e6>
4200cf16:	470d                	li	a4,3
4200cf18:	7ce50163          	beq	a0,a4,4200d6da <resnet_execute+0x12c2>
4200cf1c:	3c2536b7          	lui	a3,0x3c253
4200cf20:	3c253637          	lui	a2,0x3c253
4200cf24:	3c2535b7          	lui	a1,0x3c253
4200cf28:	3c253737          	lui	a4,0x3c253
4200cf2c:	92068693          	addi	a3,a3,-1760 # 3c252920 <r_mult_9>
4200cf30:	a2060613          	addi	a2,a2,-1504 # 3c252a20 <r_bias_9>
4200cf34:	b2058593          	addi	a1,a1,-1248 # 3c252b20 <r_weights_9>
4200cf38:	82070713          	addi	a4,a4,-2016 # 3c252820 <r_shift_9>
4200cf3c:	a0ad                	j	4200cfa6 <resnet_execute+0xb8e>
4200cf3e:	6515                	lui	a0,0x5
4200cf40:	e1050513          	addi	a0,a0,-496 # 4e10 <RvExcFrameSize+0x4d7c>
4200cf44:	3c2975b7          	lui	a1,0x3c297
4200cf48:	6615                	lui	a2,0x5
4200cf4a:	6d858593          	addi	a1,a1,1752 # 3c2976d8 <c6_weights_8>
4200cf4e:	80060613          	addi	a2,a2,-2048 # 4800 <RvExcFrameSize+0x476c>
4200cf52:	954e                	add	a0,a0,s3
4200cf54:	fdff3097          	auipc	ra,0xfdff3
4200cf58:	404080e7          	jalr	1028(ra) # 40000358 <memcpy>
4200cf5c:	3c2976b7          	lui	a3,0x3c297
4200cf60:	3c297637          	lui	a2,0x3c297
4200cf64:	3fc945b7          	lui	a1,0x3fc94
4200cf68:	42033837          	lui	a6,0x42033
4200cf6c:	3c297737          	lui	a4,0x3c297
4200cf70:	4d868693          	addi	a3,a3,1240 # 3c2974d8 <c6_mult_8>
4200cf74:	5d860613          	addi	a2,a2,1496 # 3c2975d8 <c6_bias_8>
4200cf78:	8dc58593          	addi	a1,a1,-1828 # 3fc938dc <padded+0x4e10>
4200cf7c:	19080813          	addi	a6,a6,400 # 42033190 <r_tile16_b2p4_sram_layer_8>
4200cf80:	3d870713          	addi	a4,a4,984 # 3c2973d8 <c6_shift_8>
4200cf84:	b791                	j	4200cec8 <resnet_execute+0xab0>
4200cf86:	3c2a26b7          	lui	a3,0x3c2a2
4200cf8a:	3c2a2637          	lui	a2,0x3c2a2
4200cf8e:	3c2a25b7          	lui	a1,0x3c2a2
4200cf92:	3c2a2737          	lui	a4,0x3c2a2
4200cf96:	04068693          	addi	a3,a3,64 # 3c2a2040 <c7_mult_9>
4200cf9a:	14060613          	addi	a2,a2,320 # 3c2a2140 <c7_bias_9>
4200cf9e:	24058593          	addi	a1,a1,576 # 3c2a2240 <c7_weights_9>
4200cfa2:	f4070713          	addi	a4,a4,-192 # 3c2a1f40 <c7_shift_9>
4200cfa6:	3fc99a37          	lui	s4,0x3fc99
4200cfaa:	0dc48513          	addi	a0,s1,220
4200cfae:	0dca0793          	addi	a5,s4,220 # 3fc990dc <activations+0x1000>
4200cfb2:	9802                	jalr	a6
4200cfb4:	4525                	li	a0,9
4200cfb6:	ec1fe0ef          	jal	4200be76 <profile_end>
4200cfba:	560a9f63          	bnez	s5,4200d538 <resnet_execute+0x1120>
4200cfbe:	4529                	li	a0,10
4200cfc0:	e6dfe0ef          	jal	4200be2c <profile_begin>
4200cfc4:	85a2                	mv	a1,s0
4200cfc6:	4629                	li	a2,10
4200cfc8:	4505                	li	a0,1
4200cfca:	e29fe0ef          	jal	4200bdf2 <choose_backend>
4200cfce:	4719                	li	a4,6
4200cfd0:	52e50063          	beq	a0,a4,4200d4f0 <resnet_execute+0x10d8>
4200cfd4:	3c1ca737          	lui	a4,0x3c1ca
4200cfd8:	00651693          	slli	a3,a0,0x6
4200cfdc:	64070713          	addi	a4,a4,1600 # 3c1ca640 <r_kernels>
4200cfe0:	9736                	add	a4,a4,a3
4200cfe2:	469d                	li	a3,7
4200cfe4:	02872803          	lw	a6,40(a4)
4200cfe8:	18d50863          	beq	a0,a3,4200d178 <resnet_execute+0xd60>
4200cfec:	4715                	li	a4,5
4200cfee:	60e50c63          	beq	a0,a4,4200d606 <resnet_execute+0x11ee>
4200cff2:	4711                	li	a4,4
4200cff4:	5ee50863          	beq	a0,a4,4200d5e4 <resnet_execute+0x11cc>
4200cff8:	470d                	li	a4,3
4200cffa:	72e50463          	beq	a0,a4,4200d722 <resnet_execute+0x130a>
4200cffe:	3c2526b7          	lui	a3,0x3c252
4200d002:	3c252637          	lui	a2,0x3c252
4200d006:	3c2525b7          	lui	a1,0x3c252
4200d00a:	3c252737          	lui	a4,0x3c252
4200d00e:	0dcc0513          	addi	a0,s8,220
4200d012:	e2068693          	addi	a3,a3,-480 # 3c251e20 <r_mult_10>
4200d016:	f2060613          	addi	a2,a2,-224 # 3c251f20 <r_bias_10>
4200d01a:	02058593          	addi	a1,a1,32 # 3c252020 <r_weights_10>
4200d01e:	d2070713          	addi	a4,a4,-736 # 3c251d20 <r_shift_10>
4200d022:	0dc48793          	addi	a5,s1,220
4200d026:	9802                	jalr	a6
4200d028:	4529                	li	a0,10
4200d02a:	e4dfe0ef          	jal	4200be76 <profile_end>
4200d02e:	160a8f63          	beqz	s5,4200d1ac <resnet_execute+0xd94>
4200d032:	3c1de7b7          	lui	a5,0x3c1de
4200d036:	6a878793          	addi	a5,a5,1704 # 3c1de6a8 <r_oracles>
4200d03a:	97de                	add	a5,a5,s7
4200d03c:	0007ab03          	lw	s6,0(a5)
4200d040:	0dc48713          	addi	a4,s1,220
4200d044:	6805                	lui	a6,0x1
4200d046:	028b2783          	lw	a5,40(s6)
4200d04a:	46a9                	li	a3,10
4200d04c:	864a                	mv	a2,s2
4200d04e:	85a2                	mv	a1,s0
4200d050:	4505                	li	a0,1
4200d052:	ebffe0ef          	jal	4200bf10 <record_check>
4200d056:	452d                	li	a0,11
4200d058:	dd5fe0ef          	jal	4200be2c <profile_begin>
4200d05c:	85a2                	mv	a1,s0
4200d05e:	462d                	li	a2,11
4200d060:	4505                	li	a0,1
4200d062:	d91fe0ef          	jal	4200bdf2 <choose_backend>
4200d066:	6d6ae7b7          	lui	a5,0x6d6ae
4200d06a:	0dcc8513          	addi	a0,s9,220
4200d06e:	6c05                	lui	s8,0x1
4200d070:	f8000713          	li	a4,-128
4200d074:	07f00893          	li	a7,127
4200d078:	5839                	li	a6,-18
4200d07a:	3c778793          	addi	a5,a5,967 # 6d6ae3c7 <SYSTEM+0xd5ee3c7>
4200d07e:	4651                	li	a2,20
4200d080:	62d196b7          	lui	a3,0x62d19
4200d084:	0dca0593          	addi	a1,s4,220
4200d088:	cc46                	sw	a7,24(sp)
4200d08a:	c842                	sw	a6,16(sp)
4200d08c:	c63e                	sw	a5,12(sp)
4200d08e:	c22a                	sw	a0,4(sp)
4200d090:	c032                	sw	a2,0(sp)
4200d092:	ca3a                	sw	a4,20(sp)
4200d094:	c43a                	sw	a4,8(sp)
4200d096:	3c368693          	addi	a3,a3,963 # 62d193c3 <SYSTEM+0x2c593c3>
4200d09a:	4881                	li	a7,0
4200d09c:	40000837          	lui	a6,0x40000
4200d0a0:	4789                	li	a5,2
4200d0a2:	5779                	li	a4,-2
4200d0a4:	fda00613          	li	a2,-38
4200d0a8:	ce62                	sw	s8,28(sp)
4200d0aa:	0dc48513          	addi	a0,s1,220
4200d0ae:	5da020ef          	jal	4200f688 <muriscv_nn_elementwise_add_s8>
4200d0b2:	452d                	li	a0,11
4200d0b4:	dc3fe0ef          	jal	4200be76 <profile_end>
4200d0b8:	02cb2783          	lw	a5,44(s6)
4200d0bc:	8862                	mv	a6,s8
4200d0be:	0dcc8713          	addi	a4,s9,220
4200d0c2:	864a                	mv	a2,s2
4200d0c4:	85a2                	mv	a1,s0
4200d0c6:	46ad                	li	a3,11
4200d0c8:	4505                	li	a0,1
4200d0ca:	e47fe0ef          	jal	4200bf10 <record_check>
4200d0ce:	a281                	j	4200d20e <resnet_execute+0xdf6>
4200d0d0:	3c28e5b7          	lui	a1,0x3c28e
4200d0d4:	3d858593          	addi	a1,a1,984 # 3c28e3d8 <c6_weights_9>
4200d0d8:	6625                	lui	a2,0x9
4200d0da:	61098513          	addi	a0,s3,1552
4200d0de:	fdff3097          	auipc	ra,0xfdff3
4200d0e2:	27a080e7          	jalr	634(ra) # 40000358 <memcpy>
4200d0e6:	3c28e6b7          	lui	a3,0x3c28e
4200d0ea:	3c28e637          	lui	a2,0x3c28e
4200d0ee:	3fc8f5b7          	lui	a1,0x3fc8f
4200d0f2:	42034837          	lui	a6,0x42034
4200d0f6:	3c28e737          	lui	a4,0x3c28e
4200d0fa:	1d868693          	addi	a3,a3,472 # 3c28e1d8 <c6_mult_9>
4200d0fe:	2d860613          	addi	a2,a2,728 # 3c28e2d8 <c6_bias_9>
4200d102:	0dc58593          	addi	a1,a1,220 # 3fc8f0dc <padded+0x610>
4200d106:	b0280813          	addi	a6,a6,-1278 # 42033b02 <r_tile16_b2p4_sram_layer_9>
4200d10a:	0d870713          	addi	a4,a4,216 # 3c28e0d8 <c6_shift_9>
4200d10e:	bd61                	j	4200cfa6 <resnet_execute+0xb8e>
4200d110:	6525                	lui	a0,0x9
4200d112:	41050513          	addi	a0,a0,1040 # 9410 <RvExcFrameSize+0x937c>
4200d116:	3c29c5b7          	lui	a1,0x3c29c
4200d11a:	05858593          	addi	a1,a1,88 # 3c29c058 <c6_weights_6>
4200d11e:	20000613          	li	a2,512
4200d122:	954e                	add	a0,a0,s3
4200d124:	fdff3097          	auipc	ra,0xfdff3
4200d128:	234080e7          	jalr	564(ra) # 40000358 <memcpy>
4200d12c:	3c29c6b7          	lui	a3,0x3c29c
4200d130:	3c29c637          	lui	a2,0x3c29c
4200d134:	3fc985b7          	lui	a1,0x3fc98
4200d138:	42033837          	lui	a6,0x42033
4200d13c:	3c29c737          	lui	a4,0x3c29c
4200d140:	f5868693          	addi	a3,a3,-168 # 3c29bf58 <c6_mult_6>
4200d144:	fd860613          	addi	a2,a2,-40 # 3c29bfd8 <c6_bias_6>
4200d148:	edc58593          	addi	a1,a1,-292 # 3fc97edc <padded+0x9410>
4200d14c:	a6480813          	addi	a6,a6,-1436 # 42032a64 <r_tile16_b2p4_sram_layer_6>
4200d150:	ed870713          	addi	a4,a4,-296 # 3c29bed8 <c6_shift_6>
4200d154:	b655                	j	4200ccf8 <resnet_execute+0x8e0>
4200d156:	3c1de7b7          	lui	a5,0x3c1de
4200d15a:	6a878793          	addi	a5,a5,1704 # 3c1de6a8 <r_oracles>
4200d15e:	97de                	add	a5,a5,s7
4200d160:	439c                	lw	a5,0(a5)
4200d162:	6809                	lui	a6,0x2
4200d164:	0dcc8713          	addi	a4,s9,220
4200d168:	4bdc                	lw	a5,20(a5)
4200d16a:	4695                	li	a3,5
4200d16c:	864a                	mv	a2,s2
4200d16e:	85a2                	mv	a1,s0
4200d170:	4505                	li	a0,1
4200d172:	d9ffe0ef          	jal	4200bf10 <record_check>
4200d176:	b24d                	j	4200cb18 <resnet_execute+0x700>
4200d178:	3c2a16b7          	lui	a3,0x3c2a1
4200d17c:	3c2a1637          	lui	a2,0x3c2a1
4200d180:	3c2a15b7          	lui	a1,0x3c2a1
4200d184:	3c2a1737          	lui	a4,0x3c2a1
4200d188:	54068693          	addi	a3,a3,1344 # 3c2a1540 <c7_mult_10>
4200d18c:	64060613          	addi	a2,a2,1600 # 3c2a1640 <c7_bias_10>
4200d190:	74058593          	addi	a1,a1,1856 # 3c2a1740 <c7_weights_10>
4200d194:	44070713          	addi	a4,a4,1088 # 3c2a1440 <c7_shift_10>
4200d198:	0dcc0513          	addi	a0,s8,220 # 10dc <RvExcFrameSize+0x1048>
4200d19c:	0dc48793          	addi	a5,s1,220
4200d1a0:	9802                	jalr	a6
4200d1a2:	4529                	li	a0,10
4200d1a4:	cd3fe0ef          	jal	4200be76 <profile_end>
4200d1a8:	e80a95e3          	bnez	s5,4200d032 <resnet_execute+0xc1a>
4200d1ac:	452d                	li	a0,11
4200d1ae:	c7ffe0ef          	jal	4200be2c <profile_begin>
4200d1b2:	85a2                	mv	a1,s0
4200d1b4:	462d                	li	a2,11
4200d1b6:	4505                	li	a0,1
4200d1b8:	c3bfe0ef          	jal	4200bdf2 <choose_backend>
4200d1bc:	6d6ae7b7          	lui	a5,0x6d6ae
4200d1c0:	f8000713          	li	a4,-128
4200d1c4:	6885                	lui	a7,0x1
4200d1c6:	07f00813          	li	a6,127
4200d1ca:	5539                	li	a0,-18
4200d1cc:	3c778793          	addi	a5,a5,967 # 6d6ae3c7 <SYSTEM+0xd5ee3c7>
4200d1d0:	4651                	li	a2,20
4200d1d2:	0dcc8c93          	addi	s9,s9,220
4200d1d6:	62d196b7          	lui	a3,0x62d19
4200d1da:	ce46                	sw	a7,28(sp)
4200d1dc:	cc42                	sw	a6,24(sp)
4200d1de:	c82a                	sw	a0,16(sp)
4200d1e0:	c63e                	sw	a5,12(sp)
4200d1e2:	c032                	sw	a2,0(sp)
4200d1e4:	ca3a                	sw	a4,20(sp)
4200d1e6:	c43a                	sw	a4,8(sp)
4200d1e8:	c266                	sw	s9,4(sp)
4200d1ea:	0dca0593          	addi	a1,s4,220
4200d1ee:	0dc48513          	addi	a0,s1,220
4200d1f2:	3c368693          	addi	a3,a3,963 # 62d193c3 <SYSTEM+0x2c593c3>
4200d1f6:	4881                	li	a7,0
4200d1f8:	40000837          	lui	a6,0x40000
4200d1fc:	4789                	li	a5,2
4200d1fe:	5779                	li	a4,-2
4200d200:	fda00613          	li	a2,-38
4200d204:	484020ef          	jal	4200f688 <muriscv_nn_elementwise_add_s8>
4200d208:	452d                	li	a0,11
4200d20a:	c6dfe0ef          	jal	4200be76 <profile_end>
4200d20e:	4531                	li	a0,12
4200d210:	c1dfe0ef          	jal	4200be2c <profile_begin>
4200d214:	85a2                	mv	a1,s0
4200d216:	4631                	li	a2,12
4200d218:	4505                	li	a0,1
4200d21a:	bd9fe0ef          	jal	4200bdf2 <choose_backend>
4200d21e:	650d                	lui	a0,0x3
4200d220:	0dc48593          	addi	a1,s1,220
4200d224:	862a                	mv	a2,a0
4200d226:	04050513          	addi	a0,a0,64 # 3040 <RvExcFrameSize+0x2fac>
4200d22a:	962e                	add	a2,a2,a1
4200d22c:	952e                	add	a0,a0,a1
4200d22e:	787d                	lui	a6,0xfffff
4200d230:	010607b3          	add	a5,a2,a6
4200d234:	4701                	li	a4,0
4200d236:	00078683          	lb	a3,0(a5)
4200d23a:	04078793          	addi	a5,a5,64
4200d23e:	9736                	add	a4,a4,a3
4200d240:	fec79be3          	bne	a5,a2,4200d236 <resnet_execute+0xe1e>
4200d244:	14e05763          	blez	a4,4200d392 <resnet_execute+0xf7a>
4200d248:	02070793          	addi	a5,a4,32
4200d24c:	8799                	srai	a5,a5,0x6
4200d24e:	00f58023          	sb	a5,0(a1)
4200d252:	0605                	addi	a2,a2,1
4200d254:	0585                	addi	a1,a1,1
4200d256:	fcc51de3          	bne	a0,a2,4200d230 <resnet_execute+0xe18>
4200d25a:	4531                	li	a0,12
4200d25c:	c1bfe0ef          	jal	4200be76 <profile_end>
4200d260:	140a9b63          	bnez	s5,4200d3b6 <resnet_execute+0xf9e>
4200d264:	4535                	li	a0,13
4200d266:	bc7fe0ef          	jal	4200be2c <profile_begin>
4200d26a:	85a2                	mv	a1,s0
4200d26c:	4635                	li	a2,13
4200d26e:	4505                	li	a0,1
4200d270:	b83fe0ef          	jal	4200bdf2 <choose_backend>
4200d274:	3fc98a37          	lui	s4,0x3fc98
4200d278:	0dc48593          	addi	a1,s1,220
4200d27c:	11ca0513          	addi	a0,s4,284 # 3fc9811c <activations+0x40>
4200d280:	04000613          	li	a2,64
4200d284:	fdff3097          	auipc	ra,0xfdff3
4200d288:	0d4080e7          	jalr	212(ra) # 40000358 <memcpy>
4200d28c:	4535                	li	a0,13
4200d28e:	be9fe0ef          	jal	4200be76 <profile_end>
4200d292:	4539                	li	a0,14
4200d294:	b99fe0ef          	jal	4200be2c <profile_begin>
4200d298:	85a2                	mv	a1,s0
4200d29a:	4639                	li	a2,14
4200d29c:	4505                	li	a0,1
4200d29e:	b55fe0ef          	jal	4200bdf2 <choose_backend>
4200d2a2:	4719                	li	a4,6
4200d2a4:	20e50363          	beq	a0,a4,4200d4aa <resnet_execute+0x1092>
4200d2a8:	3c1ca737          	lui	a4,0x3c1ca
4200d2ac:	00651693          	slli	a3,a0,0x6
4200d2b0:	64070713          	addi	a4,a4,1600 # 3c1ca640 <r_kernels>
4200d2b4:	9736                	add	a4,a4,a3
4200d2b6:	469d                	li	a3,7
4200d2b8:	03872803          	lw	a6,56(a4)
4200d2bc:	16d50463          	beq	a0,a3,4200d424 <resnet_execute+0x100c>
4200d2c0:	4715                	li	a4,5
4200d2c2:	2ce50f63          	beq	a0,a4,4200d5a0 <resnet_execute+0x1188>
4200d2c6:	4711                	li	a4,4
4200d2c8:	2ae50b63          	beq	a0,a4,4200d57e <resnet_execute+0x1166>
4200d2cc:	470d                	li	a4,3
4200d2ce:	2ee50a63          	beq	a0,a4,4200d5c2 <resnet_execute+0x11aa>
4200d2d2:	3c2526b7          	lui	a3,0x3c252
4200d2d6:	3c252637          	lui	a2,0x3c252
4200d2da:	3c2525b7          	lui	a1,0x3c252
4200d2de:	3c252737          	lui	a4,0x3c252
4200d2e2:	11ca0513          	addi	a0,s4,284
4200d2e6:	a5068693          	addi	a3,a3,-1456 # 3c251a50 <r_mult_14>
4200d2ea:	a7860613          	addi	a2,a2,-1416 # 3c251a78 <r_bias_14>
4200d2ee:	aa058593          	addi	a1,a1,-1376 # 3c251aa0 <r_weights_14>
4200d2f2:	a2870713          	addi	a4,a4,-1496 # 3c251a28 <r_shift_14>
4200d2f6:	0dc48793          	addi	a5,s1,220
4200d2fa:	9802                	jalr	a6
4200d2fc:	4539                	li	a0,14
4200d2fe:	b79fe0ef          	jal	4200be76 <profile_end>
4200d302:	140a8b63          	beqz	s5,4200d458 <resnet_execute+0x1040>
4200d306:	3c1de7b7          	lui	a5,0x3c1de
4200d30a:	6a878793          	addi	a5,a5,1704 # 3c1de6a8 <r_oracles>
4200d30e:	97de                	add	a5,a5,s7
4200d310:	0007aa03          	lw	s4,0(a5)
4200d314:	0dc48713          	addi	a4,s1,220
4200d318:	4829                	li	a6,10
4200d31a:	038a2783          	lw	a5,56(s4)
4200d31e:	46b9                	li	a3,14
4200d320:	864a                	mv	a2,s2
4200d322:	85a2                	mv	a1,s0
4200d324:	4505                	li	a0,1
4200d326:	bebfe0ef          	jal	4200bf10 <record_check>
4200d32a:	453d                	li	a0,15
4200d32c:	b01fe0ef          	jal	4200be2c <profile_begin>
4200d330:	85a2                	mv	a1,s0
4200d332:	463d                	li	a2,15
4200d334:	4505                	li	a0,1
4200d336:	abdfe0ef          	jal	4200bdf2 <choose_backend>
4200d33a:	3fc989b7          	lui	s3,0x3fc98
4200d33e:	57fd36b7          	lui	a3,0x57fd3
4200d342:	0ec98813          	addi	a6,s3,236 # 3fc980ec <activations+0x10>
4200d346:	0dc48513          	addi	a0,s1,220
4200d34a:	f0068693          	addi	a3,a3,-256 # 57fd2f00 <_rtc_reserved_end+0x7fd0f00>
4200d34e:	f8400793          	li	a5,-124
4200d352:	4761                	li	a4,24
4200d354:	4629                	li	a2,10
4200d356:	4585                	li	a1,1
4200d358:	2585                	jal	4200d9b8 <muriscv_nn_softmax_s8>
4200d35a:	453d                	li	a0,15
4200d35c:	b1bfe0ef          	jal	4200be76 <profile_end>
4200d360:	85a2                	mv	a1,s0
4200d362:	5426                	lw	s0,104(sp)
4200d364:	03ca2783          	lw	a5,60(s4)
4200d368:	50b6                	lw	ra,108(sp)
4200d36a:	5496                	lw	s1,100(sp)
4200d36c:	4a66                	lw	s4,88(sp)
4200d36e:	4ad6                	lw	s5,84(sp)
4200d370:	4b46                	lw	s6,80(sp)
4200d372:	4bb6                	lw	s7,76(sp)
4200d374:	4c26                	lw	s8,72(sp)
4200d376:	4c96                	lw	s9,68(sp)
4200d378:	4d06                	lw	s10,64(sp)
4200d37a:	5df2                	lw	s11,60(sp)
4200d37c:	0ec98713          	addi	a4,s3,236
4200d380:	864a                	mv	a2,s2
4200d382:	49f6                	lw	s3,92(sp)
4200d384:	5906                	lw	s2,96(sp)
4200d386:	4829                	li	a6,10
4200d388:	46bd                	li	a3,15
4200d38a:	4505                	li	a0,1
4200d38c:	6165                	addi	sp,sp,112
4200d38e:	b83fe06f          	j	4200bf10 <record_check>
4200d392:	1701                	addi	a4,a4,-32
4200d394:	41f75793          	srai	a5,a4,0x1f
4200d398:	03f7f793          	andi	a5,a5,63
4200d39c:	97ba                	add	a5,a5,a4
4200d39e:	8799                	srai	a5,a5,0x6
4200d3a0:	00f58023          	sb	a5,0(a1)
4200d3a4:	0605                	addi	a2,a2,1
4200d3a6:	0585                	addi	a1,a1,1
4200d3a8:	e8c514e3          	bne	a0,a2,4200d230 <resnet_execute+0xe18>
4200d3ac:	4531                	li	a0,12
4200d3ae:	ac9fe0ef          	jal	4200be76 <profile_end>
4200d3b2:	ea0a89e3          	beqz	s5,4200d264 <resnet_execute+0xe4c>
4200d3b6:	3c1de7b7          	lui	a5,0x3c1de
4200d3ba:	6a878793          	addi	a5,a5,1704 # 3c1de6a8 <r_oracles>
4200d3be:	97de                	add	a5,a5,s7
4200d3c0:	0007ab03          	lw	s6,0(a5)
4200d3c4:	0dc48713          	addi	a4,s1,220
4200d3c8:	04000813          	li	a6,64
4200d3cc:	030b2783          	lw	a5,48(s6)
4200d3d0:	46b1                	li	a3,12
4200d3d2:	864a                	mv	a2,s2
4200d3d4:	85a2                	mv	a1,s0
4200d3d6:	4505                	li	a0,1
4200d3d8:	b39fe0ef          	jal	4200bf10 <record_check>
4200d3dc:	4535                	li	a0,13
4200d3de:	a4ffe0ef          	jal	4200be2c <profile_begin>
4200d3e2:	85a2                	mv	a1,s0
4200d3e4:	4635                	li	a2,13
4200d3e6:	4505                	li	a0,1
4200d3e8:	a0bfe0ef          	jal	4200bdf2 <choose_backend>
4200d3ec:	3fc98a37          	lui	s4,0x3fc98
4200d3f0:	0dc48593          	addi	a1,s1,220
4200d3f4:	04000613          	li	a2,64
4200d3f8:	11ca0513          	addi	a0,s4,284 # 3fc9811c <activations+0x40>
4200d3fc:	fdff3097          	auipc	ra,0xfdff3
4200d400:	f5c080e7          	jalr	-164(ra) # 40000358 <memcpy>
4200d404:	4535                	li	a0,13
4200d406:	a71fe0ef          	jal	4200be76 <profile_end>
4200d40a:	034b2783          	lw	a5,52(s6)
4200d40e:	11ca0713          	addi	a4,s4,284
4200d412:	864a                	mv	a2,s2
4200d414:	85a2                	mv	a1,s0
4200d416:	04000813          	li	a6,64
4200d41a:	46b5                	li	a3,13
4200d41c:	4505                	li	a0,1
4200d41e:	af3fe0ef          	jal	4200bf10 <record_check>
4200d422:	bd85                	j	4200d292 <resnet_execute+0xe7a>
4200d424:	3c2a16b7          	lui	a3,0x3c2a1
4200d428:	3c2a1637          	lui	a2,0x3c2a1
4200d42c:	3c2a15b7          	lui	a1,0x3c2a1
4200d430:	3c2a1737          	lui	a4,0x3c2a1
4200d434:	17068693          	addi	a3,a3,368 # 3c2a1170 <c7_mult_14>
4200d438:	19860613          	addi	a2,a2,408 # 3c2a1198 <c7_bias_14>
4200d43c:	1c058593          	addi	a1,a1,448 # 3c2a11c0 <c6_weights_14>
4200d440:	14870713          	addi	a4,a4,328 # 3c2a1148 <c7_shift_14>
4200d444:	11ca0513          	addi	a0,s4,284
4200d448:	0dc48793          	addi	a5,s1,220
4200d44c:	9802                	jalr	a6
4200d44e:	4539                	li	a0,14
4200d450:	a27fe0ef          	jal	4200be76 <profile_end>
4200d454:	ea0a99e3          	bnez	s5,4200d306 <resnet_execute+0xeee>
4200d458:	453d                	li	a0,15
4200d45a:	9d3fe0ef          	jal	4200be2c <profile_begin>
4200d45e:	85a2                	mv	a1,s0
4200d460:	463d                	li	a2,15
4200d462:	4505                	li	a0,1
4200d464:	98ffe0ef          	jal	4200bdf2 <choose_backend>
4200d468:	3fc98837          	lui	a6,0x3fc98
4200d46c:	57fd36b7          	lui	a3,0x57fd3
4200d470:	0dc48513          	addi	a0,s1,220
4200d474:	0ec80813          	addi	a6,a6,236 # 3fc980ec <activations+0x10>
4200d478:	f0068693          	addi	a3,a3,-256 # 57fd2f00 <_rtc_reserved_end+0x7fd0f00>
4200d47c:	f8400793          	li	a5,-124
4200d480:	4761                	li	a4,24
4200d482:	4629                	li	a2,10
4200d484:	4585                	li	a1,1
4200d486:	2b0d                	jal	4200d9b8 <muriscv_nn_softmax_s8>
4200d488:	5426                	lw	s0,104(sp)
4200d48a:	50b6                	lw	ra,108(sp)
4200d48c:	5496                	lw	s1,100(sp)
4200d48e:	5906                	lw	s2,96(sp)
4200d490:	49f6                	lw	s3,92(sp)
4200d492:	4a66                	lw	s4,88(sp)
4200d494:	4ad6                	lw	s5,84(sp)
4200d496:	4b46                	lw	s6,80(sp)
4200d498:	4bb6                	lw	s7,76(sp)
4200d49a:	4c26                	lw	s8,72(sp)
4200d49c:	4c96                	lw	s9,68(sp)
4200d49e:	4d06                	lw	s10,64(sp)
4200d4a0:	5df2                	lw	s11,60(sp)
4200d4a2:	453d                	li	a0,15
4200d4a4:	6165                	addi	sp,sp,112
4200d4a6:	9d1fe06f          	j	4200be76 <profile_end>
4200d4aa:	6525                	lui	a0,0x9
4200d4ac:	39050513          	addi	a0,a0,912 # 9390 <RvExcFrameSize+0x92fc>
4200d4b0:	3c2a15b7          	lui	a1,0x3c2a1
4200d4b4:	1c058593          	addi	a1,a1,448 # 3c2a11c0 <c6_weights_14>
4200d4b8:	28000613          	li	a2,640
4200d4bc:	954e                	add	a0,a0,s3
4200d4be:	fdff3097          	auipc	ra,0xfdff3
4200d4c2:	e9a080e7          	jalr	-358(ra) # 40000358 <memcpy>
4200d4c6:	3c28d6b7          	lui	a3,0x3c28d
4200d4ca:	3c28d637          	lui	a2,0x3c28d
4200d4ce:	3fc985b7          	lui	a1,0x3fc98
4200d4d2:	42034837          	lui	a6,0x42034
4200d4d6:	3c28d737          	lui	a4,0x3c28d
4200d4da:	58868693          	addi	a3,a3,1416 # 3c28d588 <c6_mult_14>
4200d4de:	5b060613          	addi	a2,a2,1456 # 3c28d5b0 <c6_bias_14>
4200d4e2:	e5c58593          	addi	a1,a1,-420 # 3fc97e5c <padded+0x9390>
4200d4e6:	2f480813          	addi	a6,a6,756 # 420342f4 <r_tile16_b2p4_sram_layer_14>
4200d4ea:	56070713          	addi	a4,a4,1376 # 3c28d560 <c6_shift_14>
4200d4ee:	bf99                	j	4200d444 <resnet_execute+0x102c>
4200d4f0:	6525                	lui	a0,0x9
4200d4f2:	e1050513          	addi	a0,a0,-496 # 8e10 <RvExcFrameSize+0x8d7c>
4200d4f6:	3c28e5b7          	lui	a1,0x3c28e
4200d4fa:	6605                	lui	a2,0x1
4200d4fc:	8d858593          	addi	a1,a1,-1832 # 3c28d8d8 <c6_weights_10>
4200d500:	80060613          	addi	a2,a2,-2048 # 800 <RvExcFrameSize+0x76c>
4200d504:	954e                	add	a0,a0,s3
4200d506:	fdff3097          	auipc	ra,0xfdff3
4200d50a:	e52080e7          	jalr	-430(ra) # 40000358 <memcpy>
4200d50e:	3c28d6b7          	lui	a3,0x3c28d
4200d512:	3c28d637          	lui	a2,0x3c28d
4200d516:	3fc985b7          	lui	a1,0x3fc98
4200d51a:	42034837          	lui	a6,0x42034
4200d51e:	3c28d737          	lui	a4,0x3c28d
4200d522:	6d868693          	addi	a3,a3,1752 # 3c28d6d8 <c6_mult_10>
4200d526:	7d860613          	addi	a2,a2,2008 # 3c28d7d8 <c6_bias_10>
4200d52a:	8dc58593          	addi	a1,a1,-1828 # 3fc978dc <padded+0x8e10>
4200d52e:	14a80813          	addi	a6,a6,330 # 4203414a <r_tile16_b2p4_sram_layer_10>
4200d532:	5d870713          	addi	a4,a4,1496 # 3c28d5d8 <c6_shift_10>
4200d536:	b18d                	j	4200d198 <resnet_execute+0xd80>
4200d538:	3c1de7b7          	lui	a5,0x3c1de
4200d53c:	6a878793          	addi	a5,a5,1704 # 3c1de6a8 <r_oracles>
4200d540:	97de                	add	a5,a5,s7
4200d542:	439c                	lw	a5,0(a5)
4200d544:	6805                	lui	a6,0x1
4200d546:	0dca0713          	addi	a4,s4,220
4200d54a:	53dc                	lw	a5,36(a5)
4200d54c:	46a5                	li	a3,9
4200d54e:	864a                	mv	a2,s2
4200d550:	85a2                	mv	a1,s0
4200d552:	4505                	li	a0,1
4200d554:	9bdfe0ef          	jal	4200bf10 <record_check>
4200d558:	b49d                	j	4200cfbe <resnet_execute+0xba6>
4200d55a:	3c28d6b7          	lui	a3,0x3c28d
4200d55e:	3c28d637          	lui	a2,0x3c28d
4200d562:	3c28d5b7          	lui	a1,0x3c28d
4200d566:	3c28d737          	lui	a4,0x3c28d
4200d56a:	97068693          	addi	a3,a3,-1680 # 3c28c970 <c5_mult_1>
4200d56e:	9b060613          	addi	a2,a2,-1616 # 3c28c9b0 <c5_bias_1>
4200d572:	9f058593          	addi	a1,a1,-1552 # 3c28c9f0 <c5_weights_1>
4200d576:	93070713          	addi	a4,a4,-1744 # 3c28c930 <c5_shift_1>
4200d57a:	862ff06f          	j	4200c5dc <resnet_execute+0x1c4>
4200d57e:	3c2666b7          	lui	a3,0x3c266
4200d582:	3c266637          	lui	a2,0x3c266
4200d586:	3c2665b7          	lui	a1,0x3c266
4200d58a:	3c266737          	lui	a4,0x3c266
4200d58e:	8b868693          	addi	a3,a3,-1864 # 3c2658b8 <c4_mult_14>
4200d592:	8e060613          	addi	a2,a2,-1824 # 3c2658e0 <c4_bias_14>
4200d596:	90858593          	addi	a1,a1,-1784 # 3c265908 <c4_weights_14>
4200d59a:	89070713          	addi	a4,a4,-1904 # 3c265890 <c4_shift_14>
4200d59e:	b55d                	j	4200d444 <resnet_execute+0x102c>
4200d5a0:	3c2796b7          	lui	a3,0x3c279
4200d5a4:	3c279637          	lui	a2,0x3c279
4200d5a8:	3c2795b7          	lui	a1,0x3c279
4200d5ac:	3c279737          	lui	a4,0x3c279
4200d5b0:	72068693          	addi	a3,a3,1824 # 3c279720 <c5_mult_14>
4200d5b4:	74860613          	addi	a2,a2,1864 # 3c279748 <c5_bias_14>
4200d5b8:	77058593          	addi	a1,a1,1904 # 3c279770 <c5_weights_14>
4200d5bc:	6f870713          	addi	a4,a4,1784 # 3c2796f8 <c5_shift_14>
4200d5c0:	b551                	j	4200d444 <resnet_execute+0x102c>
4200d5c2:	3c1cb6b7          	lui	a3,0x3c1cb
4200d5c6:	3c1cb637          	lui	a2,0x3c1cb
4200d5ca:	3c1cb5b7          	lui	a1,0x3c1cb
4200d5ce:	3c1cb737          	lui	a4,0x3c1cb
4200d5d2:	86868693          	addi	a3,a3,-1944 # 3c1ca868 <t_mult_14>
4200d5d6:	89060613          	addi	a2,a2,-1904 # 3c1ca890 <t_bias_14>
4200d5da:	8b858593          	addi	a1,a1,-1864 # 3c1ca8b8 <t_weights_14>
4200d5de:	84070713          	addi	a4,a4,-1984 # 3c1ca840 <t_shift_14>
4200d5e2:	b58d                	j	4200d444 <resnet_execute+0x102c>
4200d5e4:	3c2666b7          	lui	a3,0x3c266
4200d5e8:	3c266637          	lui	a2,0x3c266
4200d5ec:	3c2665b7          	lui	a1,0x3c266
4200d5f0:	3c266737          	lui	a4,0x3c266
4200d5f4:	c8868693          	addi	a3,a3,-888 # 3c265c88 <c4_mult_10>
4200d5f8:	d8860613          	addi	a2,a2,-632 # 3c265d88 <c4_bias_10>
4200d5fc:	e8858593          	addi	a1,a1,-376 # 3c265e88 <c4_weights_10>
4200d600:	b8870713          	addi	a4,a4,-1144 # 3c265b88 <c4_shift_10>
4200d604:	be51                	j	4200d198 <resnet_execute+0xd80>
4200d606:	3c27a6b7          	lui	a3,0x3c27a
4200d60a:	3c27a637          	lui	a2,0x3c27a
4200d60e:	3c27a5b7          	lui	a1,0x3c27a
4200d612:	3c27a737          	lui	a4,0x3c27a
4200d616:	af068693          	addi	a3,a3,-1296 # 3c279af0 <c5_mult_10>
4200d61a:	bf060613          	addi	a2,a2,-1040 # 3c279bf0 <c5_bias_10>
4200d61e:	cf058593          	addi	a1,a1,-784 # 3c279cf0 <c5_weights_10>
4200d622:	9f070713          	addi	a4,a4,-1552 # 3c2799f0 <c5_shift_10>
4200d626:	be8d                	j	4200d198 <resnet_execute+0xd80>
4200d628:	3c2706b7          	lui	a3,0x3c270
4200d62c:	3c270637          	lui	a2,0x3c270
4200d630:	3c2705b7          	lui	a1,0x3c270
4200d634:	3c270737          	lui	a4,0x3c270
4200d638:	a8868693          	addi	a3,a3,-1400 # 3c26fa88 <c4_mult_8>
4200d63c:	b8860613          	addi	a2,a2,-1144 # 3c26fb88 <c4_bias_8>
4200d640:	c8858593          	addi	a1,a1,-888 # 3c26fc88 <c4_weights_8>
4200d644:	98870713          	addi	a4,a4,-1656 # 3c26f988 <c4_shift_8>
4200d648:	881ff06f          	j	4200cec8 <resnet_execute+0xab0>
4200d64c:	3c2846b7          	lui	a3,0x3c284
4200d650:	3c284637          	lui	a2,0x3c284
4200d654:	3c2845b7          	lui	a1,0x3c284
4200d658:	3c283737          	lui	a4,0x3c283
4200d65c:	8f068693          	addi	a3,a3,-1808 # 3c2838f0 <c5_mult_8>
4200d660:	9f060613          	addi	a2,a2,-1552 # 3c2839f0 <c5_bias_8>
4200d664:	af058593          	addi	a1,a1,-1296 # 3c283af0 <c5_weights_8>
4200d668:	7f070713          	addi	a4,a4,2032 # 3c2837f0 <c5_shift_8>
4200d66c:	85dff06f          	j	4200cec8 <resnet_execute+0xab0>
4200d670:	3c2746b7          	lui	a3,0x3c274
4200d674:	3c274637          	lui	a2,0x3c274
4200d678:	3c2745b7          	lui	a1,0x3c274
4200d67c:	3c274737          	lui	a4,0x3c274
4200d680:	50868693          	addi	a3,a3,1288 # 3c274508 <c4_mult_6>
4200d684:	58860613          	addi	a2,a2,1416 # 3c274588 <c4_bias_6>
4200d688:	60858593          	addi	a1,a1,1544 # 3c274608 <c4_weights_6>
4200d68c:	48870713          	addi	a4,a4,1160 # 3c274488 <c4_shift_6>
4200d690:	e68ff06f          	j	4200ccf8 <resnet_execute+0x8e0>
4200d694:	3c1d56b7          	lui	a3,0x3c1d5
4200d698:	3c1d5637          	lui	a2,0x3c1d5
4200d69c:	3c1d55b7          	lui	a1,0x3c1d5
4200d6a0:	3c1d5737          	lui	a4,0x3c1d5
4200d6a4:	a3868693          	addi	a3,a3,-1480 # 3c1d4a38 <t_mult_8>
4200d6a8:	b3860613          	addi	a2,a2,-1224 # 3c1d4b38 <t_bias_8>
4200d6ac:	c3858593          	addi	a1,a1,-968 # 3c1d4c38 <t_weights_8>
4200d6b0:	93870713          	addi	a4,a4,-1736 # 3c1d4938 <t_shift_8>
4200d6b4:	815ff06f          	j	4200cec8 <resnet_execute+0xab0>
4200d6b8:	3c27a6b7          	lui	a3,0x3c27a
4200d6bc:	3c27a637          	lui	a2,0x3c27a
4200d6c0:	3c27a5b7          	lui	a1,0x3c27a
4200d6c4:	3c27a737          	lui	a4,0x3c27a
4200d6c8:	5f068693          	addi	a3,a3,1520 # 3c27a5f0 <c5_mult_9>
4200d6cc:	6f060613          	addi	a2,a2,1776 # 3c27a6f0 <c5_bias_9>
4200d6d0:	7f058593          	addi	a1,a1,2032 # 3c27a7f0 <c5_weights_9>
4200d6d4:	4f070713          	addi	a4,a4,1264 # 3c27a4f0 <c5_shift_9>
4200d6d8:	b0f9                	j	4200cfa6 <resnet_execute+0xb8e>
4200d6da:	3c1cb6b7          	lui	a3,0x3c1cb
4200d6de:	3c1cc637          	lui	a2,0x3c1cc
4200d6e2:	3c1cc5b7          	lui	a1,0x3c1cc
4200d6e6:	3c1cb737          	lui	a4,0x3c1cb
4200d6ea:	73868693          	addi	a3,a3,1848 # 3c1cb738 <t_mult_9>
4200d6ee:	83860613          	addi	a2,a2,-1992 # 3c1cb838 <t_bias_9>
4200d6f2:	93858593          	addi	a1,a1,-1736 # 3c1cb938 <t_weights_9>
4200d6f6:	63870713          	addi	a4,a4,1592 # 3c1cb638 <t_shift_9>
4200d6fa:	8adff06f          	j	4200cfa6 <resnet_execute+0xb8e>
4200d6fe:	3c2666b7          	lui	a3,0x3c266
4200d702:	3c267637          	lui	a2,0x3c267
4200d706:	3c2675b7          	lui	a1,0x3c267
4200d70a:	3c266737          	lui	a4,0x3c266
4200d70e:	78868693          	addi	a3,a3,1928 # 3c266788 <c4_mult_9>
4200d712:	88860613          	addi	a2,a2,-1912 # 3c266888 <c4_bias_9>
4200d716:	98858593          	addi	a1,a1,-1656 # 3c266988 <c4_weights_9>
4200d71a:	68870713          	addi	a4,a4,1672 # 3c266688 <c4_shift_9>
4200d71e:	889ff06f          	j	4200cfa6 <resnet_execute+0xb8e>
4200d722:	3c1cb6b7          	lui	a3,0x3c1cb
4200d726:	3c1cb637          	lui	a2,0x3c1cb
4200d72a:	3c1cb5b7          	lui	a1,0x3c1cb
4200d72e:	3c1cb737          	lui	a4,0x3c1cb
4200d732:	c3868693          	addi	a3,a3,-968 # 3c1cac38 <t_mult_10>
4200d736:	d3860613          	addi	a2,a2,-712 # 3c1cad38 <t_bias_10>
4200d73a:	e3858593          	addi	a1,a1,-456 # 3c1cae38 <t_weights_10>
4200d73e:	b3870713          	addi	a4,a4,-1224 # 3c1cab38 <t_shift_10>
4200d742:	bc99                	j	4200d198 <resnet_execute+0xd80>
4200d744:	3c2886b7          	lui	a3,0x3c288
4200d748:	3c288637          	lui	a2,0x3c288
4200d74c:	3c2885b7          	lui	a1,0x3c288
4200d750:	3c288737          	lui	a4,0x3c288
4200d754:	37068693          	addi	a3,a3,880 # 3c288370 <c5_mult_6>
4200d758:	3f060613          	addi	a2,a2,1008 # 3c2883f0 <c5_bias_6>
4200d75c:	47058593          	addi	a1,a1,1136 # 3c288470 <c5_weights_6>
4200d760:	2f070713          	addi	a4,a4,752 # 3c2882f0 <c5_shift_6>
4200d764:	d94ff06f          	j	4200ccf8 <resnet_execute+0x8e0>
4200d768:	3c1d96b7          	lui	a3,0x3c1d9
4200d76c:	3c1d9637          	lui	a2,0x3c1d9
4200d770:	3c1d95b7          	lui	a1,0x3c1d9
4200d774:	3c1d9737          	lui	a4,0x3c1d9
4200d778:	4b868693          	addi	a3,a3,1208 # 3c1d94b8 <t_mult_6>
4200d77c:	53860613          	addi	a2,a2,1336 # 3c1d9538 <t_bias_6>
4200d780:	5b858593          	addi	a1,a1,1464 # 3c1d95b8 <t_weights_6>
4200d784:	43870713          	addi	a4,a4,1080 # 3c1d9438 <t_shift_6>
4200d788:	d70ff06f          	j	4200ccf8 <resnet_execute+0x8e0>
4200d78c:	3c2756b7          	lui	a3,0x3c275
4200d790:	3c275637          	lui	a2,0x3c275
4200d794:	3c2755b7          	lui	a1,0x3c275
4200d798:	3c275737          	lui	a4,0x3c275
4200d79c:	88868693          	addi	a3,a3,-1912 # 3c274888 <c4_mult_5>
4200d7a0:	90860613          	addi	a2,a2,-1784 # 3c274908 <c4_bias_5>
4200d7a4:	98858593          	addi	a1,a1,-1656 # 3c274988 <c4_weights_5>
4200d7a8:	80870713          	addi	a4,a4,-2040 # 3c274808 <c4_shift_5>
4200d7ac:	b54ff06f          	j	4200cb00 <resnet_execute+0x6e8>
4200d7b0:	3c2886b7          	lui	a3,0x3c288
4200d7b4:	3c288637          	lui	a2,0x3c288
4200d7b8:	3c2885b7          	lui	a1,0x3c288
4200d7bc:	3c288737          	lui	a4,0x3c288
4200d7c0:	6f068693          	addi	a3,a3,1776 # 3c2886f0 <c5_mult_5>
4200d7c4:	77060613          	addi	a2,a2,1904 # 3c288770 <c5_bias_5>
4200d7c8:	7f058593          	addi	a1,a1,2032 # 3c2887f0 <c5_weights_5>
4200d7cc:	67070713          	addi	a4,a4,1648 # 3c288670 <c5_shift_5>
4200d7d0:	b30ff06f          	j	4200cb00 <resnet_execute+0x6e8>
4200d7d4:	3c28b6b7          	lui	a3,0x3c28b
4200d7d8:	3c28b637          	lui	a2,0x3c28b
4200d7dc:	3c28b5b7          	lui	a1,0x3c28b
4200d7e0:	3c28b737          	lui	a4,0x3c28b
4200d7e4:	c7068693          	addi	a3,a3,-912 # 3c28ac70 <c5_mult_4>
4200d7e8:	cf060613          	addi	a2,a2,-784 # 3c28acf0 <c5_bias_4>
4200d7ec:	d7058593          	addi	a1,a1,-656 # 3c28ad70 <c5_weights_4>
4200d7f0:	bf070713          	addi	a4,a4,-1040 # 3c28abf0 <c5_shift_4>
4200d7f4:	a2eff06f          	j	4200ca22 <resnet_execute+0x60a>
4200d7f8:	3c1dc6b7          	lui	a3,0x3c1dc
4200d7fc:	3c1dc637          	lui	a2,0x3c1dc
4200d800:	3c1dc5b7          	lui	a1,0x3c1dc
4200d804:	3c1dc737          	lui	a4,0x3c1dc
4200d808:	db868693          	addi	a3,a3,-584 # 3c1dbdb8 <t_mult_4>
4200d80c:	e3860613          	addi	a2,a2,-456 # 3c1dbe38 <t_bias_4>
4200d810:	eb858593          	addi	a1,a1,-328 # 3c1dbeb8 <t_weights_4>
4200d814:	d3870713          	addi	a4,a4,-712 # 3c1dbd38 <t_shift_4>
4200d818:	a0aff06f          	j	4200ca22 <resnet_execute+0x60a>
4200d81c:	3c2776b7          	lui	a3,0x3c277
4200d820:	3c277637          	lui	a2,0x3c277
4200d824:	3c2775b7          	lui	a1,0x3c277
4200d828:	3c277737          	lui	a4,0x3c277
4200d82c:	e0868693          	addi	a3,a3,-504 # 3c276e08 <c4_mult_4>
4200d830:	e8860613          	addi	a2,a2,-376 # 3c276e88 <c4_bias_4>
4200d834:	f0858593          	addi	a1,a1,-248 # 3c276f08 <c4_weights_4>
4200d838:	d8870713          	addi	a4,a4,-632 # 3c276d88 <c4_shift_4>
4200d83c:	9e6ff06f          	j	4200ca22 <resnet_execute+0x60a>
4200d840:	3c1da6b7          	lui	a3,0x3c1da
4200d844:	3c1da637          	lui	a2,0x3c1da
4200d848:	3c1da5b7          	lui	a1,0x3c1da
4200d84c:	3c1d9737          	lui	a4,0x3c1d9
4200d850:	83868693          	addi	a3,a3,-1992 # 3c1d9838 <t_mult_5>
4200d854:	8b860613          	addi	a2,a2,-1864 # 3c1d98b8 <t_bias_5>
4200d858:	93858593          	addi	a1,a1,-1736 # 3c1d9938 <t_weights_5>
4200d85c:	7b870713          	addi	a4,a4,1976 # 3c1d97b8 <t_shift_5>
4200d860:	aa0ff06f          	j	4200cb00 <resnet_execute+0x6e8>
4200d864:	3c2786b7          	lui	a3,0x3c278
4200d868:	3c278637          	lui	a2,0x3c278
4200d86c:	3c2785b7          	lui	a1,0x3c278
4200d870:	3c278737          	lui	a4,0x3c278
4200d874:	14868693          	addi	a3,a3,328 # 3c278148 <c4_mult_2>
4200d878:	18860613          	addi	a2,a2,392 # 3c278188 <c4_bias_2>
4200d87c:	1c858593          	addi	a1,a1,456 # 3c2781c8 <c4_weights_2>
4200d880:	10870713          	addi	a4,a4,264 # 3c278108 <c4_shift_2>
4200d884:	e69fe06f          	j	4200c6ec <resnet_execute+0x2d4>
4200d888:	3c28c6b7          	lui	a3,0x3c28c
4200d88c:	3c28c637          	lui	a2,0x3c28c
4200d890:	3c28c5b7          	lui	a1,0x3c28c
4200d894:	3c28c737          	lui	a4,0x3c28c
4200d898:	fb068693          	addi	a3,a3,-80 # 3c28bfb0 <c5_mult_2>
4200d89c:	ff060613          	addi	a2,a2,-16 # 3c28bff0 <c5_bias_2>
4200d8a0:	03058593          	addi	a1,a1,48 # 3c28c030 <c5_weights_2>
4200d8a4:	f7070713          	addi	a4,a4,-144 # 3c28bf70 <c5_shift_2>
4200d8a8:	e45fe06f          	j	4200c6ec <resnet_execute+0x2d4>
4200d8ac:	3c2796b7          	lui	a3,0x3c279
4200d8b0:	3c279637          	lui	a2,0x3c279
4200d8b4:	3c2795b7          	lui	a1,0x3c279
4200d8b8:	3c279737          	lui	a4,0x3c279
4200d8bc:	3fc8fb37          	lui	s6,0x3fc8f
4200d8c0:	4c868693          	addi	a3,a3,1224 # 3c2794c8 <c4_mult_0>
4200d8c4:	50860613          	addi	a2,a2,1288 # 3c279508 <c4_bias_0>
4200d8c8:	54858593          	addi	a1,a1,1352 # 3c279548 <c4_weights_0>
4200d8cc:	48870713          	addi	a4,a4,1160 # 3c279488 <c4_shift_0>
4200d8d0:	accb0993          	addi	s3,s6,-1332 # 3fc8eacc <padded>
4200d8d4:	c3bfe06f          	j	4200c50e <resnet_execute+0xf6>
4200d8d8:	3c1dd6b7          	lui	a3,0x3c1dd
4200d8dc:	3c1dd637          	lui	a2,0x3c1dd
4200d8e0:	3c1dd5b7          	lui	a1,0x3c1dd
4200d8e4:	3c1dd737          	lui	a4,0x3c1dd
4200d8e8:	0f868693          	addi	a3,a3,248 # 3c1dd0f8 <t_mult_2>
4200d8ec:	13860613          	addi	a2,a2,312 # 3c1dd138 <t_bias_2>
4200d8f0:	17858593          	addi	a1,a1,376 # 3c1dd178 <t_weights_2>
4200d8f4:	0b870713          	addi	a4,a4,184 # 3c1dd0b8 <t_shift_2>
4200d8f8:	df5fe06f          	j	4200c6ec <resnet_execute+0x2d4>
4200d8fc:	3c28d6b7          	lui	a3,0x3c28d
4200d900:	3c28d637          	lui	a2,0x3c28d
4200d904:	3c28d5b7          	lui	a1,0x3c28d
4200d908:	3c28d737          	lui	a4,0x3c28d
4200d90c:	3fc8fb37          	lui	s6,0x3fc8f
4200d910:	33068693          	addi	a3,a3,816 # 3c28d330 <c5_mult_0>
4200d914:	37060613          	addi	a2,a2,880 # 3c28d370 <c5_bias_0>
4200d918:	3b058593          	addi	a1,a1,944 # 3c28d3b0 <c5_weights_0>
4200d91c:	2f070713          	addi	a4,a4,752 # 3c28d2f0 <c5_shift_0>
4200d920:	accb0993          	addi	s3,s6,-1332 # 3fc8eacc <padded>
4200d924:	bebfe06f          	j	4200c50e <resnet_execute+0xf6>
4200d928:	3c1de6b7          	lui	a3,0x3c1de
4200d92c:	3c1de637          	lui	a2,0x3c1de
4200d930:	3c1de5b7          	lui	a1,0x3c1de
4200d934:	3c1de737          	lui	a4,0x3c1de
4200d938:	3fc8fb37          	lui	s6,0x3fc8f
4200d93c:	47868693          	addi	a3,a3,1144 # 3c1de478 <t_mult_0>
4200d940:	4b860613          	addi	a2,a2,1208 # 3c1de4b8 <t_bias_0>
4200d944:	4f858593          	addi	a1,a1,1272 # 3c1de4f8 <t_weights_0>
4200d948:	43870713          	addi	a4,a4,1080 # 3c1de438 <t_shift_0>
4200d94c:	accb0993          	addi	s3,s6,-1332 # 3fc8eacc <padded>
4200d950:	bbffe06f          	j	4200c50e <resnet_execute+0xf6>
4200d954:	3c2796b7          	lui	a3,0x3c279
4200d958:	3c279637          	lui	a2,0x3c279
4200d95c:	3c2795b7          	lui	a1,0x3c279
4200d960:	3c279737          	lui	a4,0x3c279
4200d964:	b0868693          	addi	a3,a3,-1272 # 3c278b08 <c4_mult_1>
4200d968:	b4860613          	addi	a2,a2,-1208 # 3c278b48 <c4_bias_1>
4200d96c:	b8858593          	addi	a1,a1,-1144 # 3c278b88 <c4_weights_1>
4200d970:	ac870713          	addi	a4,a4,-1336 # 3c278ac8 <c4_shift_1>
4200d974:	c69fe06f          	j	4200c5dc <resnet_execute+0x1c4>
4200d978:	3c1de6b7          	lui	a3,0x3c1de
4200d97c:	3c1de637          	lui	a2,0x3c1de
4200d980:	3c1de5b7          	lui	a1,0x3c1de
4200d984:	3c1de737          	lui	a4,0x3c1de
4200d988:	ab868693          	addi	a3,a3,-1352 # 3c1ddab8 <t_mult_1>
4200d98c:	af860613          	addi	a2,a2,-1288 # 3c1ddaf8 <t_bias_1>
4200d990:	b3858593          	addi	a1,a1,-1224 # 3c1ddb38 <t_weights_1>
4200d994:	a7870713          	addi	a4,a4,-1416 # 3c1dda78 <t_shift_1>
4200d998:	c45fe06f          	j	4200c5dc <resnet_execute+0x1c4>
