
signal_test:     file format elf64-littleriscv


Disassembly of section .text:

0000000000010120 <exit>:
   10120:	fe010113          	addi	sp,sp,-32
   10124:	00a13423          	sd	a0,8(sp)
   10128:	00113c23          	sd	ra,24(sp)
   1012c:	530000ef          	jal	ra,1065c <__funcs_on_exit>
   10130:	530000ef          	jal	ra,10660 <__libc_exit_fini>
   10134:	0f9030ef          	jal	ra,13a2c <__stdio_exit>
   10138:	00813503          	ld	a0,8(sp)
   1013c:	08d020ef          	jal	ra,129c8 <_Exit>

0000000000010140 <_start>:
   10140:	00008197          	auipc	gp,0x8
   10144:	6c018193          	addi	gp,gp,1728 # 18800 <__global_pointer$>
   10148:	00010513          	mv	a0,sp
   1014c:	00000593          	li	a1,0
   10150:	ff017113          	andi	sp,sp,-16
   10154:	0040006f          	j	10158 <_start_c>

0000000000010158 <_start_c>:
   10158:	00052583          	lw	a1,0(a0)
   1015c:	00010737          	lui	a4,0x10
   10160:	000106b7          	lui	a3,0x10
   10164:	00850613          	addi	a2,a0,8
   10168:	00010537          	lui	a0,0x10
   1016c:	00000793          	li	a5,0
   10170:	65c70713          	addi	a4,a4,1628 # 1065c <__funcs_on_exit>
   10174:	3b068693          	addi	a3,a3,944 # 103b0 <_init>
   10178:	34050513          	addi	a0,a0,832 # 10340 <main>
   1017c:	4880006f          	j	10604 <__libc_start_main>

0000000000010180 <deregister_tm_clones>:
   10180:	8e818713          	addi	a4,gp,-1816 # 180e8 <__TMC_END__>
   10184:	8e818793          	addi	a5,gp,-1816 # 180e8 <__TMC_END__>
   10188:	00e78a63          	beq	a5,a4,1019c <deregister_tm_clones+0x1c>
   1018c:	00000793          	li	a5,0
   10190:	00078663          	beqz	a5,1019c <deregister_tm_clones+0x1c>
   10194:	8e818513          	addi	a0,gp,-1816 # 180e8 <__TMC_END__>
   10198:	00078067          	jr	a5
   1019c:	00008067          	ret

00000000000101a0 <register_tm_clones>:
   101a0:	8e818593          	addi	a1,gp,-1816 # 180e8 <__TMC_END__>
   101a4:	8e818793          	addi	a5,gp,-1816 # 180e8 <__TMC_END__>
   101a8:	40f585b3          	sub	a1,a1,a5
   101ac:	4035d593          	srai	a1,a1,0x3
   101b0:	00200793          	li	a5,2
   101b4:	02f5c5b3          	div	a1,a1,a5
   101b8:	00058a63          	beqz	a1,101cc <register_tm_clones+0x2c>
   101bc:	00000793          	li	a5,0
   101c0:	00078663          	beqz	a5,101cc <register_tm_clones+0x2c>
   101c4:	8e818513          	addi	a0,gp,-1816 # 180e8 <__TMC_END__>
   101c8:	00078067          	jr	a5
   101cc:	00008067          	ret

00000000000101d0 <__do_global_dtors_aux>:
   101d0:	9c01c703          	lbu	a4,-1600(gp) # 181c0 <completed.1>
   101d4:	04071463          	bnez	a4,1021c <__do_global_dtors_aux+0x4c>
   101d8:	ff010113          	addi	sp,sp,-16
   101dc:	00813023          	sd	s0,0(sp)
   101e0:	00113423          	sd	ra,8(sp)
   101e4:	00078413          	mv	s0,a5
   101e8:	f99ff0ef          	jal	ra,10180 <deregister_tm_clones>
   101ec:	00000793          	li	a5,0
   101f0:	00078a63          	beqz	a5,10204 <__do_global_dtors_aux+0x34>
   101f4:	00017537          	lui	a0,0x17
   101f8:	bd050513          	addi	a0,a0,-1072 # 16bd0 <__FRAME_END__>
   101fc:	00000097          	auipc	ra,0x0
   10200:	000000e7          	jalr	zero # 0 <exit-0x10120>
   10204:	00100793          	li	a5,1
   10208:	00813083          	ld	ra,8(sp)
   1020c:	9cf18023          	sb	a5,-1600(gp) # 181c0 <completed.1>
   10210:	00013403          	ld	s0,0(sp)
   10214:	01010113          	addi	sp,sp,16
   10218:	00008067          	ret
   1021c:	00008067          	ret

0000000000010220 <frame_dummy>:
   10220:	00000793          	li	a5,0
   10224:	02078463          	beqz	a5,1024c <frame_dummy+0x2c>
   10228:	00017537          	lui	a0,0x17
   1022c:	ff010113          	addi	sp,sp,-16
   10230:	9c818593          	addi	a1,gp,-1592 # 181c8 <object.0>
   10234:	bd050513          	addi	a0,a0,-1072 # 16bd0 <__FRAME_END__>
   10238:	00113423          	sd	ra,8(sp)
   1023c:	00000097          	auipc	ra,0x0
   10240:	000000e7          	jalr	zero # 0 <exit-0x10120>
   10244:	00813083          	ld	ra,8(sp)
   10248:	01010113          	addi	sp,sp,16
   1024c:	f55ff06f          	j	101a0 <register_tm_clones>

0000000000010250 <func_1>:
   10250:	fe010113          	addi	sp,sp,-32
   10254:	00113c23          	sd	ra,24(sp)
   10258:	00813823          	sd	s0,16(sp)
   1025c:	00913423          	sd	s1,8(sp)
   10260:	9541a783          	lw	a5,-1708(gp) # 18154 <cnt1>
   10264:	0017879b          	addiw	a5,a5,1
   10268:	0007841b          	sext.w	s0,a5
   1026c:	94f1aa23          	sw	a5,-1708(gp) # 18154 <cnt1>
   10270:	00300793          	li	a5,3
   10274:	0087dc63          	bge	a5,s0,1028c <func_1+0x3c>
   10278:	01813083          	ld	ra,24(sp)
   1027c:	01013403          	ld	s0,16(sp)
   10280:	00813483          	ld	s1,8(sp)
   10284:	02010113          	addi	sp,sp,32
   10288:	00008067          	ret
   1028c:	00040593          	mv	a1,s0
   10290:	00016537          	lui	a0,0x16
   10294:	ec050513          	addi	a0,a0,-320 # 15ec0 <__clzdi2+0x40>
   10298:	49c000ef          	jal	ra,10734 <printf>
   1029c:	00200593          	li	a1,2
   102a0:	9581a503          	lw	a0,-1704(gp) # 18158 <pid>
   102a4:	404000ef          	jal	ra,106a8 <kill>
   102a8:	01200593          	li	a1,18
   102ac:	9581a503          	lw	a0,-1704(gp) # 18158 <pid>
   102b0:	3f8000ef          	jal	ra,106a8 <kill>
   102b4:	00040593          	mv	a1,s0
   102b8:	00016537          	lui	a0,0x16
   102bc:	ec850513          	addi	a0,a0,-312 # 15ec8 <__clzdi2+0x48>
   102c0:	474000ef          	jal	ra,10734 <printf>
   102c4:	fb5ff06f          	j	10278 <func_1+0x28>

00000000000102c8 <func_2>:
   102c8:	fe010113          	addi	sp,sp,-32
   102cc:	00113c23          	sd	ra,24(sp)
   102d0:	00813823          	sd	s0,16(sp)
   102d4:	00913423          	sd	s1,8(sp)
   102d8:	9501a783          	lw	a5,-1712(gp) # 18150 <_edata>
   102dc:	0017879b          	addiw	a5,a5,1
   102e0:	0007841b          	sext.w	s0,a5
   102e4:	94f1a823          	sw	a5,-1712(gp) # 18150 <_edata>
   102e8:	00300793          	li	a5,3
   102ec:	0087dc63          	bge	a5,s0,10304 <func_2+0x3c>
   102f0:	01813083          	ld	ra,24(sp)
   102f4:	01013403          	ld	s0,16(sp)
   102f8:	00813483          	ld	s1,8(sp)
   102fc:	02010113          	addi	sp,sp,32
   10300:	00008067          	ret
   10304:	00040593          	mv	a1,s0
   10308:	00016537          	lui	a0,0x16
   1030c:	ed050513          	addi	a0,a0,-304 # 15ed0 <__clzdi2+0x50>
   10310:	424000ef          	jal	ra,10734 <printf>
   10314:	00200593          	li	a1,2
   10318:	9581a503          	lw	a0,-1704(gp) # 18158 <pid>
   1031c:	38c000ef          	jal	ra,106a8 <kill>
   10320:	01200593          	li	a1,18
   10324:	9581a503          	lw	a0,-1704(gp) # 18158 <pid>
   10328:	380000ef          	jal	ra,106a8 <kill>
   1032c:	00040593          	mv	a1,s0
   10330:	00016537          	lui	a0,0x16
   10334:	ed850513          	addi	a0,a0,-296 # 15ed8 <__clzdi2+0x58>
   10338:	3fc000ef          	jal	ra,10734 <printf>
   1033c:	fb5ff06f          	j	102f0 <func_2+0x28>

0000000000010340 <main>:
   10340:	ff010113          	addi	sp,sp,-16
   10344:	00113423          	sd	ra,8(sp)
   10348:	00813023          	sd	s0,0(sp)
   1034c:	000105b7          	lui	a1,0x10
   10350:	25058593          	addi	a1,a1,592 # 10250 <func_1>
   10354:	00200513          	li	a0,2
   10358:	374000ef          	jal	ra,106cc <signal>
   1035c:	000105b7          	lui	a1,0x10
   10360:	2c858593          	addi	a1,a1,712 # 102c8 <func_2>
   10364:	01200513          	li	a0,18
   10368:	364000ef          	jal	ra,106cc <signal>
   1036c:	2ec020ef          	jal	ra,12658 <getpid>
   10370:	00050593          	mv	a1,a0
   10374:	94a1ac23          	sw	a0,-1704(gp) # 18158 <pid>
   10378:	00016537          	lui	a0,0x16
   1037c:	ee050513          	addi	a0,a0,-288 # 15ee0 <__clzdi2+0x60>
   10380:	3b4000ef          	jal	ra,10734 <printf>
   10384:	00200593          	li	a1,2
   10388:	9581a503          	lw	a0,-1704(gp) # 18158 <pid>
   1038c:	31c000ef          	jal	ra,106a8 <kill>
   10390:	01200593          	li	a1,18
   10394:	9581a503          	lw	a0,-1704(gp) # 18158 <pid>
   10398:	310000ef          	jal	ra,106a8 <kill>
   1039c:	00000513          	li	a0,0
   103a0:	00813083          	ld	ra,8(sp)
   103a4:	00013403          	ld	s0,0(sp)
   103a8:	01010113          	addi	sp,sp,16
   103ac:	00008067          	ret

00000000000103b0 <_init>:
   103b0:	00008067          	ret

00000000000103b4 <__init_ssp>:
   103b4:	00008067          	ret

00000000000103b8 <__init_libc>:
   103b8:	e8010113          	addi	sp,sp,-384
   103bc:	17213023          	sd	s2,352(sp)
   103c0:	03010913          	addi	s2,sp,48
   103c4:	16813823          	sd	s0,368(sp)
   103c8:	16913423          	sd	s1,360(sp)
   103cc:	00058413          	mv	s0,a1
   103d0:	00050493          	mv	s1,a0
   103d4:	13000613          	li	a2,304
   103d8:	00000593          	li	a1,0
   103dc:	00090513          	mv	a0,s2
   103e0:	16113c23          	sd	ra,376(sp)
   103e4:	138020ef          	jal	ra,1251c <memset>
   103e8:	9891b023          	sd	s1,-1664(gp) # 18180 <__environ>
   103ec:	00000513          	li	a0,0
   103f0:	00351793          	slli	a5,a0,0x3
   103f4:	00f487b3          	add	a5,s1,a5
   103f8:	0007b783          	ld	a5,0(a5)
   103fc:	00150513          	addi	a0,a0,1
   10400:	fe0798e3          	bnez	a5,103f0 <__init_libc+0x38>
   10404:	00351513          	slli	a0,a0,0x3
   10408:	00a48533          	add	a0,s1,a0
   1040c:	9f818793          	addi	a5,gp,-1544 # 181f8 <__libc>
   10410:	00a7b823          	sd	a0,16(a5)
   10414:	9f818493          	addi	s1,gp,-1544 # 181f8 <__libc>
   10418:	02500713          	li	a4,37
   1041c:	00053783          	ld	a5,0(a0)
   10420:	0e079e63          	bnez	a5,1051c <__init_libc+0x164>
   10424:	0b013703          	ld	a4,176(sp)
   10428:	96e1b423          	sd	a4,-1688(gp) # 18168 <__hwcap>
   1042c:	13013783          	ld	a5,304(sp)
   10430:	00078463          	beqz	a5,10438 <__init_libc+0x80>
   10434:	96f1b023          	sd	a5,-1696(gp) # 18160 <__sysinfo>
   10438:	06013783          	ld	a5,96(sp)
   1043c:	02f4bc23          	sd	a5,56(s1)
   10440:	00041a63          	bnez	s0,10454 <__init_libc+0x9c>
   10444:	12813403          	ld	s0,296(sp)
   10448:	00041663          	bnez	s0,10454 <__init_libc+0x9c>
   1044c:	00016437          	lui	s0,0x16
   10450:	ee840413          	addi	s0,s0,-280 # 15ee8 <__clzdi2+0x68>
   10454:	9681bc23          	sd	s0,-1672(gp) # 18178 <__progname_full>
   10458:	9681b823          	sd	s0,-1680(gp) # 18170 <__progname>
   1045c:	02f00693          	li	a3,47
   10460:	00140413          	addi	s0,s0,1
   10464:	fff44703          	lbu	a4,-1(s0)
   10468:	0c071a63          	bnez	a4,1053c <__init_libc+0x184>
   1046c:	00090513          	mv	a0,s2
   10470:	358020ef          	jal	ra,127c8 <__init_tls>
   10474:	0f813503          	ld	a0,248(sp)
   10478:	f3dff0ef          	jal	ra,103b4 <__init_ssp>
   1047c:	08813703          	ld	a4,136(sp)
   10480:	09013783          	ld	a5,144(sp)
   10484:	00f71c63          	bne	a4,a5,1049c <__init_libc+0xe4>
   10488:	09813703          	ld	a4,152(sp)
   1048c:	0a013783          	ld	a5,160(sp)
   10490:	00f71663          	bne	a4,a5,1049c <__init_libc+0xe4>
   10494:	0e813783          	ld	a5,232(sp)
   10498:	0c078263          	beqz	a5,1055c <__init_libc+0x1a4>
   1049c:	02013023          	sd	zero,32(sp)
   104a0:	00100793          	li	a5,1
   104a4:	02013423          	sd	zero,40(sp)
   104a8:	02f12023          	sw	a5,32(sp)
   104ac:	00200793          	li	a5,2
   104b0:	00013c23          	sd	zero,24(sp)
   104b4:	02f12423          	sw	a5,40(sp)
   104b8:	00013423          	sd	zero,8(sp)
   104bc:	00013823          	sd	zero,16(sp)
   104c0:	04900893          	li	a7,73
   104c4:	01810513          	addi	a0,sp,24
   104c8:	00300593          	li	a1,3
   104cc:	00810613          	addi	a2,sp,8
   104d0:	00000693          	li	a3,0
   104d4:	00800713          	li	a4,8
   104d8:	00000073          	ecall
   104dc:	0005051b          	sext.w	a0,a0
   104e0:	01810793          	addi	a5,sp,24
   104e4:	02054863          	bltz	a0,10514 <__init_libc+0x15c>
   104e8:	00008637          	lui	a2,0x8
   104ec:	000166b7          	lui	a3,0x16
   104f0:	00260613          	addi	a2,a2,2 # 8002 <exit-0x811e>
   104f4:	0067d703          	lhu	a4,6(a5)
   104f8:	02077713          	andi	a4,a4,32
   104fc:	04070863          	beqz	a4,1054c <__init_libc+0x194>
   10500:	03800893          	li	a7,56
   10504:	f9c00513          	li	a0,-100
   10508:	ef068593          	addi	a1,a3,-272 # 15ef0 <__clzdi2+0x70>
   1050c:	00000073          	ecall
   10510:	02055e63          	bgez	a0,1054c <__init_libc+0x194>
   10514:	00000023          	sb	zero,0(zero) # 0 <exit-0x10120>
   10518:	00100073          	ebreak
   1051c:	00f76c63          	bltu	a4,a5,10534 <__init_libc+0x17c>
   10520:	16010693          	addi	a3,sp,352
   10524:	00379793          	slli	a5,a5,0x3
   10528:	00f687b3          	add	a5,a3,a5
   1052c:	00853683          	ld	a3,8(a0)
   10530:	ecd7b823          	sd	a3,-304(a5)
   10534:	01050513          	addi	a0,a0,16
   10538:	ee5ff06f          	j	1041c <__init_libc+0x64>
   1053c:	00d71463          	bne	a4,a3,10544 <__init_libc+0x18c>
   10540:	9681b823          	sd	s0,-1680(gp) # 18170 <__progname>
   10544:	00140413          	addi	s0,s0,1
   10548:	f1dff06f          	j	10464 <__init_libc+0xac>
   1054c:	00878793          	addi	a5,a5,8
   10550:	faf912e3          	bne	s2,a5,104f4 <__init_libc+0x13c>
   10554:	00100793          	li	a5,1
   10558:	00f4a423          	sw	a5,8(s1)
   1055c:	17813083          	ld	ra,376(sp)
   10560:	17013403          	ld	s0,368(sp)
   10564:	16813483          	ld	s1,360(sp)
   10568:	16013903          	ld	s2,352(sp)
   1056c:	18010113          	addi	sp,sp,384
   10570:	00008067          	ret

0000000000010574 <__libc_start_init>:
   10574:	fe010113          	addi	sp,sp,-32
   10578:	00813823          	sd	s0,16(sp)
   1057c:	00913423          	sd	s1,8(sp)
   10580:	00018437          	lui	s0,0x18
   10584:	000184b7          	lui	s1,0x18
   10588:	00113c23          	sd	ra,24(sp)
   1058c:	ff040413          	addi	s0,s0,-16 # 17ff0 <__frame_dummy_init_array_entry>
   10590:	e21ff0ef          	jal	ra,103b0 <_init>
   10594:	ff848493          	addi	s1,s1,-8 # 17ff8 <__init_array_end>
   10598:	00946c63          	bltu	s0,s1,105b0 <__libc_start_init+0x3c>
   1059c:	01813083          	ld	ra,24(sp)
   105a0:	01013403          	ld	s0,16(sp)
   105a4:	00813483          	ld	s1,8(sp)
   105a8:	02010113          	addi	sp,sp,32
   105ac:	00008067          	ret
   105b0:	00043783          	ld	a5,0(s0)
   105b4:	00840413          	addi	s0,s0,8
   105b8:	000780e7          	jalr	a5
   105bc:	fddff06f          	j	10598 <__libc_start_init+0x24>

00000000000105c0 <libc_start_main_stage2>:
   105c0:	fd010113          	addi	sp,sp,-48
   105c4:	02813023          	sd	s0,32(sp)
   105c8:	00158413          	addi	s0,a1,1
   105cc:	00341413          	slli	s0,s0,0x3
   105d0:	02113423          	sd	ra,40(sp)
   105d4:	00913c23          	sd	s1,24(sp)
   105d8:	01213823          	sd	s2,16(sp)
   105dc:	00058493          	mv	s1,a1
   105e0:	00050913          	mv	s2,a0
   105e4:	00860433          	add	s0,a2,s0
   105e8:	00c13423          	sd	a2,8(sp)
   105ec:	f89ff0ef          	jal	ra,10574 <__libc_start_init>
   105f0:	00813583          	ld	a1,8(sp)
   105f4:	00040613          	mv	a2,s0
   105f8:	00048513          	mv	a0,s1
   105fc:	000900e7          	jalr	s2
   10600:	b21ff0ef          	jal	ra,10120 <exit>

0000000000010604 <__libc_start_main>:
   10604:	fd010113          	addi	sp,sp,-48
   10608:	02813023          	sd	s0,32(sp)
   1060c:	00913c23          	sd	s1,24(sp)
   10610:	00058413          	mv	s0,a1
   10614:	00050493          	mv	s1,a0
   10618:	00158513          	addi	a0,a1,1
   1061c:	00063583          	ld	a1,0(a2)
   10620:	00351513          	slli	a0,a0,0x3
   10624:	00a60533          	add	a0,a2,a0
   10628:	02113423          	sd	ra,40(sp)
   1062c:	00c13423          	sd	a2,8(sp)
   10630:	d89ff0ef          	jal	ra,103b8 <__init_libc>
   10634:	000107b7          	lui	a5,0x10
   10638:	5c078793          	addi	a5,a5,1472 # 105c0 <libc_start_main_stage2>
   1063c:	00040593          	mv	a1,s0
   10640:	02013403          	ld	s0,32(sp)
   10644:	00813603          	ld	a2,8(sp)
   10648:	02813083          	ld	ra,40(sp)
   1064c:	00048513          	mv	a0,s1
   10650:	01813483          	ld	s1,24(sp)
   10654:	03010113          	addi	sp,sp,48
   10658:	00078067          	jr	a5

000000000001065c <__funcs_on_exit>:
   1065c:	00008067          	ret

0000000000010660 <__libc_exit_fini>:
   10660:	fe010113          	addi	sp,sp,-32
   10664:	00813823          	sd	s0,16(sp)
   10668:	00913423          	sd	s1,8(sp)
   1066c:	00018437          	lui	s0,0x18
   10670:	000184b7          	lui	s1,0x18
   10674:	00113c23          	sd	ra,24(sp)
   10678:	00040413          	mv	s0,s0
   1067c:	ff848493          	addi	s1,s1,-8 # 17ff8 <__init_array_end>
   10680:	0084ec63          	bltu	s1,s0,10698 <__libc_exit_fini+0x38>
   10684:	01013403          	ld	s0,16(sp)
   10688:	01813083          	ld	ra,24(sp)
   1068c:	00813483          	ld	s1,8(sp)
   10690:	02010113          	addi	sp,sp,32
   10694:	fc9ff06f          	j	1065c <__funcs_on_exit>
   10698:	ff843783          	ld	a5,-8(s0) # 17ff8 <__init_array_end>
   1069c:	ff840413          	addi	s0,s0,-8
   106a0:	000780e7          	jalr	a5
   106a4:	fddff06f          	j	10680 <__libc_exit_fini+0x20>

00000000000106a8 <kill>:
   106a8:	ff010113          	addi	sp,sp,-16
   106ac:	00113423          	sd	ra,8(sp)
   106b0:	08100893          	li	a7,129
   106b4:	00000073          	ecall
   106b8:	328020ef          	jal	ra,129e0 <__syscall_ret>
   106bc:	00813083          	ld	ra,8(sp)
   106c0:	0005051b          	sext.w	a0,a0
   106c4:	01010113          	addi	sp,sp,16
   106c8:	00008067          	ret

00000000000106cc <signal>:
   106cc:	eb010113          	addi	sp,sp,-336
   106d0:	14813023          	sd	s0,320(sp)
   106d4:	12913c23          	sd	s1,312(sp)
   106d8:	00050413          	mv	s0,a0
   106dc:	00058493          	mv	s1,a1
   106e0:	09000613          	li	a2,144
   106e4:	00000593          	li	a1,0
   106e8:	0a010513          	addi	a0,sp,160
   106ec:	14113423          	sd	ra,328(sp)
   106f0:	62d010ef          	jal	ra,1251c <memset>
   106f4:	100007b7          	lui	a5,0x10000
   106f8:	00010613          	mv	a2,sp
   106fc:	09810593          	addi	a1,sp,152
   10700:	00040513          	mv	a0,s0
   10704:	08913c23          	sd	s1,152(sp)
   10708:	12f12023          	sw	a5,288(sp)
   1070c:	690020ef          	jal	ra,12d9c <__sigaction>
   10710:	00054e63          	bltz	a0,1072c <signal+0x60>
   10714:	00013503          	ld	a0,0(sp)
   10718:	14813083          	ld	ra,328(sp)
   1071c:	14013403          	ld	s0,320(sp)
   10720:	13813483          	ld	s1,312(sp)
   10724:	15010113          	addi	sp,sp,336
   10728:	00008067          	ret
   1072c:	fff00513          	li	a0,-1
   10730:	fe9ff06f          	j	10718 <signal+0x4c>

0000000000010734 <printf>:
   10734:	fa010113          	addi	sp,sp,-96
   10738:	02b13423          	sd	a1,40(sp)
   1073c:	00050593          	mv	a1,a0
   10740:	00018537          	lui	a0,0x18
   10744:	02c13823          	sd	a2,48(sp)
   10748:	00050513          	mv	a0,a0
   1074c:	02810613          	addi	a2,sp,40
   10750:	00113c23          	sd	ra,24(sp)
   10754:	02d13c23          	sd	a3,56(sp)
   10758:	04e13023          	sd	a4,64(sp)
   1075c:	04f13423          	sd	a5,72(sp)
   10760:	05013823          	sd	a6,80(sp)
   10764:	05113c23          	sd	a7,88(sp)
   10768:	00c13423          	sd	a2,8(sp)
   1076c:	439010ef          	jal	ra,123a4 <vfprintf>
   10770:	01813083          	ld	ra,24(sp)
   10774:	06010113          	addi	sp,sp,96
   10778:	00008067          	ret

000000000001077c <pop_arg>:
   1077c:	ff75859b          	addiw	a1,a1,-9
   10780:	0005871b          	sext.w	a4,a1
   10784:	01100793          	li	a5,17
   10788:	0ee7ea63          	bltu	a5,a4,1087c <pop_arg+0x100>
   1078c:	02059593          	slli	a1,a1,0x20
   10790:	000167b7          	lui	a5,0x16
   10794:	efc78793          	addi	a5,a5,-260 # 15efc <__clzdi2+0x7c>
   10798:	01e5d593          	srli	a1,a1,0x1e
   1079c:	00f585b3          	add	a1,a1,a5
   107a0:	0005a703          	lw	a4,0(a1)
   107a4:	ff010113          	addi	sp,sp,-16
   107a8:	00813023          	sd	s0,0(sp)
   107ac:	00063783          	ld	a5,0(a2)
   107b0:	00113423          	sd	ra,8(sp)
   107b4:	00050413          	mv	s0,a0
   107b8:	00070067          	jr	a4
   107bc:	00878713          	addi	a4,a5,8
   107c0:	0007a783          	lw	a5,0(a5)
   107c4:	00e63023          	sd	a4,0(a2)
   107c8:	00f43023          	sd	a5,0(s0)
   107cc:	00813083          	ld	ra,8(sp)
   107d0:	00013403          	ld	s0,0(sp)
   107d4:	01010113          	addi	sp,sp,16
   107d8:	00008067          	ret
   107dc:	00878713          	addi	a4,a5,8
   107e0:	00e63023          	sd	a4,0(a2)
   107e4:	0007e783          	lwu	a5,0(a5)
   107e8:	fe1ff06f          	j	107c8 <pop_arg+0x4c>
   107ec:	00878713          	addi	a4,a5,8
   107f0:	00e63023          	sd	a4,0(a2)
   107f4:	00079783          	lh	a5,0(a5)
   107f8:	fd1ff06f          	j	107c8 <pop_arg+0x4c>
   107fc:	00878713          	addi	a4,a5,8
   10800:	00e63023          	sd	a4,0(a2)
   10804:	0007d783          	lhu	a5,0(a5)
   10808:	fc1ff06f          	j	107c8 <pop_arg+0x4c>
   1080c:	00878713          	addi	a4,a5,8
   10810:	00e63023          	sd	a4,0(a2)
   10814:	00078783          	lb	a5,0(a5)
   10818:	fb1ff06f          	j	107c8 <pop_arg+0x4c>
   1081c:	00878713          	addi	a4,a5,8
   10820:	00e63023          	sd	a4,0(a2)
   10824:	0007c783          	lbu	a5,0(a5)
   10828:	fa1ff06f          	j	107c8 <pop_arg+0x4c>
   1082c:	00878713          	addi	a4,a5,8
   10830:	00e63023          	sd	a4,0(a2)
   10834:	0007b783          	ld	a5,0(a5)
   10838:	f91ff06f          	j	107c8 <pop_arg+0x4c>
   1083c:	0007b503          	ld	a0,0(a5)
   10840:	00878713          	addi	a4,a5,8
   10844:	00e63023          	sd	a4,0(a2)
   10848:	51c050ef          	jal	ra,15d64 <__extenddftf2>
   1084c:	00a43023          	sd	a0,0(s0)
   10850:	00b43423          	sd	a1,8(s0)
   10854:	f79ff06f          	j	107cc <pop_arg+0x50>
   10858:	00f78793          	addi	a5,a5,15
   1085c:	ff07f793          	andi	a5,a5,-16
   10860:	01078713          	addi	a4,a5,16
   10864:	00e63023          	sd	a4,0(a2)
   10868:	0007b703          	ld	a4,0(a5)
   1086c:	0087b783          	ld	a5,8(a5)
   10870:	00e53023          	sd	a4,0(a0) # 18000 <__fini_array_end>
   10874:	00f53423          	sd	a5,8(a0)
   10878:	f55ff06f          	j	107cc <pop_arg+0x50>
   1087c:	00008067          	ret

0000000000010880 <fmt_u>:
   10880:	00050793          	mv	a5,a0
   10884:	00a00693          	li	a3,10
   10888:	00058513          	mv	a0,a1
   1088c:	00079463          	bnez	a5,10894 <fmt_u+0x14>
   10890:	00008067          	ret
   10894:	02d7f733          	remu	a4,a5,a3
   10898:	fff50513          	addi	a0,a0,-1
   1089c:	0307071b          	addiw	a4,a4,48
   108a0:	02d7d7b3          	divu	a5,a5,a3
   108a4:	00e50023          	sb	a4,0(a0)
   108a8:	fe5ff06f          	j	1088c <fmt_u+0xc>

00000000000108ac <getint>:
   108ac:	0cccd637          	lui	a2,0xcccd
   108b0:	800005b7          	lui	a1,0x80000
   108b4:	00050693          	mv	a3,a0
   108b8:	00900313          	li	t1,9
   108bc:	00000513          	li	a0,0
   108c0:	ccc60613          	addi	a2,a2,-820 # ccccccc <__BSS_END__+0xccb44c4>
   108c4:	ff600e13          	li	t3,-10
   108c8:	fff5c593          	not	a1,a1
   108cc:	00a00e93          	li	t4,10
   108d0:	0006b703          	ld	a4,0(a3)
   108d4:	00074783          	lbu	a5,0(a4)
   108d8:	fd07889b          	addiw	a7,a5,-48
   108dc:	00088793          	mv	a5,a7
   108e0:	01137463          	bgeu	t1,a7,108e8 <getint+0x3c>
   108e4:	00008067          	ret
   108e8:	02a66263          	bltu	a2,a0,1090c <getint+0x60>
   108ec:	02ae083b          	mulw	a6,t3,a0
   108f0:	00b8083b          	addw	a6,a6,a1
   108f4:	01184c63          	blt	a6,a7,1090c <getint+0x60>
   108f8:	02ae853b          	mulw	a0,t4,a0
   108fc:	00a7853b          	addw	a0,a5,a0
   10900:	00170713          	addi	a4,a4,1
   10904:	00e6b023          	sd	a4,0(a3)
   10908:	fc9ff06f          	j	108d0 <getint+0x24>
   1090c:	fff00513          	li	a0,-1
   10910:	ff1ff06f          	j	10900 <getint+0x54>

0000000000010914 <out>:
   10914:	00050793          	mv	a5,a0
   10918:	0007a703          	lw	a4,0(a5)
   1091c:	00058513          	mv	a0,a1
   10920:	00060593          	mv	a1,a2
   10924:	02077713          	andi	a4,a4,32
   10928:	00071663          	bnez	a4,10934 <out+0x20>
   1092c:	00078613          	mv	a2,a5
   10930:	6dc0206f          	j	1300c <__fwritex>
   10934:	00008067          	ret

0000000000010938 <pad>:
   10938:	000127b7          	lui	a5,0x12
   1093c:	00f77733          	and	a4,a4,a5
   10940:	08071e63          	bnez	a4,109dc <pad+0xa4>
   10944:	08c6dc63          	bge	a3,a2,109dc <pad+0xa4>
   10948:	ed010113          	addi	sp,sp,-304
   1094c:	12813023          	sd	s0,288(sp)
   10950:	10913c23          	sd	s1,280(sp)
   10954:	11213823          	sd	s2,272(sp)
   10958:	40d604bb          	subw	s1,a2,a3
   1095c:	12113423          	sd	ra,296(sp)
   10960:	11313423          	sd	s3,264(sp)
   10964:	10000793          	li	a5,256
   10968:	00050913          	mv	s2,a0
   1096c:	00048413          	mv	s0,s1
   10970:	0004861b          	sext.w	a2,s1
   10974:	0097d463          	bge	a5,s1,1097c <pad+0x44>
   10978:	10000613          	li	a2,256
   1097c:	00010513          	mv	a0,sp
   10980:	39d010ef          	jal	ra,1251c <memset>
   10984:	0ff00993          	li	s3,255
   10988:	0299ce63          	blt	s3,s1,109c4 <pad+0x8c>
   1098c:	0084561b          	srliw	a2,s0,0x8
   10990:	f0000793          	li	a5,-256
   10994:	02f6063b          	mulw	a2,a2,a5
   10998:	00010593          	mv	a1,sp
   1099c:	00090513          	mv	a0,s2
   109a0:	0086063b          	addw	a2,a2,s0
   109a4:	f71ff0ef          	jal	ra,10914 <out>
   109a8:	12813083          	ld	ra,296(sp)
   109ac:	12013403          	ld	s0,288(sp)
   109b0:	11813483          	ld	s1,280(sp)
   109b4:	11013903          	ld	s2,272(sp)
   109b8:	10813983          	ld	s3,264(sp)
   109bc:	13010113          	addi	sp,sp,304
   109c0:	00008067          	ret
   109c4:	10000613          	li	a2,256
   109c8:	00010593          	mv	a1,sp
   109cc:	00090513          	mv	a0,s2
   109d0:	f45ff0ef          	jal	ra,10914 <out>
   109d4:	f004849b          	addiw	s1,s1,-256
   109d8:	fb1ff06f          	j	10988 <pad+0x50>
   109dc:	00008067          	ret

00000000000109e0 <fmt_fp>:
   109e0:	fffff337          	lui	t1,0xfffff
   109e4:	81010113          	addi	sp,sp,-2032
   109e8:	a3030313          	addi	t1,t1,-1488 # ffffffffffffea30 <__BSS_END__+0xfffffffffffe6228>
   109ec:	7e813023          	sd	s0,2016(sp)
   109f0:	7c913c23          	sd	s1,2008(sp)
   109f4:	7d313423          	sd	s3,1992(sp)
   109f8:	7d413023          	sd	s4,1984(sp)
   109fc:	7b613823          	sd	s6,1968(sp)
   10a00:	7b813023          	sd	s8,1952(sp)
   10a04:	79913c23          	sd	s9,1944(sp)
   10a08:	7e113423          	sd	ra,2024(sp)
   10a0c:	7d213823          	sd	s2,2000(sp)
   10a10:	7b513c23          	sd	s5,1976(sp)
   10a14:	7b713423          	sd	s7,1960(sp)
   10a18:	79a13823          	sd	s10,1936(sp)
   10a1c:	79b13423          	sd	s11,1928(sp)
   10a20:	00070413          	mv	s0,a4
   10a24:	00610133          	add	sp,sp,t1
   10a28:	00002737          	lui	a4,0x2
   10a2c:	00068993          	mv	s3,a3
   10a30:	d1070713          	addi	a4,a4,-752 # 1d10 <exit-0xe410>
   10a34:	04010693          	addi	a3,sp,64
   10a38:	00078a13          	mv	s4,a5
   10a3c:	00d70733          	add	a4,a4,a3
   10a40:	ffffe7b7          	lui	a5,0xffffe
   10a44:	00f707b3          	add	a5,a4,a5
   10a48:	00050493          	mv	s1,a0
   10a4c:	00058c93          	mv	s9,a1
   10a50:	00058513          	mv	a0,a1
   10a54:	00060593          	mv	a1,a2
   10a58:	00060c13          	mv	s8,a2
   10a5c:	00080b13          	mv	s6,a6
   10a60:	00f13023          	sd	a5,0(sp)
   10a64:	2e07ae23          	sw	zero,764(a5) # ffffffffffffe2fc <__BSS_END__+0xfffffffffffe5af4>
   10a68:	00c020ef          	jal	ra,12a74 <__signbitl>
   10a6c:	10050e63          	beqz	a0,10b88 <fmt_fp+0x1a8>
   10a70:	fff00793          	li	a5,-1
   10a74:	03f79793          	slli	a5,a5,0x3f
   10a78:	00016937          	lui	s2,0x16
   10a7c:	00fc4c33          	xor	s8,s8,a5
   10a80:	00100a93          	li	s5,1
   10a84:	f4890913          	addi	s2,s2,-184 # 15f48 <__clzdi2+0xc8>
   10a88:	000c8513          	mv	a0,s9
   10a8c:	000c0593          	mv	a1,s8
   10a90:	7a1010ef          	jal	ra,12a30 <__fpclassifyl>
   10a94:	00100793          	li	a5,1
   10a98:	14a7c863          	blt	a5,a0,10be8 <fmt_fp+0x208>
   10a9c:	020b7b13          	andi	s6,s6,32
   10aa0:	120b1863          	bnez	s6,10bd0 <fmt_fp+0x1f0>
   10aa4:	00016bb7          	lui	s7,0x16
   10aa8:	f68b8b93          	addi	s7,s7,-152 # 15f68 <__clzdi2+0xe8>
   10aac:	000c8613          	mv	a2,s9
   10ab0:	000c0693          	mv	a3,s8
   10ab4:	000c8513          	mv	a0,s9
   10ab8:	000c0593          	mv	a1,s8
   10abc:	3ed030ef          	jal	ra,146a8 <__eqtf2>
   10ac0:	00050863          	beqz	a0,10ad0 <fmt_fp+0xf0>
   10ac4:	100b1c63          	bnez	s6,10bdc <fmt_fp+0x1fc>
   10ac8:	00016bb7          	lui	s7,0x16
   10acc:	f78b8b93          	addi	s7,s7,-136 # 15f78 <__clzdi2+0xf8>
   10ad0:	ffff0737          	lui	a4,0xffff0
   10ad4:	003a8b1b          	addiw	s6,s5,3
   10ad8:	fff70713          	addi	a4,a4,-1 # fffffffffffeffff <__BSS_END__+0xfffffffffffd77f7>
   10adc:	00ea7733          	and	a4,s4,a4
   10ae0:	000b0693          	mv	a3,s6
   10ae4:	00098613          	mv	a2,s3
   10ae8:	02000593          	li	a1,32
   10aec:	00048513          	mv	a0,s1
   10af0:	e49ff0ef          	jal	ra,10938 <pad>
   10af4:	000a8613          	mv	a2,s5
   10af8:	00090593          	mv	a1,s2
   10afc:	00048513          	mv	a0,s1
   10b00:	e15ff0ef          	jal	ra,10914 <out>
   10b04:	00300613          	li	a2,3
   10b08:	000b8593          	mv	a1,s7
   10b0c:	00048513          	mv	a0,s1
   10b10:	e05ff0ef          	jal	ra,10914 <out>
   10b14:	00002737          	lui	a4,0x2
   10b18:	00ea4733          	xor	a4,s4,a4
   10b1c:	000b0693          	mv	a3,s6
   10b20:	00098613          	mv	a2,s3
   10b24:	02000593          	li	a1,32
   10b28:	00048513          	mv	a0,s1
   10b2c:	000b0413          	mv	s0,s6
   10b30:	e09ff0ef          	jal	ra,10938 <pad>
   10b34:	013b5463          	bge	s6,s3,10b3c <fmt_fp+0x15c>
   10b38:	00098413          	mv	s0,s3
   10b3c:	0004051b          	sext.w	a0,s0
   10b40:	00001337          	lui	t1,0x1
   10b44:	5d030313          	addi	t1,t1,1488 # 15d0 <exit-0xeb50>
   10b48:	00610133          	add	sp,sp,t1
   10b4c:	7e813083          	ld	ra,2024(sp)
   10b50:	7e013403          	ld	s0,2016(sp)
   10b54:	7d813483          	ld	s1,2008(sp)
   10b58:	7d013903          	ld	s2,2000(sp)
   10b5c:	7c813983          	ld	s3,1992(sp)
   10b60:	7c013a03          	ld	s4,1984(sp)
   10b64:	7b813a83          	ld	s5,1976(sp)
   10b68:	7b013b03          	ld	s6,1968(sp)
   10b6c:	7a813b83          	ld	s7,1960(sp)
   10b70:	7a013c03          	ld	s8,1952(sp)
   10b74:	79813c83          	ld	s9,1944(sp)
   10b78:	79013d03          	ld	s10,1936(sp)
   10b7c:	78813d83          	ld	s11,1928(sp)
   10b80:	7f010113          	addi	sp,sp,2032
   10b84:	00008067          	ret
   10b88:	00ba5713          	srli	a4,s4,0xb
   10b8c:	00177713          	andi	a4,a4,1
   10b90:	000a079b          	sext.w	a5,s4
   10b94:	00071e63          	bnez	a4,10bb0 <fmt_fp+0x1d0>
   10b98:	0017f793          	andi	a5,a5,1
   10b9c:	02079263          	bnez	a5,10bc0 <fmt_fp+0x1e0>
   10ba0:	00016937          	lui	s2,0x16
   10ba4:	00050a93          	mv	s5,a0
   10ba8:	f4990913          	addi	s2,s2,-183 # 15f49 <__clzdi2+0xc9>
   10bac:	eddff06f          	j	10a88 <fmt_fp+0xa8>
   10bb0:	00016937          	lui	s2,0x16
   10bb4:	00100a93          	li	s5,1
   10bb8:	f4b90913          	addi	s2,s2,-181 # 15f4b <__clzdi2+0xcb>
   10bbc:	ecdff06f          	j	10a88 <fmt_fp+0xa8>
   10bc0:	00016937          	lui	s2,0x16
   10bc4:	00100a93          	li	s5,1
   10bc8:	f4e90913          	addi	s2,s2,-178 # 15f4e <__clzdi2+0xce>
   10bcc:	ebdff06f          	j	10a88 <fmt_fp+0xa8>
   10bd0:	00016bb7          	lui	s7,0x16
   10bd4:	f60b8b93          	addi	s7,s7,-160 # 15f60 <__clzdi2+0xe0>
   10bd8:	ed5ff06f          	j	10aac <fmt_fp+0xcc>
   10bdc:	00016bb7          	lui	s7,0x16
   10be0:	f70b8b93          	addi	s7,s7,-144 # 15f70 <__clzdi2+0xf0>
   10be4:	eedff06f          	j	10ad0 <fmt_fp+0xf0>
   10be8:	00002bb7          	lui	s7,0x2
   10bec:	04010713          	addi	a4,sp,64
   10bf0:	ffffed37          	lui	s10,0xffffe
   10bf4:	d10b8793          	addi	a5,s7,-752 # 1d10 <exit-0xe410>
   10bf8:	00e787b3          	add	a5,a5,a4
   10bfc:	2fcd0613          	addi	a2,s10,764 # ffffffffffffe2fc <__BSS_END__+0xfffffffffffe5af4>
   10c00:	00c78633          	add	a2,a5,a2
   10c04:	000c8513          	mv	a0,s9
   10c08:	000c0593          	mv	a1,s8
   10c0c:	671010ef          	jal	ra,12a7c <frexpl>
   10c10:	00050613          	mv	a2,a0
   10c14:	00058693          	mv	a3,a1
   10c18:	799020ef          	jal	ra,13bb0 <__addtf3>
   10c1c:	00000613          	li	a2,0
   10c20:	00000693          	li	a3,0
   10c24:	00050c93          	mv	s9,a0
   10c28:	00058d93          	mv	s11,a1
   10c2c:	00a13023          	sd	a0,0(sp)
   10c30:	00058c13          	mv	s8,a1
   10c34:	275030ef          	jal	ra,146a8 <__eqtf2>
   10c38:	000c8713          	mv	a4,s9
   10c3c:	02050263          	beqz	a0,10c60 <fmt_fp+0x280>
   10c40:	04010693          	addi	a3,sp,64
   10c44:	d10b8793          	addi	a5,s7,-752
   10c48:	00d787b3          	add	a5,a5,a3
   10c4c:	01a787b3          	add	a5,a5,s10
   10c50:	2fc7a683          	lw	a3,764(a5)
   10c54:	00f13023          	sd	a5,0(sp)
   10c58:	fff6869b          	addiw	a3,a3,-1
   10c5c:	2ed7ae23          	sw	a3,764(a5)
   10c60:	020b6793          	ori	a5,s6,32
   10c64:	00f13023          	sd	a5,0(sp)
   10c68:	06100693          	li	a3,97
   10c6c:	3cd79a63          	bne	a5,a3,11040 <fmt_fp+0x660>
   10c70:	020b7793          	andi	a5,s6,32
   10c74:	00f13c23          	sd	a5,24(sp)
   10c78:	00078463          	beqz	a5,10c80 <fmt_fp+0x2a0>
   10c7c:	00990913          	addi	s2,s2,9
   10c80:	002a879b          	addiw	a5,s5,2
   10c84:	00078a93          	mv	s5,a5
   10c88:	00f13023          	sd	a5,0(sp)
   10c8c:	0004079b          	sext.w	a5,s0
   10c90:	00f13823          	sd	a5,16(sp)
   10c94:	01a00693          	li	a3,26
   10c98:	00016d37          	lui	s10,0x16
   10c9c:	0686e263          	bltu	a3,s0,10d00 <fmt_fp+0x320>
   10ca0:	0a8d3b83          	ld	s7,168(s10) # 160a8 <__clzdi2+0x228>
   10ca4:	01b00c13          	li	s8,27
   10ca8:	408c0c3b          	subw	s8,s8,s0
   10cac:	00000c93          	li	s9,0
   10cb0:	fff00813          	li	a6,-1
   10cb4:	000b8693          	mv	a3,s7
   10cb8:	fffc0c1b          	addiw	s8,s8,-1
   10cbc:	210c1863          	bne	s8,a6,10ecc <fmt_fp+0x4ec>
   10cc0:	00094603          	lbu	a2,0(s2)
   10cc4:	02d00693          	li	a3,45
   10cc8:	22d61a63          	bne	a2,a3,10efc <fmt_fp+0x51c>
   10ccc:	03fc1c13          	slli	s8,s8,0x3f
   10cd0:	000c8613          	mv	a2,s9
   10cd4:	000b8693          	mv	a3,s7
   10cd8:	00070513          	mv	a0,a4
   10cdc:	018dc5b3          	xor	a1,s11,s8
   10ce0:	310040ef          	jal	ra,14ff0 <__subtf3>
   10ce4:	00050613          	mv	a2,a0
   10ce8:	00058693          	mv	a3,a1
   10cec:	000c8513          	mv	a0,s9
   10cf0:	000b8593          	mv	a1,s7
   10cf4:	6bd020ef          	jal	ra,13bb0 <__addtf3>
   10cf8:	00050c93          	mv	s9,a0
   10cfc:	0185cc33          	xor	s8,a1,s8
   10d00:	000026b7          	lui	a3,0x2
   10d04:	ffffe7b7          	lui	a5,0xffffe
   10d08:	04010613          	addi	a2,sp,64
   10d0c:	d1068713          	addi	a4,a3,-752 # 1d10 <exit-0xe410>
   10d10:	30078b93          	addi	s7,a5,768 # ffffffffffffe300 <__BSS_END__+0xfffffffffffe5af8>
   10d14:	00c70733          	add	a4,a4,a2
   10d18:	01770bb3          	add	s7,a4,s7
   10d1c:	d1068713          	addi	a4,a3,-752
   10d20:	00c70733          	add	a4,a4,a2
   10d24:	00f707b3          	add	a5,a4,a5
   10d28:	00f13423          	sd	a5,8(sp)
   10d2c:	2fc7a783          	lw	a5,764(a5)
   10d30:	00cb8d93          	addi	s11,s7,12
   10d34:	000d8593          	mv	a1,s11
   10d38:	41f7d51b          	sraiw	a0,a5,0x1f
   10d3c:	00f547b3          	xor	a5,a0,a5
   10d40:	40a7853b          	subw	a0,a5,a0
   10d44:	b3dff0ef          	jal	ra,10880 <fmt_u>
   10d48:	01b51a63          	bne	a0,s11,10d5c <fmt_fp+0x37c>
   10d4c:	00813703          	ld	a4,8(sp)
   10d50:	03000793          	li	a5,48
   10d54:	00bb8513          	addi	a0,s7,11
   10d58:	30f705a3          	sb	a5,779(a4) # 230b <exit-0xde15>
   10d5c:	00002737          	lui	a4,0x2
   10d60:	d1070713          	addi	a4,a4,-752 # 1d10 <exit-0xe410>
   10d64:	04010693          	addi	a3,sp,64
   10d68:	00d70733          	add	a4,a4,a3
   10d6c:	ffffe7b7          	lui	a5,0xffffe
   10d70:	00f707b3          	add	a5,a4,a5
   10d74:	2fc7a703          	lw	a4,764(a5) # ffffffffffffe2fc <__BSS_END__+0xfffffffffffe5af4>
   10d78:	00f13423          	sd	a5,8(sp)
   10d7c:	02d00793          	li	a5,45
   10d80:	00074463          	bltz	a4,10d88 <fmt_fp+0x3a8>
   10d84:	02b00793          	li	a5,43
   10d88:	00002737          	lui	a4,0x2
   10d8c:	fef50fa3          	sb	a5,-1(a0)
   10d90:	d1070713          	addi	a4,a4,-752 # 1d10 <exit-0xe410>
   10d94:	ffffe7b7          	lui	a5,0xffffe
   10d98:	04010693          	addi	a3,sp,64
   10d9c:	31078793          	addi	a5,a5,784 # ffffffffffffe310 <__BSS_END__+0xfffffffffffe5b08>
   10da0:	00d70733          	add	a4,a4,a3
   10da4:	00f70db3          	add	s11,a4,a5
   10da8:	00016737          	lui	a4,0x16
   10dac:	2f070793          	addi	a5,a4,752 # 162f0 <xdigits>
   10db0:	00f13423          	sd	a5,8(sp)
   10db4:	0a8d3783          	ld	a5,168(s10)
   10db8:	00fb0b1b          	addiw	s6,s6,15
   10dbc:	ffe50b93          	addi	s7,a0,-2
   10dc0:	ff650f23          	sb	s6,-2(a0)
   10dc4:	02f13423          	sd	a5,40(sp)
   10dc8:	03b13023          	sd	s11,32(sp)
   10dcc:	008a7d13          	andi	s10,s4,8
   10dd0:	000c0593          	mv	a1,s8
   10dd4:	000c8513          	mv	a0,s9
   10dd8:	519040ef          	jal	ra,15af0 <__fixtfsi>
   10ddc:	00813783          	ld	a5,8(sp)
   10de0:	0005051b          	sext.w	a0,a0
   10de4:	001d8b13          	addi	s6,s11,1
   10de8:	00a78733          	add	a4,a5,a0
   10dec:	00074703          	lbu	a4,0(a4)
   10df0:	01813783          	ld	a5,24(sp)
   10df4:	00e7e733          	or	a4,a5,a4
   10df8:	00ed8023          	sb	a4,0(s11)
   10dfc:	67d040ef          	jal	ra,15c78 <__floatsitf>
   10e00:	00050613          	mv	a2,a0
   10e04:	00058693          	mv	a3,a1
   10e08:	000c8513          	mv	a0,s9
   10e0c:	000c0593          	mv	a1,s8
   10e10:	1e0040ef          	jal	ra,14ff0 <__subtf3>
   10e14:	02813683          	ld	a3,40(sp)
   10e18:	00000613          	li	a2,0
   10e1c:	15d030ef          	jal	ra,14778 <__multf3>
   10e20:	02013783          	ld	a5,32(sp)
   10e24:	00050813          	mv	a6,a0
   10e28:	00058713          	mv	a4,a1
   10e2c:	40fb06b3          	sub	a3,s6,a5
   10e30:	00100793          	li	a5,1
   10e34:	00050c93          	mv	s9,a0
   10e38:	00058c13          	mv	s8,a1
   10e3c:	02f69c63          	bne	a3,a5,10e74 <fmt_fp+0x494>
   10e40:	00000613          	li	a2,0
   10e44:	00000693          	li	a3,0
   10e48:	02b13c23          	sd	a1,56(sp)
   10e4c:	02a13823          	sd	a0,48(sp)
   10e50:	059030ef          	jal	ra,146a8 <__eqtf2>
   10e54:	000c8813          	mv	a6,s9
   10e58:	000c0713          	mv	a4,s8
   10e5c:	00051663          	bnez	a0,10e68 <fmt_fp+0x488>
   10e60:	00804463          	bgtz	s0,10e68 <fmt_fp+0x488>
   10e64:	020d0463          	beqz	s10,10e8c <fmt_fp+0x4ac>
   10e68:	02e00793          	li	a5,46
   10e6c:	002d8b13          	addi	s6,s11,2
   10e70:	00fd80a3          	sb	a5,1(s11)
   10e74:	00000613          	li	a2,0
   10e78:	00000693          	li	a3,0
   10e7c:	00080513          	mv	a0,a6
   10e80:	00070593          	mv	a1,a4
   10e84:	025030ef          	jal	ra,146a8 <__eqtf2>
   10e88:	0a051063          	bnez	a0,10f28 <fmt_fp+0x548>
   10e8c:	000027b7          	lui	a5,0x2
   10e90:	04010693          	addi	a3,sp,64
   10e94:	d1078793          	addi	a5,a5,-752 # 1d10 <exit-0xe410>
   10e98:	00d787b3          	add	a5,a5,a3
   10e9c:	ffffe737          	lui	a4,0xffffe
   10ea0:	00e78c33          	add	s8,a5,a4
   10ea4:	00013683          	ld	a3,0(sp)
   10ea8:	30cc0c13          	addi	s8,s8,780
   10eac:	800007b7          	lui	a5,0x80000
   10eb0:	417c0c33          	sub	s8,s8,s7
   10eb4:	ffd7c793          	xori	a5,a5,-3
   10eb8:	418787b3          	sub	a5,a5,s8
   10ebc:	40d787b3          	sub	a5,a5,a3
   10ec0:	0687d863          	bge	a5,s0,10f30 <fmt_fp+0x550>
   10ec4:	fff00513          	li	a0,-1
   10ec8:	c79ff06f          	j	10b40 <fmt_fp+0x160>
   10ecc:	000c8513          	mv	a0,s9
   10ed0:	000b8593          	mv	a1,s7
   10ed4:	00000613          	li	a2,0
   10ed8:	00e13423          	sd	a4,8(sp)
   10edc:	09d030ef          	jal	ra,14778 <__multf3>
   10ee0:	040037b7          	lui	a5,0x4003
   10ee4:	00813703          	ld	a4,8(sp)
   10ee8:	00050c93          	mv	s9,a0
   10eec:	00058b93          	mv	s7,a1
   10ef0:	fff00813          	li	a6,-1
   10ef4:	02479693          	slli	a3,a5,0x24
   10ef8:	dc1ff06f          	j	10cb8 <fmt_fp+0x2d8>
   10efc:	00070613          	mv	a2,a4
   10f00:	000d8693          	mv	a3,s11
   10f04:	000c8513          	mv	a0,s9
   10f08:	000b8593          	mv	a1,s7
   10f0c:	4a5020ef          	jal	ra,13bb0 <__addtf3>
   10f10:	000c8613          	mv	a2,s9
   10f14:	000b8693          	mv	a3,s7
   10f18:	0d8040ef          	jal	ra,14ff0 <__subtf3>
   10f1c:	00050c93          	mv	s9,a0
   10f20:	00058c13          	mv	s8,a1
   10f24:	dddff06f          	j	10d00 <fmt_fp+0x320>
   10f28:	000b0d93          	mv	s11,s6
   10f2c:	ea5ff06f          	j	10dd0 <fmt_fp+0x3f0>
   10f30:	000027b7          	lui	a5,0x2
   10f34:	d1078793          	addi	a5,a5,-752 # 1d10 <exit-0xe410>
   10f38:	04010693          	addi	a3,sp,64
   10f3c:	00d787b3          	add	a5,a5,a3
   10f40:	31070713          	addi	a4,a4,784 # ffffffffffffe310 <__BSS_END__+0xfffffffffffe5b08>
   10f44:	00e78733          	add	a4,a5,a4
   10f48:	40eb0b33          	sub	s6,s6,a4
   10f4c:	000c079b          	sext.w	a5,s8
   10f50:	016c0dbb          	addw	s11,s8,s6
   10f54:	0e040263          	beqz	s0,11038 <fmt_fp+0x658>
   10f58:	fffb0713          	addi	a4,s6,-1
   10f5c:	0ce44e63          	blt	s0,a4,11038 <fmt_fp+0x658>
   10f60:	01013703          	ld	a4,16(sp)
   10f64:	0027041b          	addiw	s0,a4,2
   10f68:	00f4043b          	addw	s0,s0,a5
   10f6c:	01540cbb          	addw	s9,s0,s5
   10f70:	000a0713          	mv	a4,s4
   10f74:	000c8693          	mv	a3,s9
   10f78:	00098613          	mv	a2,s3
   10f7c:	02000593          	li	a1,32
   10f80:	00048513          	mv	a0,s1
   10f84:	9b5ff0ef          	jal	ra,10938 <pad>
   10f88:	00013603          	ld	a2,0(sp)
   10f8c:	00090593          	mv	a1,s2
   10f90:	00048513          	mv	a0,s1
   10f94:	981ff0ef          	jal	ra,10914 <out>
   10f98:	00010737          	lui	a4,0x10
   10f9c:	000c8693          	mv	a3,s9
   10fa0:	00ea4733          	xor	a4,s4,a4
   10fa4:	00098613          	mv	a2,s3
   10fa8:	03000593          	li	a1,48
   10fac:	00048513          	mv	a0,s1
   10fb0:	989ff0ef          	jal	ra,10938 <pad>
   10fb4:	000027b7          	lui	a5,0x2
   10fb8:	04010713          	addi	a4,sp,64
   10fbc:	ffffe5b7          	lui	a1,0xffffe
   10fc0:	d1078793          	addi	a5,a5,-752 # 1d10 <exit-0xe410>
   10fc4:	00e787b3          	add	a5,a5,a4
   10fc8:	31058593          	addi	a1,a1,784 # ffffffffffffe310 <__BSS_END__+0xfffffffffffe5b08>
   10fcc:	00b785b3          	add	a1,a5,a1
   10fd0:	000b0613          	mv	a2,s6
   10fd4:	00048513          	mv	a0,s1
   10fd8:	93dff0ef          	jal	ra,10914 <out>
   10fdc:	00000713          	li	a4,0
   10fe0:	00000693          	li	a3,0
   10fe4:	41b4063b          	subw	a2,s0,s11
   10fe8:	03000593          	li	a1,48
   10fec:	00048513          	mv	a0,s1
   10ff0:	949ff0ef          	jal	ra,10938 <pad>
   10ff4:	000c0613          	mv	a2,s8
   10ff8:	000b8593          	mv	a1,s7
   10ffc:	00048513          	mv	a0,s1
   11000:	915ff0ef          	jal	ra,10914 <out>
   11004:	000a091b          	sext.w	s2,s4
   11008:	00002737          	lui	a4,0x2
   1100c:	00e94733          	xor	a4,s2,a4
   11010:	000c8693          	mv	a3,s9
   11014:	00098613          	mv	a2,s3
   11018:	02000593          	li	a1,32
   1101c:	00048513          	mv	a0,s1
   11020:	000c8a93          	mv	s5,s9
   11024:	915ff0ef          	jal	ra,10938 <pad>
   11028:	013cd463          	bge	s9,s3,11030 <fmt_fp+0x650>
   1102c:	00098a93          	mv	s5,s3
   11030:	000a851b          	sext.w	a0,s5
   11034:	b0dff06f          	j	10b40 <fmt_fp+0x160>
   11038:	000d841b          	sext.w	s0,s11
   1103c:	f31ff06f          	j	10f6c <fmt_fp+0x58c>
   11040:	00045463          	bgez	s0,11048 <fmt_fp+0x668>
   11044:	00600413          	li	s0,6
   11048:	00000613          	li	a2,0
   1104c:	00000693          	li	a3,0
   11050:	00070513          	mv	a0,a4
   11054:	000d8593          	mv	a1,s11
   11058:	00e13823          	sd	a4,16(sp)
   1105c:	64c030ef          	jal	ra,146a8 <__eqtf2>
   11060:	04050a63          	beqz	a0,110b4 <fmt_fp+0x6d4>
   11064:	01013703          	ld	a4,16(sp)
   11068:	000166b7          	lui	a3,0x16
   1106c:	1086b683          	ld	a3,264(a3) # 16108 <__clzdi2+0x288>
   11070:	00070513          	mv	a0,a4
   11074:	00000613          	li	a2,0
   11078:	000d8593          	mv	a1,s11
   1107c:	6fc030ef          	jal	ra,14778 <__multf3>
   11080:	00002737          	lui	a4,0x2
   11084:	d1070713          	addi	a4,a4,-752 # 1d10 <exit-0xe410>
   11088:	04010693          	addi	a3,sp,64
   1108c:	00d70733          	add	a4,a4,a3
   11090:	ffffe7b7          	lui	a5,0xffffe
   11094:	00f707b3          	add	a5,a4,a5
   11098:	00f13823          	sd	a5,16(sp)
   1109c:	2fc7a783          	lw	a5,764(a5) # ffffffffffffe2fc <__BSS_END__+0xfffffffffffe5af4>
   110a0:	01013703          	ld	a4,16(sp)
   110a4:	00050c93          	mv	s9,a0
   110a8:	fe47879b          	addiw	a5,a5,-28
   110ac:	00058c13          	mv	s8,a1
   110b0:	2ef72e23          	sw	a5,764(a4)
   110b4:	04c12303          	lw	t1,76(sp)
   110b8:	00002737          	lui	a4,0x2
   110bc:	b4870693          	addi	a3,a4,-1208 # 1b48 <exit-0xe5d8>
   110c0:	04010613          	addi	a2,sp,64
   110c4:	ffffe7b7          	lui	a5,0xffffe
   110c8:	00d608b3          	add	a7,a2,a3
   110cc:	00035a63          	bgez	t1,110e0 <fmt_fp+0x700>
   110d0:	d1070713          	addi	a4,a4,-752
   110d4:	33878793          	addi	a5,a5,824 # ffffffffffffe338 <__BSS_END__+0xfffffffffffe5b30>
   110d8:	00c70733          	add	a4,a4,a2
   110dc:	00f708b3          	add	a7,a4,a5
   110e0:	00016737          	lui	a4,0x16
   110e4:	11873d83          	ld	s11,280(a4) # 16118 <__clzdi2+0x298>
   110e8:	00088d13          	mv	s10,a7
   110ec:	000c0593          	mv	a1,s8
   110f0:	000c8513          	mv	a0,s9
   110f4:	01113c23          	sd	a7,24(sp)
   110f8:	00613823          	sd	t1,16(sp)
   110fc:	2c9040ef          	jal	ra,15bc4 <__fixunstfsi>
   11100:	00ad2023          	sw	a0,0(s10)
   11104:	0005051b          	sext.w	a0,a0
   11108:	3f9040ef          	jal	ra,15d00 <__floatunsitf>
   1110c:	00050613          	mv	a2,a0
   11110:	00058693          	mv	a3,a1
   11114:	000c8513          	mv	a0,s9
   11118:	000c0593          	mv	a1,s8
   1111c:	6d5030ef          	jal	ra,14ff0 <__subtf3>
   11120:	00000613          	li	a2,0
   11124:	000d8693          	mv	a3,s11
   11128:	650030ef          	jal	ra,14778 <__multf3>
   1112c:	00000613          	li	a2,0
   11130:	00000693          	li	a3,0
   11134:	00050c93          	mv	s9,a0
   11138:	00058c13          	mv	s8,a1
   1113c:	56c030ef          	jal	ra,146a8 <__eqtf2>
   11140:	01013303          	ld	t1,16(sp)
   11144:	01813883          	ld	a7,24(sp)
   11148:	004d0d13          	addi	s10,s10,4
   1114c:	fa0510e3          	bnez	a0,110ec <fmt_fp+0x70c>
   11150:	3b9ad537          	lui	a0,0x3b9ad
   11154:	00088b93          	mv	s7,a7
   11158:	00000713          	li	a4,0
   1115c:	01d00e13          	li	t3,29
   11160:	a0050513          	addi	a0,a0,-1536 # 3b9aca00 <__BSS_END__+0x3b9941f8>
   11164:	1a604c63          	bgtz	t1,1131c <fmt_fp+0x93c>
   11168:	02070263          	beqz	a4,1118c <fmt_fp+0x7ac>
   1116c:	000027b7          	lui	a5,0x2
   11170:	d1078793          	addi	a5,a5,-752 # 1d10 <exit-0xe410>
   11174:	04010693          	addi	a3,sp,64
   11178:	ffffe737          	lui	a4,0xffffe
   1117c:	00d787b3          	add	a5,a5,a3
   11180:	00e787b3          	add	a5,a5,a4
   11184:	00f13823          	sd	a5,16(sp)
   11188:	2e67ae23          	sw	t1,764(a5)
   1118c:	00900693          	li	a3,9
   11190:	02d4071b          	addiw	a4,s0,45
   11194:	02d7573b          	divuw	a4,a4,a3
   11198:	000027b7          	lui	a5,0x2
   1119c:	04010613          	addi	a2,sp,64
   111a0:	d1078793          	addi	a5,a5,-752 # 1d10 <exit-0xe410>
   111a4:	ffffe6b7          	lui	a3,0xffffe
   111a8:	00c787b3          	add	a5,a5,a2
   111ac:	00d787b3          	add	a5,a5,a3
   111b0:	2fc7a683          	lw	a3,764(a5)
   111b4:	3b9ade37          	lui	t3,0x3b9ad
   111b8:	00f13823          	sd	a5,16(sp)
   111bc:	00000613          	li	a2,0
   111c0:	ff700f13          	li	t5,-9
   111c4:	00100f93          	li	t6,1
   111c8:	a00e0e1b          	addiw	t3,t3,-1536
   111cc:	06600293          	li	t0,102
   111d0:	0017071b          	addiw	a4,a4,1
   111d4:	02071713          	slli	a4,a4,0x20
   111d8:	02075713          	srli	a4,a4,0x20
   111dc:	00271e93          	slli	t4,a4,0x2
   111e0:	1a06c663          	bltz	a3,1138c <fmt_fp+0x9ac>
   111e4:	02060263          	beqz	a2,11208 <fmt_fp+0x828>
   111e8:	000027b7          	lui	a5,0x2
   111ec:	d1078793          	addi	a5,a5,-752 # 1d10 <exit-0xe410>
   111f0:	04010613          	addi	a2,sp,64
   111f4:	ffffe737          	lui	a4,0xffffe
   111f8:	00c787b3          	add	a5,a5,a2
   111fc:	00e787b3          	add	a5,a5,a4
   11200:	00f13823          	sd	a5,16(sp)
   11204:	2ed7ae23          	sw	a3,764(a5)
   11208:	00000313          	li	t1,0
   1120c:	03abf263          	bgeu	s7,s10,11230 <fmt_fp+0x850>
   11210:	41788333          	sub	t1,a7,s7
   11214:	000ba683          	lw	a3,0(s7)
   11218:	40235713          	srai	a4,t1,0x2
   1121c:	00900313          	li	t1,9
   11220:	02e3033b          	mulw	t1,t1,a4
   11224:	00a00613          	li	a2,10
   11228:	00a00713          	li	a4,10
   1122c:	1ee6f463          	bgeu	a3,a4,11414 <fmt_fp+0xa34>
   11230:	00013783          	ld	a5,0(sp)
   11234:	06700613          	li	a2,103
   11238:	00000693          	li	a3,0
   1123c:	f9a78713          	addi	a4,a5,-102
   11240:	00e03733          	snez	a4,a4
   11244:	0267073b          	mulw	a4,a4,t1
   11248:	40e4073b          	subw	a4,s0,a4
   1124c:	00c79463          	bne	a5,a2,11254 <fmt_fp+0x874>
   11250:	008036b3          	snez	a3,s0
   11254:	40d7063b          	subw	a2,a4,a3
   11258:	411d0733          	sub	a4,s10,a7
   1125c:	40275713          	srai	a4,a4,0x2
   11260:	fff70713          	addi	a4,a4,-1 # ffffffffffffdfff <__BSS_END__+0xfffffffffffe57f7>
   11264:	00900593          	li	a1,9
   11268:	02b70733          	mul	a4,a4,a1
   1126c:	24e65663          	bge	a2,a4,114b8 <fmt_fp+0xad8>
   11270:	00024737          	lui	a4,0x24
   11274:	00c7073b          	addw	a4,a4,a2
   11278:	00900613          	li	a2,9
   1127c:	02c74c3b          	divw	s8,a4,a2
   11280:	ffff06b7          	lui	a3,0xffff0
   11284:	00468693          	addi	a3,a3,4 # ffffffffffff0004 <__BSS_END__+0xfffffffffffd77fc>
   11288:	00a00593          	li	a1,10
   1128c:	02c7673b          	remw	a4,a4,a2
   11290:	002c1c13          	slli	s8,s8,0x2
   11294:	00dc0c33          	add	s8,s8,a3
   11298:	01888c33          	add	s8,a7,s8
   1129c:	00a00693          	li	a3,10
   112a0:	00900613          	li	a2,9
   112a4:	0017071b          	addiw	a4,a4,1
   112a8:	16c71c63          	bne	a4,a2,11420 <fmt_fp+0xa40>
   112ac:	000c2d83          	lw	s11,0(s8)
   112b0:	00068e1b          	sext.w	t3,a3
   112b4:	02ddf63b          	remuw	a2,s11,a3
   112b8:	00060593          	mv	a1,a2
   112bc:	00061663          	bnez	a2,112c8 <fmt_fp+0x8e8>
   112c0:	004c0713          	addi	a4,s8,4
   112c4:	1eed0463          	beq	s10,a4,114ac <fmt_fp+0xacc>
   112c8:	03cdd73b          	divuw	a4,s11,t3
   112cc:	00177713          	andi	a4,a4,1
   112d0:	02071063          	bnez	a4,112f0 <fmt_fp+0x910>
   112d4:	3b9ad737          	lui	a4,0x3b9ad
   112d8:	a0070713          	addi	a4,a4,-1536 # 3b9aca00 <__BSS_END__+0x3b9941f8>
   112dc:	14e69863          	bne	a3,a4,1142c <fmt_fp+0xa4c>
   112e0:	158bf663          	bgeu	s7,s8,1142c <fmt_fp+0xa4c>
   112e4:	ffcc2703          	lw	a4,-4(s8)
   112e8:	00177713          	andi	a4,a4,1
   112ec:	14070063          	beqz	a4,1142c <fmt_fp+0xa4c>
   112f0:	00016537          	lui	a0,0x16
   112f4:	0c853c83          	ld	s9,200(a0) # 160c8 <__clzdi2+0x248>
   112f8:	00100713          	li	a4,1
   112fc:	4016d693          	srai	a3,a3,0x1
   11300:	12d66e63          	bltu	a2,a3,1143c <fmt_fp+0xa5c>
   11304:	00c69663          	bne	a3,a2,11310 <fmt_fp+0x930>
   11308:	004c0693          	addi	a3,s8,4
   1130c:	38dd0e63          	beq	s10,a3,116a8 <fmt_fp+0xcc8>
   11310:	000166b7          	lui	a3,0x16
   11314:	0d86b683          	ld	a3,216(a3) # 160d8 <__clzdi2+0x258>
   11318:	12c0006f          	j	11444 <fmt_fp+0xa64>
   1131c:	0003061b          	sext.w	a2,t1
   11320:	006e5463          	bge	t3,t1,11328 <fmt_fp+0x948>
   11324:	01d00613          	li	a2,29
   11328:	ffcd0593          	addi	a1,s10,-4
   1132c:	00000713          	li	a4,0
   11330:	0375f463          	bgeu	a1,s7,11358 <fmt_fp+0x978>
   11334:	00070663          	beqz	a4,11340 <fmt_fp+0x960>
   11338:	feebae23          	sw	a4,-4(s7)
   1133c:	ffcb8b93          	addi	s7,s7,-4
   11340:	01abf663          	bgeu	s7,s10,1134c <fmt_fp+0x96c>
   11344:	ffcd2703          	lw	a4,-4(s10)
   11348:	02070e63          	beqz	a4,11384 <fmt_fp+0x9a4>
   1134c:	40c3033b          	subw	t1,t1,a2
   11350:	00100713          	li	a4,1
   11354:	e11ff06f          	j	11164 <fmt_fp+0x784>
   11358:	0005e683          	lwu	a3,0(a1)
   1135c:	02071713          	slli	a4,a4,0x20
   11360:	02075713          	srli	a4,a4,0x20
   11364:	00c696b3          	sll	a3,a3,a2
   11368:	00e68733          	add	a4,a3,a4
   1136c:	02a776b3          	remu	a3,a4,a0
   11370:	ffc58593          	addi	a1,a1,-4
   11374:	02a75733          	divu	a4,a4,a0
   11378:	00d5a223          	sw	a3,4(a1)
   1137c:	0007071b          	sext.w	a4,a4
   11380:	fb1ff06f          	j	11330 <fmt_fp+0x950>
   11384:	ffcd0d13          	addi	s10,s10,-4
   11388:	fb9ff06f          	j	11340 <fmt_fp+0x960>
   1138c:	00900593          	li	a1,9
   11390:	01e6c463          	blt	a3,t5,11398 <fmt_fp+0x9b8>
   11394:	40d005bb          	negw	a1,a3
   11398:	00bf93bb          	sllw	t2,t6,a1
   1139c:	fff3839b          	addiw	t2,t2,-1
   113a0:	40be5c3b          	sraw	s8,t3,a1
   113a4:	000b8613          	mv	a2,s7
   113a8:	00000513          	li	a0,0
   113ac:	05a66463          	bltu	a2,s10,113f4 <fmt_fp+0xa14>
   113b0:	000ba603          	lw	a2,0(s7)
   113b4:	00061463          	bnez	a2,113bc <fmt_fp+0x9dc>
   113b8:	004b8b93          	addi	s7,s7,4
   113bc:	00050663          	beqz	a0,113c8 <fmt_fp+0x9e8>
   113c0:	00ad2023          	sw	a0,0(s10)
   113c4:	004d0d13          	addi	s10,s10,4
   113c8:	00013783          	ld	a5,0(sp)
   113cc:	00088613          	mv	a2,a7
   113d0:	00578463          	beq	a5,t0,113d8 <fmt_fp+0x9f8>
   113d4:	000b8613          	mv	a2,s7
   113d8:	40cd0533          	sub	a0,s10,a2
   113dc:	40255513          	srai	a0,a0,0x2
   113e0:	00a75463          	bge	a4,a0,113e8 <fmt_fp+0xa08>
   113e4:	01d60d33          	add	s10,a2,t4
   113e8:	00b686bb          	addw	a3,a3,a1
   113ec:	00100613          	li	a2,1
   113f0:	df1ff06f          	j	111e0 <fmt_fp+0x800>
   113f4:	00062303          	lw	t1,0(a2)
   113f8:	00460613          	addi	a2,a2,4
   113fc:	00b35cbb          	srlw	s9,t1,a1
   11400:	00ac853b          	addw	a0,s9,a0
   11404:	fea62e23          	sw	a0,-4(a2)
   11408:	00737533          	and	a0,t1,t2
   1140c:	0385053b          	mulw	a0,a0,s8
   11410:	f9dff06f          	j	113ac <fmt_fp+0x9cc>
   11414:	02e6073b          	mulw	a4,a2,a4
   11418:	0013031b          	addiw	t1,t1,1
   1141c:	e11ff06f          	j	1122c <fmt_fp+0x84c>
   11420:	02d586bb          	mulw	a3,a1,a3
   11424:	0017071b          	addiw	a4,a4,1
   11428:	e81ff06f          	j	112a8 <fmt_fp+0x8c8>
   1142c:	00016537          	lui	a0,0x16
   11430:	0b853c83          	ld	s9,184(a0) # 160b8 <__clzdi2+0x238>
   11434:	00000713          	li	a4,0
   11438:	ec5ff06f          	j	112fc <fmt_fp+0x91c>
   1143c:	000166b7          	lui	a3,0x16
   11440:	0f86b683          	ld	a3,248(a3) # 160f8 <__clzdi2+0x278>
   11444:	020a8063          	beqz	s5,11464 <fmt_fp+0xa84>
   11448:	00094503          	lbu	a0,0(s2)
   1144c:	02d00613          	li	a2,45
   11450:	00c51a63          	bne	a0,a2,11464 <fmt_fp+0xa84>
   11454:	fff00613          	li	a2,-1
   11458:	03f61613          	slli	a2,a2,0x3f
   1145c:	00ccccb3          	xor	s9,s9,a2
   11460:	00c6c6b3          	xor	a3,a3,a2
   11464:	00000613          	li	a2,0
   11468:	00070513          	mv	a0,a4
   1146c:	40bd8dbb          	subw	s11,s11,a1
   11470:	000c8593          	mv	a1,s9
   11474:	03113423          	sd	a7,40(sp)
   11478:	01c13c23          	sd	t3,24(sp)
   1147c:	02613023          	sd	t1,32(sp)
   11480:	00e13823          	sd	a4,16(sp)
   11484:	72c020ef          	jal	ra,13bb0 <__addtf3>
   11488:	01013703          	ld	a4,16(sp)
   1148c:	000c8693          	mv	a3,s9
   11490:	00070613          	mv	a2,a4
   11494:	214030ef          	jal	ra,146a8 <__eqtf2>
   11498:	01813e03          	ld	t3,24(sp)
   1149c:	02813883          	ld	a7,40(sp)
   114a0:	20051a63          	bnez	a0,116b4 <fmt_fp+0xcd4>
   114a4:	02013303          	ld	t1,32(sp)
   114a8:	01bc2023          	sw	s11,0(s8)
   114ac:	004c0c13          	addi	s8,s8,4
   114b0:	01ac7463          	bgeu	s8,s10,114b8 <fmt_fp+0xad8>
   114b4:	000c0d13          	mv	s10,s8
   114b8:	01abf663          	bgeu	s7,s10,114c4 <fmt_fp+0xae4>
   114bc:	ffcd2703          	lw	a4,-4(s10)
   114c0:	24070e63          	beqz	a4,1171c <fmt_fp+0xd3c>
   114c4:	00013783          	ld	a5,0(sp)
   114c8:	06700713          	li	a4,103
   114cc:	06e79a63          	bne	a5,a4,11540 <fmt_fp+0xb60>
   114d0:	00041463          	bnez	s0,114d8 <fmt_fp+0xaf8>
   114d4:	00100413          	li	s0,1
   114d8:	24835663          	bge	t1,s0,11724 <fmt_fp+0xd44>
   114dc:	ffc00713          	li	a4,-4
   114e0:	24e34263          	blt	t1,a4,11724 <fmt_fp+0xd44>
   114e4:	0013071b          	addiw	a4,t1,1
   114e8:	fffb0b1b          	addiw	s6,s6,-1
   114ec:	40e4043b          	subw	s0,s0,a4
   114f0:	008a7713          	andi	a4,s4,8
   114f4:	04071663          	bnez	a4,11540 <fmt_fp+0xb60>
   114f8:	00900693          	li	a3,9
   114fc:	01abf663          	bgeu	s7,s10,11508 <fmt_fp+0xb28>
   11500:	ffcd2603          	lw	a2,-4(s10)
   11504:	24061063          	bnez	a2,11744 <fmt_fp+0xd64>
   11508:	411d0733          	sub	a4,s10,a7
   1150c:	40275713          	srai	a4,a4,0x2
   11510:	00900613          	li	a2,9
   11514:	fff70713          	addi	a4,a4,-1
   11518:	02c70733          	mul	a4,a4,a2
   1151c:	06600593          	li	a1,102
   11520:	020b6613          	ori	a2,s6,32
   11524:	22b61863          	bne	a2,a1,11754 <fmt_fp+0xd74>
   11528:	40d70733          	sub	a4,a4,a3
   1152c:	00075463          	bgez	a4,11534 <fmt_fp+0xb54>
   11530:	00000713          	li	a4,0
   11534:	00e45463          	bge	s0,a4,1153c <fmt_fp+0xb5c>
   11538:	00040713          	mv	a4,s0
   1153c:	0007041b          	sext.w	s0,a4
   11540:	80000737          	lui	a4,0x80000
   11544:	ffd74713          	xori	a4,a4,-3
   11548:	96874ee3          	blt	a4,s0,10ec4 <fmt_fp+0x4e4>
   1154c:	008a7c93          	andi	s9,s4,8
   11550:	01946c33          	or	s8,s0,s9
   11554:	018036b3          	snez	a3,s8
   11558:	0014071b          	addiw	a4,s0,1
   1155c:	00e6853b          	addw	a0,a3,a4
   11560:	000a079b          	sext.w	a5,s4
   11564:	800006b7          	lui	a3,0x80000
   11568:	fff6c693          	not	a3,a3
   1156c:	00f13023          	sd	a5,0(sp)
   11570:	020b6d93          	ori	s11,s6,32
   11574:	06600613          	li	a2,102
   11578:	40a686bb          	subw	a3,a3,a0
   1157c:	1ecd9063          	bne	s11,a2,1175c <fmt_fp+0xd7c>
   11580:	9466c2e3          	blt	a3,t1,10ec4 <fmt_fp+0x4e4>
   11584:	00605463          	blez	t1,1158c <fmt_fp+0xbac>
   11588:	0065053b          	addw	a0,a0,t1
   1158c:	80000737          	lui	a4,0x80000
   11590:	280a9663          	bnez	s5,1181c <fmt_fp+0xe3c>
   11594:	fff74713          	not	a4,a4
   11598:	01113c23          	sd	a7,24(sp)
   1159c:	0007071b          	sext.w	a4,a4
   115a0:	92a742e3          	blt	a4,a0,10ec4 <fmt_fp+0x4e4>
   115a4:	01550b3b          	addw	s6,a0,s5
   115a8:	000a0713          	mv	a4,s4
   115ac:	000b0693          	mv	a3,s6
   115b0:	00098613          	mv	a2,s3
   115b4:	02000593          	li	a1,32
   115b8:	00048513          	mv	a0,s1
   115bc:	01612823          	sw	s6,16(sp)
   115c0:	b78ff0ef          	jal	ra,10938 <pad>
   115c4:	000a8613          	mv	a2,s5
   115c8:	00090593          	mv	a1,s2
   115cc:	00048513          	mv	a0,s1
   115d0:	b44ff0ef          	jal	ra,10914 <out>
   115d4:	00013783          	ld	a5,0(sp)
   115d8:	00010737          	lui	a4,0x10
   115dc:	000b0693          	mv	a3,s6
   115e0:	00e7c733          	xor	a4,a5,a4
   115e4:	00098613          	mv	a2,s3
   115e8:	03000593          	li	a1,48
   115ec:	00048513          	mv	a0,s1
   115f0:	b48ff0ef          	jal	ra,10938 <pad>
   115f4:	06600713          	li	a4,102
   115f8:	01813883          	ld	a7,24(sp)
   115fc:	32ed9c63          	bne	s11,a4,11934 <fmt_fp+0xf54>
   11600:	00088913          	mv	s2,a7
   11604:	011bf463          	bgeu	s7,a7,1160c <fmt_fp+0xc2c>
   11608:	000b8913          	mv	s2,s7
   1160c:	00002737          	lui	a4,0x2
   11610:	ffffea37          	lui	s4,0xffffe
   11614:	04010693          	addi	a3,sp,64
   11618:	d1070793          	addi	a5,a4,-752 # 1d10 <exit-0xe410>
   1161c:	310a0b93          	addi	s7,s4,784 # ffffffffffffe310 <__BSS_END__+0xfffffffffffe5b08>
   11620:	00d787b3          	add	a5,a5,a3
   11624:	01778bb3          	add	s7,a5,s7
   11628:	d1070793          	addi	a5,a4,-752
   1162c:	00d787b3          	add	a5,a5,a3
   11630:	00090c93          	mv	s9,s2
   11634:	009b8d93          	addi	s11,s7,9
   11638:	03000a93          	li	s5,48
   1163c:	01478a33          	add	s4,a5,s4
   11640:	1f98f263          	bgeu	a7,s9,11824 <fmt_fp+0xe44>
   11644:	00188693          	addi	a3,a7,1
   11648:	ffd90713          	addi	a4,s2,-3
   1164c:	00000613          	li	a2,0
   11650:	00e6e863          	bltu	a3,a4,11660 <fmt_fp+0xc80>
   11654:	00488893          	addi	a7,a7,4
   11658:	412888b3          	sub	a7,a7,s2
   1165c:	ffc8f613          	andi	a2,a7,-4
   11660:	00c90933          	add	s2,s2,a2
   11664:	2c0c0463          	beqz	s8,1192c <fmt_fp+0xf4c>
   11668:	000165b7          	lui	a1,0x16
   1166c:	00100613          	li	a2,1
   11670:	f8058593          	addi	a1,a1,-128 # 15f80 <__clzdi2+0x100>
   11674:	00048513          	mv	a0,s1
   11678:	a9cff0ef          	jal	ra,10914 <out>
   1167c:	000027b7          	lui	a5,0x2
   11680:	ffffea37          	lui	s4,0xffffe
   11684:	d1078793          	addi	a5,a5,-752 # 1d10 <exit-0xe410>
   11688:	04010713          	addi	a4,sp,64
   1168c:	310a0a13          	addi	s4,s4,784 # ffffffffffffe310 <__BSS_END__+0xfffffffffffe5b08>
   11690:	00e787b3          	add	a5,a5,a4
   11694:	01478a33          	add	s4,a5,s4
   11698:	009a0a93          	addi	s5,s4,9
   1169c:	03000b93          	li	s7,48
   116a0:	00900c93          	li	s9,9
   116a4:	2280006f          	j	118cc <fmt_fp+0xeec>
   116a8:	000166b7          	lui	a3,0x16
   116ac:	0e86b683          	ld	a3,232(a3) # 160e8 <__clzdi2+0x268>
   116b0:	d95ff06f          	j	11444 <fmt_fp+0xa64>
   116b4:	01be0e3b          	addw	t3,t3,s11
   116b8:	3b9ad737          	lui	a4,0x3b9ad
   116bc:	01cc2023          	sw	t3,0(s8)
   116c0:	9ff70713          	addi	a4,a4,-1537 # 3b9ac9ff <__BSS_END__+0x3b9941f7>
   116c4:	000c2683          	lw	a3,0(s8)
   116c8:	02d76863          	bltu	a4,a3,116f8 <fmt_fp+0xd18>
   116cc:	41788333          	sub	t1,a7,s7
   116d0:	000ba683          	lw	a3,0(s7)
   116d4:	40235713          	srai	a4,t1,0x2
   116d8:	00900313          	li	t1,9
   116dc:	02e3033b          	mulw	t1,t1,a4
   116e0:	00a00613          	li	a2,10
   116e4:	00a00713          	li	a4,10
   116e8:	dce6e2e3          	bltu	a3,a4,114ac <fmt_fp+0xacc>
   116ec:	0013031b          	addiw	t1,t1,1
   116f0:	02e6073b          	mulw	a4,a2,a4
   116f4:	ff5ff06f          	j	116e8 <fmt_fp+0xd08>
   116f8:	ffcc0c13          	addi	s8,s8,-4
   116fc:	000c2223          	sw	zero,4(s8)
   11700:	017c7663          	bgeu	s8,s7,1170c <fmt_fp+0xd2c>
   11704:	fe0bae23          	sw	zero,-4(s7)
   11708:	ffcb8b93          	addi	s7,s7,-4
   1170c:	000c2683          	lw	a3,0(s8)
   11710:	0016869b          	addiw	a3,a3,1
   11714:	00dc2023          	sw	a3,0(s8)
   11718:	fadff06f          	j	116c4 <fmt_fp+0xce4>
   1171c:	ffcd0d13          	addi	s10,s10,-4
   11720:	d99ff06f          	j	114b8 <fmt_fp+0xad8>
   11724:	ffeb0b1b          	addiw	s6,s6,-2
   11728:	fff4041b          	addiw	s0,s0,-1
   1172c:	dc5ff06f          	j	114f0 <fmt_fp+0xb10>
   11730:	02e5073b          	mulw	a4,a0,a4
   11734:	0016869b          	addiw	a3,a3,1
   11738:	02e675bb          	remuw	a1,a2,a4
   1173c:	fe058ae3          	beqz	a1,11730 <fmt_fp+0xd50>
   11740:	dc9ff06f          	j	11508 <fmt_fp+0xb28>
   11744:	00000693          	li	a3,0
   11748:	00a00713          	li	a4,10
   1174c:	00a00513          	li	a0,10
   11750:	fe9ff06f          	j	11738 <fmt_fp+0xd58>
   11754:	00e30733          	add	a4,t1,a4
   11758:	dd1ff06f          	j	11528 <fmt_fp+0xb48>
   1175c:	00002737          	lui	a4,0x2
   11760:	02d13023          	sd	a3,32(sp)
   11764:	d1070713          	addi	a4,a4,-752 # 1d10 <exit-0xe410>
   11768:	04010693          	addi	a3,sp,64
   1176c:	00d70733          	add	a4,a4,a3
   11770:	ffffe7b7          	lui	a5,0xffffe
   11774:	41f3561b          	sraiw	a2,t1,0x1f
   11778:	00f707b3          	add	a5,a4,a5
   1177c:	30c78793          	addi	a5,a5,780 # ffffffffffffe30c <__BSS_END__+0xfffffffffffe5b04>
   11780:	02a12423          	sw	a0,40(sp)
   11784:	00c34533          	xor	a0,t1,a2
   11788:	00078593          	mv	a1,a5
   1178c:	40c5053b          	subw	a0,a0,a2
   11790:	01113c23          	sd	a7,24(sp)
   11794:	00613823          	sd	t1,16(sp)
   11798:	00f13423          	sd	a5,8(sp)
   1179c:	8e4ff0ef          	jal	ra,10880 <fmt_u>
   117a0:	00813783          	ld	a5,8(sp)
   117a4:	01013303          	ld	t1,16(sp)
   117a8:	01813883          	ld	a7,24(sp)
   117ac:	02013683          	ld	a3,32(sp)
   117b0:	02812703          	lw	a4,40(sp)
   117b4:	00100613          	li	a2,1
   117b8:	03000e13          	li	t3,48
   117bc:	40a785b3          	sub	a1,a5,a0
   117c0:	04b65863          	bge	a2,a1,11810 <fmt_fp+0xe30>
   117c4:	02d00613          	li	a2,45
   117c8:	00034463          	bltz	t1,117d0 <fmt_fp+0xdf0>
   117cc:	02b00613          	li	a2,43
   117d0:	ffe50793          	addi	a5,a0,-2
   117d4:	00f13423          	sd	a5,8(sp)
   117d8:	000027b7          	lui	a5,0x2
   117dc:	fec50fa3          	sb	a2,-1(a0)
   117e0:	d1078793          	addi	a5,a5,-752 # 1d10 <exit-0xe410>
   117e4:	04010613          	addi	a2,sp,64
   117e8:	00c787b3          	add	a5,a5,a2
   117ec:	ff650f23          	sb	s6,-2(a0)
   117f0:	ffffe537          	lui	a0,0xffffe
   117f4:	00a78533          	add	a0,a5,a0
   117f8:	00813783          	ld	a5,8(sp)
   117fc:	30c50513          	addi	a0,a0,780 # ffffffffffffe30c <__BSS_END__+0xfffffffffffe5b04>
   11800:	40f50533          	sub	a0,a0,a5
   11804:	eca6c063          	blt	a3,a0,10ec4 <fmt_fp+0x4e4>
   11808:	00a7053b          	addw	a0,a4,a0
   1180c:	d81ff06f          	j	1158c <fmt_fp+0xbac>
   11810:	fff50513          	addi	a0,a0,-1
   11814:	01c50023          	sb	t3,0(a0)
   11818:	fa5ff06f          	j	117bc <fmt_fp+0xddc>
   1181c:	ffe74713          	xori	a4,a4,-2
   11820:	d79ff06f          	j	11598 <fmt_fp+0xbb8>
   11824:	000ce503          	lwu	a0,0(s9)
   11828:	000d8593          	mv	a1,s11
   1182c:	01113423          	sd	a7,8(sp)
   11830:	850ff0ef          	jal	ra,10880 <fmt_u>
   11834:	00813883          	ld	a7,8(sp)
   11838:	00050593          	mv	a1,a0
   1183c:	00050713          	mv	a4,a0
   11840:	012c9e63          	bne	s9,s2,1185c <fmt_fp+0xe7c>
   11844:	03b51663          	bne	a0,s11,11870 <fmt_fp+0xe90>
   11848:	315a0c23          	sb	s5,792(s4)
   1184c:	008b8593          	addi	a1,s7,8
   11850:	0200006f          	j	11870 <fmt_fp+0xe90>
   11854:	fff70713          	addi	a4,a4,-1
   11858:	01570023          	sb	s5,0(a4)
   1185c:	feebece3          	bltu	s7,a4,11854 <fmt_fp+0xe74>
   11860:	00000713          	li	a4,0
   11864:	0175e463          	bltu	a1,s7,1186c <fmt_fp+0xe8c>
   11868:	40bb8733          	sub	a4,s7,a1
   1186c:	00e585b3          	add	a1,a1,a4
   11870:	40bd8633          	sub	a2,s11,a1
   11874:	00048513          	mv	a0,s1
   11878:	01113423          	sd	a7,8(sp)
   1187c:	898ff0ef          	jal	ra,10914 <out>
   11880:	00813883          	ld	a7,8(sp)
   11884:	004c8c93          	addi	s9,s9,4
   11888:	db9ff06f          	j	11640 <fmt_fp+0xc60>
   1188c:	00096503          	lwu	a0,0(s2)
   11890:	000a8593          	mv	a1,s5
   11894:	fedfe0ef          	jal	ra,10880 <fmt_u>
   11898:	00050713          	mv	a4,a0
   1189c:	08ea6263          	bltu	s4,a4,11920 <fmt_fp+0xf40>
   118a0:	00000593          	li	a1,0
   118a4:	01456463          	bltu	a0,s4,118ac <fmt_fp+0xecc>
   118a8:	40aa05b3          	sub	a1,s4,a0
   118ac:	0004061b          	sext.w	a2,s0
   118b0:	008cd463          	bge	s9,s0,118b8 <fmt_fp+0xed8>
   118b4:	00900613          	li	a2,9
   118b8:	00b505b3          	add	a1,a0,a1
   118bc:	00048513          	mv	a0,s1
   118c0:	854ff0ef          	jal	ra,10914 <out>
   118c4:	00490913          	addi	s2,s2,4
   118c8:	ff74041b          	addiw	s0,s0,-9
   118cc:	01a97463          	bgeu	s2,s10,118d4 <fmt_fp+0xef4>
   118d0:	fa804ee3          	bgtz	s0,1188c <fmt_fp+0xeac>
   118d4:	00000713          	li	a4,0
   118d8:	00900693          	li	a3,9
   118dc:	0094061b          	addiw	a2,s0,9
   118e0:	03000593          	li	a1,48
   118e4:	00048513          	mv	a0,s1
   118e8:	850ff0ef          	jal	ra,10938 <pad>
   118ec:	00013783          	ld	a5,0(sp)
   118f0:	00002737          	lui	a4,0x2
   118f4:	00048513          	mv	a0,s1
   118f8:	00e7c733          	xor	a4,a5,a4
   118fc:	000b0693          	mv	a3,s6
   11900:	00098613          	mv	a2,s3
   11904:	02000593          	li	a1,32
   11908:	830ff0ef          	jal	ra,10938 <pad>
   1190c:	01012503          	lw	a0,16(sp)
   11910:	013b5463          	bge	s6,s3,11918 <fmt_fp+0xf38>
   11914:	00098513          	mv	a0,s3
   11918:	0005051b          	sext.w	a0,a0
   1191c:	a24ff06f          	j	10b40 <fmt_fp+0x160>
   11920:	fff70713          	addi	a4,a4,-1 # 1fff <exit-0xe121>
   11924:	01770023          	sb	s7,0(a4)
   11928:	f75ff06f          	j	1189c <fmt_fp+0xebc>
   1192c:	00000413          	li	s0,0
   11930:	d4dff06f          	j	1167c <fmt_fp+0xc9c>
   11934:	01abe463          	bltu	s7,s10,1193c <fmt_fp+0xf5c>
   11938:	004b8d13          	addi	s10,s7,4
   1193c:	000026b7          	lui	a3,0x2
   11940:	ffffe737          	lui	a4,0xffffe
   11944:	04010613          	addi	a2,sp,64
   11948:	d1068793          	addi	a5,a3,-752 # 1d10 <exit-0xe410>
   1194c:	00c787b3          	add	a5,a5,a2
   11950:	31070913          	addi	s2,a4,784 # ffffffffffffe310 <__BSS_END__+0xfffffffffffe5b08>
   11954:	01278933          	add	s2,a5,s2
   11958:	d1068793          	addi	a5,a3,-752
   1195c:	00c787b3          	add	a5,a5,a2
   11960:	00e787b3          	add	a5,a5,a4
   11964:	00f13c23          	sd	a5,24(sp)
   11968:	00890793          	addi	a5,s2,8
   1196c:	000b8a13          	mv	s4,s7
   11970:	00990a93          	addi	s5,s2,9
   11974:	02f13023          	sd	a5,32(sp)
   11978:	01aa7463          	bgeu	s4,s10,11980 <fmt_fp+0xfa0>
   1197c:	04045863          	bgez	s0,119cc <fmt_fp+0xfec>
   11980:	0124061b          	addiw	a2,s0,18
   11984:	00048513          	mv	a0,s1
   11988:	00000713          	li	a4,0
   1198c:	01200693          	li	a3,18
   11990:	03000593          	li	a1,48
   11994:	fa5fe0ef          	jal	ra,10938 <pad>
   11998:	000027b7          	lui	a5,0x2
   1199c:	04010713          	addi	a4,sp,64
   119a0:	d1078793          	addi	a5,a5,-752 # 1d10 <exit-0xe410>
   119a4:	00e787b3          	add	a5,a5,a4
   119a8:	ffffe637          	lui	a2,0xffffe
   119ac:	00c78633          	add	a2,a5,a2
   119b0:	00813783          	ld	a5,8(sp)
   119b4:	30c60613          	addi	a2,a2,780 # ffffffffffffe30c <__BSS_END__+0xfffffffffffe5b04>
   119b8:	00048513          	mv	a0,s1
   119bc:	40f60633          	sub	a2,a2,a5
   119c0:	00078593          	mv	a1,a5
   119c4:	f51fe0ef          	jal	ra,10914 <out>
   119c8:	f25ff06f          	j	118ec <fmt_fp+0xf0c>
   119cc:	000a6503          	lwu	a0,0(s4)
   119d0:	000a8593          	mv	a1,s5
   119d4:	eadfe0ef          	jal	ra,10880 <fmt_u>
   119d8:	00050593          	mv	a1,a0
   119dc:	01551a63          	bne	a0,s5,119f0 <fmt_fp+0x1010>
   119e0:	01813783          	ld	a5,24(sp)
   119e4:	02013583          	ld	a1,32(sp)
   119e8:	03000713          	li	a4,48
   119ec:	30e78c23          	sb	a4,792(a5)
   119f0:	00040d9b          	sext.w	s11,s0
   119f4:	00058713          	mv	a4,a1
   119f8:	057a1063          	bne	s4,s7,11a38 <fmt_fp+0x1058>
   119fc:	00100613          	li	a2,1
   11a00:	00048513          	mv	a0,s1
   11a04:	00158c13          	addi	s8,a1,1
   11a08:	f0dfe0ef          	jal	ra,10914 <out>
   11a0c:	01bce733          	or	a4,s9,s11
   11a10:	02070e63          	beqz	a4,11a4c <fmt_fp+0x106c>
   11a14:	000167b7          	lui	a5,0x16
   11a18:	00100613          	li	a2,1
   11a1c:	f8078593          	addi	a1,a5,-128 # 15f80 <__clzdi2+0x100>
   11a20:	00048513          	mv	a0,s1
   11a24:	ef1fe0ef          	jal	ra,10914 <out>
   11a28:	0240006f          	j	11a4c <fmt_fp+0x106c>
   11a2c:	fff70713          	addi	a4,a4,-1
   11a30:	03000793          	li	a5,48
   11a34:	00f70023          	sb	a5,0(a4)
   11a38:	fee96ae3          	bltu	s2,a4,11a2c <fmt_fp+0x104c>
   11a3c:	00000693          	li	a3,0
   11a40:	0125e463          	bltu	a1,s2,11a48 <fmt_fp+0x1068>
   11a44:	40b906b3          	sub	a3,s2,a1
   11a48:	00d58c33          	add	s8,a1,a3
   11a4c:	418a88b3          	sub	a7,s5,s8
   11a50:	00040613          	mv	a2,s0
   11a54:	0088d463          	bge	a7,s0,11a5c <fmt_fp+0x107c>
   11a58:	00088613          	mv	a2,a7
   11a5c:	000c0593          	mv	a1,s8
   11a60:	00048513          	mv	a0,s1
   11a64:	03113423          	sd	a7,40(sp)
   11a68:	eadfe0ef          	jal	ra,10914 <out>
   11a6c:	02813883          	ld	a7,40(sp)
   11a70:	004a0a13          	addi	s4,s4,4
   11a74:	411d843b          	subw	s0,s11,a7
   11a78:	f01ff06f          	j	11978 <fmt_fp+0xf98>

0000000000011a7c <printf_core>:
   11a7c:	ef010113          	addi	sp,sp,-272
   11a80:	000167b7          	lui	a5,0x16
   11a84:	12078793          	addi	a5,a5,288 # 16120 <states>
   11a88:	0ba13823          	sd	s10,176(sp)
   11a8c:	00016d37          	lui	s10,0x16
   11a90:	0f313423          	sd	s3,232(sp)
   11a94:	00f13823          	sd	a5,16(sp)
   11a98:	000169b7          	lui	s3,0x16
   11a9c:	fa0d0793          	addi	a5,s10,-96 # 15fa0 <__clzdi2+0x120>
   11aa0:	10813023          	sd	s0,256(sp)
   11aa4:	0f213823          	sd	s2,240(sp)
   11aa8:	0f413023          	sd	s4,224(sp)
   11aac:	0d613823          	sd	s6,208(sp)
   11ab0:	0d813023          	sd	s8,192(sp)
   11ab4:	0b913c23          	sd	s9,184(sp)
   11ab8:	10113423          	sd	ra,264(sp)
   11abc:	0e913c23          	sd	s1,248(sp)
   11ac0:	0d513c23          	sd	s5,216(sp)
   11ac4:	0d713423          	sd	s7,200(sp)
   11ac8:	0bb13423          	sd	s11,168(sp)
   11acc:	00050413          	mv	s0,a0
   11ad0:	00060a13          	mv	s4,a2
   11ad4:	00d13023          	sd	a3,0(sp)
   11ad8:	00070b13          	mv	s6,a4
   11adc:	04b13023          	sd	a1,64(sp)
   11ae0:	00000c93          	li	s9,0
   11ae4:	00000913          	li	s2,0
   11ae8:	00000c13          	li	s8,0
   11aec:	f8898993          	addi	s3,s3,-120 # 15f88 <__clzdi2+0x108>
   11af0:	00f13c23          	sd	a5,24(sp)
   11af4:	04013b83          	ld	s7,64(sp)
   11af8:	012c893b          	addw	s2,s9,s2
   11afc:	00090d93          	mv	s11,s2
   11b00:	000bc783          	lbu	a5,0(s7)
   11b04:	02078ee3          	beqz	a5,12340 <printf_core+0x8c4>
   11b08:	02500713          	li	a4,37
   11b0c:	04013303          	ld	t1,64(sp)
   11b10:	00034783          	lbu	a5,0(t1)
   11b14:	00078463          	beqz	a5,11b1c <printf_core+0xa0>
   11b18:	00e79663          	bne	a5,a4,11b24 <printf_core+0xa8>
   11b1c:	02500713          	li	a4,37
   11b20:	01c0006f          	j	11b3c <printf_core+0xc0>
   11b24:	00130313          	addi	t1,t1,1
   11b28:	04613023          	sd	t1,64(sp)
   11b2c:	fe1ff06f          	j	11b0c <printf_core+0x90>
   11b30:	00278793          	addi	a5,a5,2
   11b34:	00130313          	addi	t1,t1,1
   11b38:	04f13023          	sd	a5,64(sp)
   11b3c:	04013783          	ld	a5,64(sp)
   11b40:	0007c683          	lbu	a3,0(a5)
   11b44:	00e69663          	bne	a3,a4,11b50 <printf_core+0xd4>
   11b48:	0017c683          	lbu	a3,1(a5)
   11b4c:	fee682e3          	beq	a3,a4,11b30 <printf_core+0xb4>
   11b50:	80000ab7          	lui	s5,0x80000
   11b54:	fffaca93          	not	s5,s5
   11b58:	41ba87bb          	subw	a5,s5,s11
   11b5c:	41730333          	sub	t1,t1,s7
   11b60:	00f13423          	sd	a5,8(sp)
   11b64:	7467ca63          	blt	a5,t1,122b8 <printf_core+0x83c>
   11b68:	00030c9b          	sext.w	s9,t1
   11b6c:	00040a63          	beqz	s0,11b80 <printf_core+0x104>
   11b70:	000c8613          	mv	a2,s9
   11b74:	000b8593          	mv	a1,s7
   11b78:	00040513          	mv	a0,s0
   11b7c:	d99fe0ef          	jal	ra,10914 <out>
   11b80:	720c9863          	bnez	s9,122b0 <printf_core+0x834>
   11b84:	04013783          	ld	a5,64(sp)
   11b88:	00900713          	li	a4,9
   11b8c:	0017c683          	lbu	a3,1(a5)
   11b90:	fd06869b          	addiw	a3,a3,-48
   11b94:	18d76663          	bltu	a4,a3,11d20 <printf_core+0x2a4>
   11b98:	0027c603          	lbu	a2,2(a5)
   11b9c:	02400713          	li	a4,36
   11ba0:	18e61063          	bne	a2,a4,11d20 <printf_core+0x2a4>
   11ba4:	00378793          	addi	a5,a5,3
   11ba8:	00100c13          	li	s8,1
   11bac:	00013537          	lui	a0,0x13
   11bb0:	04f13023          	sd	a5,64(sp)
   11bb4:	00000d13          	li	s10,0
   11bb8:	01f00893          	li	a7,31
   11bbc:	8895051b          	addiw	a0,a0,-1911
   11bc0:	00100e93          	li	t4,1
   11bc4:	04013703          	ld	a4,64(sp)
   11bc8:	00074803          	lbu	a6,0(a4)
   11bcc:	fe08061b          	addiw	a2,a6,-32
   11bd0:	00c8e863          	bltu	a7,a2,11be0 <printf_core+0x164>
   11bd4:	00c555bb          	srlw	a1,a0,a2
   11bd8:	0015f593          	andi	a1,a1,1
   11bdc:	14059863          	bnez	a1,11d2c <printf_core+0x2b0>
   11be0:	02a00613          	li	a2,42
   11be4:	18c81e63          	bne	a6,a2,11d80 <printf_core+0x304>
   11be8:	00174603          	lbu	a2,1(a4)
   11bec:	00900593          	li	a1,9
   11bf0:	fd06051b          	addiw	a0,a2,-48
   11bf4:	14a5e863          	bltu	a1,a0,11d44 <printf_core+0x2c8>
   11bf8:	00274503          	lbu	a0,2(a4)
   11bfc:	02400593          	li	a1,36
   11c00:	14b51263          	bne	a0,a1,11d44 <printf_core+0x2c8>
   11c04:	00261613          	slli	a2,a2,0x2
   11c08:	00cb0633          	add	a2,s6,a2
   11c0c:	00a00593          	li	a1,10
   11c10:	f4b62023          	sw	a1,-192(a2)
   11c14:	00174603          	lbu	a2,1(a4)
   11c18:	00013783          	ld	a5,0(sp)
   11c1c:	00370713          	addi	a4,a4,3
   11c20:	00461613          	slli	a2,a2,0x4
   11c24:	00c78633          	add	a2,a5,a2
   11c28:	d0062483          	lw	s1,-768(a2)
   11c2c:	04e13023          	sd	a4,64(sp)
   11c30:	00100c13          	li	s8,1
   11c34:	0004d863          	bgez	s1,11c44 <printf_core+0x1c8>
   11c38:	00002737          	lui	a4,0x2
   11c3c:	00ed6d33          	or	s10,s10,a4
   11c40:	409004bb          	negw	s1,s1
   11c44:	04013703          	ld	a4,64(sp)
   11c48:	02e00613          	li	a2,46
   11c4c:	00074583          	lbu	a1,0(a4) # 2000 <exit-0xe120>
   11c50:	18c59a63          	bne	a1,a2,11de4 <printf_core+0x368>
   11c54:	00174583          	lbu	a1,1(a4)
   11c58:	02a00613          	li	a2,42
   11c5c:	16c59263          	bne	a1,a2,11dc0 <printf_core+0x344>
   11c60:	00274603          	lbu	a2,2(a4)
   11c64:	00900593          	li	a1,9
   11c68:	fd06051b          	addiw	a0,a2,-48
   11c6c:	12a5e863          	bltu	a1,a0,11d9c <printf_core+0x320>
   11c70:	00374503          	lbu	a0,3(a4)
   11c74:	02400593          	li	a1,36
   11c78:	12b51263          	bne	a0,a1,11d9c <printf_core+0x320>
   11c7c:	00261613          	slli	a2,a2,0x2
   11c80:	00cb0633          	add	a2,s6,a2
   11c84:	00a00593          	li	a1,10
   11c88:	f4b62023          	sw	a1,-192(a2)
   11c8c:	00274603          	lbu	a2,2(a4)
   11c90:	00013783          	ld	a5,0(sp)
   11c94:	00470713          	addi	a4,a4,4
   11c98:	00461613          	slli	a2,a2,0x4
   11c9c:	00c78633          	add	a2,a5,a2
   11ca0:	d0062a83          	lw	s5,-768(a2)
   11ca4:	fffacf13          	not	t5,s5
   11ca8:	04e13023          	sd	a4,64(sp)
   11cac:	01ff5f1b          	srliw	t5,t5,0x1f
   11cb0:	00000713          	li	a4,0
   11cb4:	03900e93          	li	t4,57
   11cb8:	03a00813          	li	a6,58
   11cbc:	00700513          	li	a0,7
   11cc0:	04013603          	ld	a2,64(sp)
   11cc4:	00064583          	lbu	a1,0(a2)
   11cc8:	fbf5859b          	addiw	a1,a1,-65
   11ccc:	06beee63          	bltu	t4,a1,11d48 <printf_core+0x2cc>
   11cd0:	00160593          	addi	a1,a2,1
   11cd4:	04b13023          	sd	a1,64(sp)
   11cd8:	00064603          	lbu	a2,0(a2)
   11cdc:	01013783          	ld	a5,16(sp)
   11ce0:	fbf6059b          	addiw	a1,a2,-65
   11ce4:	02071613          	slli	a2,a4,0x20
   11ce8:	02065613          	srli	a2,a2,0x20
   11cec:	03060633          	mul	a2,a2,a6
   11cf0:	00c78633          	add	a2,a5,a2
   11cf4:	00b60633          	add	a2,a2,a1
   11cf8:	00064f83          	lbu	t6,0(a2)
   11cfc:	ffff861b          	addiw	a2,t6,-1
   11d00:	000f859b          	sext.w	a1,t6
   11d04:	0ec57663          	bgeu	a0,a2,11df0 <printf_core+0x374>
   11d08:	04058063          	beqz	a1,11d48 <printf_core+0x2cc>
   11d0c:	01b00613          	li	a2,27
   11d10:	0ec59463          	bne	a1,a2,11df8 <printf_core+0x37c>
   11d14:	0206da63          	bgez	a3,11d48 <printf_core+0x2cc>
   11d18:	16041a63          	bnez	s0,11e8c <printf_core+0x410>
   11d1c:	dd9ff06f          	j	11af4 <printf_core+0x78>
   11d20:	00178793          	addi	a5,a5,1
   11d24:	fff00693          	li	a3,-1
   11d28:	e85ff06f          	j	11bac <printf_core+0x130>
   11d2c:	00ce963b          	sllw	a2,t4,a2
   11d30:	00cd67b3          	or	a5,s10,a2
   11d34:	00170713          	addi	a4,a4,1
   11d38:	00078d1b          	sext.w	s10,a5
   11d3c:	04e13023          	sd	a4,64(sp)
   11d40:	e85ff06f          	j	11bc4 <printf_core+0x148>
   11d44:	000c0c63          	beqz	s8,11d5c <printf_core+0x2e0>
   11d48:	41d000ef          	jal	ra,12964 <__errno_location>
   11d4c:	01600793          	li	a5,22
   11d50:	00f52023          	sw	a5,0(a0) # 13000 <__towrite+0x4c>
   11d54:	fff00913          	li	s2,-1
   11d58:	0d80006f          	j	11e30 <printf_core+0x3b4>
   11d5c:	00000493          	li	s1,0
   11d60:	00040a63          	beqz	s0,11d74 <printf_core+0x2f8>
   11d64:	000a3603          	ld	a2,0(s4)
   11d68:	00062483          	lw	s1,0(a2)
   11d6c:	00860593          	addi	a1,a2,8
   11d70:	00ba3023          	sd	a1,0(s4)
   11d74:	00170713          	addi	a4,a4,1
   11d78:	04e13023          	sd	a4,64(sp)
   11d7c:	eb9ff06f          	j	11c34 <printf_core+0x1b8>
   11d80:	04010513          	addi	a0,sp,64
   11d84:	02d13023          	sd	a3,32(sp)
   11d88:	b25fe0ef          	jal	ra,108ac <getint>
   11d8c:	02013683          	ld	a3,32(sp)
   11d90:	00050493          	mv	s1,a0
   11d94:	ea0558e3          	bgez	a0,11c44 <printf_core+0x1c8>
   11d98:	5200006f          	j	122b8 <printf_core+0x83c>
   11d9c:	fa0c16e3          	bnez	s8,11d48 <printf_core+0x2cc>
   11da0:	00000a93          	li	s5,0
   11da4:	00040a63          	beqz	s0,11db8 <printf_core+0x33c>
   11da8:	000a3603          	ld	a2,0(s4)
   11dac:	00062a83          	lw	s5,0(a2)
   11db0:	00860593          	addi	a1,a2,8
   11db4:	00ba3023          	sd	a1,0(s4)
   11db8:	00270713          	addi	a4,a4,2
   11dbc:	ee9ff06f          	j	11ca4 <printf_core+0x228>
   11dc0:	00170713          	addi	a4,a4,1
   11dc4:	04010513          	addi	a0,sp,64
   11dc8:	02d13023          	sd	a3,32(sp)
   11dcc:	04e13023          	sd	a4,64(sp)
   11dd0:	addfe0ef          	jal	ra,108ac <getint>
   11dd4:	02013683          	ld	a3,32(sp)
   11dd8:	00050a93          	mv	s5,a0
   11ddc:	00100f13          	li	t5,1
   11de0:	ed1ff06f          	j	11cb0 <printf_core+0x234>
   11de4:	00000f13          	li	t5,0
   11de8:	fff00a93          	li	s5,-1
   11dec:	ec5ff06f          	j	11cb0 <printf_core+0x234>
   11df0:	00058713          	mv	a4,a1
   11df4:	ecdff06f          	j	11cc0 <printf_core+0x244>
   11df8:	0206c863          	bltz	a3,11e28 <printf_core+0x3ac>
   11dfc:	00013783          	ld	a5,0(sp)
   11e00:	00269613          	slli	a2,a3,0x2
   11e04:	00469693          	slli	a3,a3,0x4
   11e08:	00cb0633          	add	a2,s6,a2
   11e0c:	00d786b3          	add	a3,a5,a3
   11e10:	01f62023          	sw	t6,0(a2)
   11e14:	0006b603          	ld	a2,0(a3)
   11e18:	0086b683          	ld	a3,8(a3)
   11e1c:	04c13823          	sd	a2,80(sp)
   11e20:	04d13c23          	sd	a3,88(sp)
   11e24:	ef5ff06f          	j	11d18 <printf_core+0x29c>
   11e28:	04041463          	bnez	s0,11e70 <printf_core+0x3f4>
   11e2c:	00000913          	li	s2,0
   11e30:	10813083          	ld	ra,264(sp)
   11e34:	10013403          	ld	s0,256(sp)
   11e38:	0f813483          	ld	s1,248(sp)
   11e3c:	0e813983          	ld	s3,232(sp)
   11e40:	0e013a03          	ld	s4,224(sp)
   11e44:	0d813a83          	ld	s5,216(sp)
   11e48:	0d013b03          	ld	s6,208(sp)
   11e4c:	0c813b83          	ld	s7,200(sp)
   11e50:	0c013c03          	ld	s8,192(sp)
   11e54:	0b813c83          	ld	s9,184(sp)
   11e58:	0b013d03          	ld	s10,176(sp)
   11e5c:	0a813d83          	ld	s11,168(sp)
   11e60:	00090513          	mv	a0,s2
   11e64:	0f013903          	ld	s2,240(sp)
   11e68:	11010113          	addi	sp,sp,272
   11e6c:	00008067          	ret
   11e70:	000a0613          	mv	a2,s4
   11e74:	05010513          	addi	a0,sp,80
   11e78:	02e13423          	sd	a4,40(sp)
   11e7c:	03e13023          	sd	t5,32(sp)
   11e80:	8fdfe0ef          	jal	ra,1077c <pop_arg>
   11e84:	02013f03          	ld	t5,32(sp)
   11e88:	02813703          	ld	a4,40(sp)
   11e8c:	04013683          	ld	a3,64(sp)
   11e90:	fff6c683          	lbu	a3,-1(a3)
   11e94:	0006881b          	sext.w	a6,a3
   11e98:	00070a63          	beqz	a4,11eac <printf_core+0x430>
   11e9c:	00f6f693          	andi	a3,a3,15
   11ea0:	00300613          	li	a2,3
   11ea4:	00c69463          	bne	a3,a2,11eac <printf_core+0x430>
   11ea8:	0df87813          	andi	a6,a6,223
   11eac:	00dd5693          	srli	a3,s10,0xd
   11eb0:	0016f693          	andi	a3,a3,1
   11eb4:	00068863          	beqz	a3,11ec4 <printf_core+0x448>
   11eb8:	ffff06b7          	lui	a3,0xffff0
   11ebc:	fff68693          	addi	a3,a3,-1 # fffffffffffeffff <__BSS_END__+0xfffffffffffd77f7>
   11ec0:	00dd7d33          	and	s10,s10,a3
   11ec4:	fbf8069b          	addiw	a3,a6,-65
   11ec8:	0006859b          	sext.w	a1,a3
   11ecc:	03700613          	li	a2,55
   11ed0:	44b66e63          	bltu	a2,a1,1232c <printf_core+0x8b0>
   11ed4:	01813783          	ld	a5,24(sp)
   11ed8:	02069693          	slli	a3,a3,0x20
   11edc:	01e6d693          	srli	a3,a3,0x1e
   11ee0:	00f686b3          	add	a3,a3,a5
   11ee4:	0006a683          	lw	a3,0(a3)
   11ee8:	00068067          	jr	a3
   11eec:	00700793          	li	a5,7
   11ef0:	3ce7e063          	bltu	a5,a4,122b0 <printf_core+0x834>
   11ef4:	00271793          	slli	a5,a4,0x2
   11ef8:	00016737          	lui	a4,0x16
   11efc:	08070713          	addi	a4,a4,128 # 16080 <__clzdi2+0x200>
   11f00:	00e787b3          	add	a5,a5,a4
   11f04:	0007a783          	lw	a5,0(a5)
   11f08:	00078067          	jr	a5
   11f0c:	05013783          	ld	a5,80(sp)
   11f10:	01b7a023          	sw	s11,0(a5)
   11f14:	be1ff06f          	j	11af4 <printf_core+0x78>
   11f18:	05013783          	ld	a5,80(sp)
   11f1c:	01279023          	sh	s2,0(a5)
   11f20:	bd5ff06f          	j	11af4 <printf_core+0x78>
   11f24:	05013783          	ld	a5,80(sp)
   11f28:	01278023          	sb	s2,0(a5)
   11f2c:	bc9ff06f          	j	11af4 <printf_core+0x78>
   11f30:	05013783          	ld	a5,80(sp)
   11f34:	0127b023          	sd	s2,0(a5)
   11f38:	bbdff06f          	j	11af4 <printf_core+0x78>
   11f3c:	000a889b          	sext.w	a7,s5
   11f40:	01000693          	li	a3,16
   11f44:	000a8713          	mv	a4,s5
   11f48:	00d8f463          	bgeu	a7,a3,11f50 <printf_core+0x4d4>
   11f4c:	01000713          	li	a4,16
   11f50:	00070a9b          	sext.w	s5,a4
   11f54:	008d6d13          	ori	s10,s10,8
   11f58:	07800813          	li	a6,120
   11f5c:	05013583          	ld	a1,80(sp)
   11f60:	000166b7          	lui	a3,0x16
   11f64:	02087513          	andi	a0,a6,32
   11f68:	00058713          	mv	a4,a1
   11f6c:	09f10b93          	addi	s7,sp,159
   11f70:	2f068693          	addi	a3,a3,752 # 162f0 <xdigits>
   11f74:	12071c63          	bnez	a4,120ac <printf_core+0x630>
   11f78:	00098e93          	mv	t4,s3
   11f7c:	00058c63          	beqz	a1,11f94 <printf_core+0x518>
   11f80:	008d7713          	andi	a4,s10,8
   11f84:	00070863          	beqz	a4,11f94 <printf_core+0x518>
   11f88:	00485813          	srli	a6,a6,0x4
   11f8c:	01098eb3          	add	t4,s3,a6
   11f90:	00200c93          	li	s9,2
   11f94:	000f0a63          	beqz	t5,11fa8 <printf_core+0x52c>
   11f98:	320ac063          	bltz	s5,122b8 <printf_core+0x83c>
   11f9c:	ffff0737          	lui	a4,0xffff0
   11fa0:	fff70713          	addi	a4,a4,-1 # fffffffffffeffff <__BSS_END__+0xfffffffffffd77f7>
   11fa4:	00ed7d33          	and	s10,s10,a4
   11fa8:	05013683          	ld	a3,80(sp)
   11fac:	09f10613          	addi	a2,sp,159
   11fb0:	00069463          	bnez	a3,11fb8 <printf_core+0x53c>
   11fb4:	380a8263          	beqz	s5,12338 <printf_core+0x8bc>
   11fb8:	41760733          	sub	a4,a2,s7
   11fbc:	0016b693          	seqz	a3,a3
   11fc0:	00d70733          	add	a4,a4,a3
   11fc4:	01575463          	bge	a4,s5,11fcc <printf_core+0x550>
   11fc8:	000a8713          	mv	a4,s5
   11fcc:	00070a9b          	sext.w	s5,a4
   11fd0:	41760db3          	sub	s11,a2,s7
   11fd4:	01bad463          	bge	s5,s11,11fdc <printf_core+0x560>
   11fd8:	000d8a9b          	sext.w	s5,s11
   11fdc:	80000737          	lui	a4,0x80000
   11fe0:	fff74713          	not	a4,a4
   11fe4:	4197073b          	subw	a4,a4,s9
   11fe8:	2d574863          	blt	a4,s5,122b8 <printf_core+0x83c>
   11fec:	019a883b          	addw	a6,s5,s9
   11ff0:	00080713          	mv	a4,a6
   11ff4:	00985463          	bge	a6,s1,11ffc <printf_core+0x580>
   11ff8:	00048713          	mv	a4,s1
   11ffc:	00813783          	ld	a5,8(sp)
   12000:	03d13023          	sd	t4,32(sp)
   12004:	0007049b          	sext.w	s1,a4
   12008:	2a97c863          	blt	a5,s1,122b8 <printf_core+0x83c>
   1200c:	00080693          	mv	a3,a6
   12010:	000d0713          	mv	a4,s10
   12014:	00048613          	mv	a2,s1
   12018:	02000593          	li	a1,32
   1201c:	00040513          	mv	a0,s0
   12020:	01013423          	sd	a6,8(sp)
   12024:	915fe0ef          	jal	ra,10938 <pad>
   12028:	02013e83          	ld	t4,32(sp)
   1202c:	000c8613          	mv	a2,s9
   12030:	00040513          	mv	a0,s0
   12034:	000e8593          	mv	a1,t4
   12038:	8ddfe0ef          	jal	ra,10914 <out>
   1203c:	00813803          	ld	a6,8(sp)
   12040:	00010737          	lui	a4,0x10
   12044:	00ed4733          	xor	a4,s10,a4
   12048:	00080693          	mv	a3,a6
   1204c:	00048613          	mv	a2,s1
   12050:	03000593          	li	a1,48
   12054:	00040513          	mv	a0,s0
   12058:	8e1fe0ef          	jal	ra,10938 <pad>
   1205c:	00000713          	li	a4,0
   12060:	000d869b          	sext.w	a3,s11
   12064:	000a8613          	mv	a2,s5
   12068:	03000593          	li	a1,48
   1206c:	00040513          	mv	a0,s0
   12070:	8c9fe0ef          	jal	ra,10938 <pad>
   12074:	000d8613          	mv	a2,s11
   12078:	000b8593          	mv	a1,s7
   1207c:	00040513          	mv	a0,s0
   12080:	895fe0ef          	jal	ra,10914 <out>
   12084:	00813803          	ld	a6,8(sp)
   12088:	00002737          	lui	a4,0x2
   1208c:	00ed4733          	xor	a4,s10,a4
   12090:	00080693          	mv	a3,a6
   12094:	00048613          	mv	a2,s1
   12098:	02000593          	li	a1,32
   1209c:	00040513          	mv	a0,s0
   120a0:	899fe0ef          	jal	ra,10938 <pad>
   120a4:	00048c93          	mv	s9,s1
   120a8:	a4dff06f          	j	11af4 <printf_core+0x78>
   120ac:	00f77613          	andi	a2,a4,15
   120b0:	00c68633          	add	a2,a3,a2
   120b4:	00064603          	lbu	a2,0(a2)
   120b8:	fffb8b93          	addi	s7,s7,-1
   120bc:	00475713          	srli	a4,a4,0x4
   120c0:	00c56633          	or	a2,a0,a2
   120c4:	00cb8023          	sb	a2,0(s7)
   120c8:	eadff06f          	j	11f74 <printf_core+0x4f8>
   120cc:	05013683          	ld	a3,80(sp)
   120d0:	09f10b93          	addi	s7,sp,159
   120d4:	000b8713          	mv	a4,s7
   120d8:	02069063          	bnez	a3,120f8 <printf_core+0x67c>
   120dc:	008d7693          	andi	a3,s10,8
   120e0:	00098e93          	mv	t4,s3
   120e4:	ea0688e3          	beqz	a3,11f94 <printf_core+0x518>
   120e8:	41770733          	sub	a4,a4,s7
   120ec:	eb5744e3          	blt	a4,s5,11f94 <printf_core+0x518>
   120f0:	00170a9b          	addiw	s5,a4,1
   120f4:	ea1ff06f          	j	11f94 <printf_core+0x518>
   120f8:	0076f613          	andi	a2,a3,7
   120fc:	fffb8b93          	addi	s7,s7,-1
   12100:	0306061b          	addiw	a2,a2,48
   12104:	00cb8023          	sb	a2,0(s7)
   12108:	0036d693          	srli	a3,a3,0x3
   1210c:	fcdff06f          	j	120d8 <printf_core+0x65c>
   12110:	05013703          	ld	a4,80(sp)
   12114:	02075c63          	bgez	a4,1214c <printf_core+0x6d0>
   12118:	40e00733          	neg	a4,a4
   1211c:	04e13823          	sd	a4,80(sp)
   12120:	00100c93          	li	s9,1
   12124:	00098e93          	mv	t4,s3
   12128:	05013503          	ld	a0,80(sp)
   1212c:	09f10593          	addi	a1,sp,159
   12130:	03d13423          	sd	t4,40(sp)
   12134:	03e13023          	sd	t5,32(sp)
   12138:	f48fe0ef          	jal	ra,10880 <fmt_u>
   1213c:	02813e83          	ld	t4,40(sp)
   12140:	02013f03          	ld	t5,32(sp)
   12144:	00050b93          	mv	s7,a0
   12148:	e4dff06f          	j	11f94 <printf_core+0x518>
   1214c:	00bd5713          	srli	a4,s10,0xb
   12150:	00177713          	andi	a4,a4,1
   12154:	00071e63          	bnez	a4,12170 <printf_core+0x6f4>
   12158:	001d7713          	andi	a4,s10,1
   1215c:	fc0704e3          	beqz	a4,12124 <printf_core+0x6a8>
   12160:	00016eb7          	lui	t4,0x16
   12164:	00100c93          	li	s9,1
   12168:	f8ae8e93          	addi	t4,t4,-118 # 15f8a <__clzdi2+0x10a>
   1216c:	fbdff06f          	j	12128 <printf_core+0x6ac>
   12170:	00016eb7          	lui	t4,0x16
   12174:	00100c93          	li	s9,1
   12178:	f89e8e93          	addi	t4,t4,-119 # 15f89 <__clzdi2+0x109>
   1217c:	fadff06f          	j	12128 <printf_core+0x6ac>
   12180:	05013703          	ld	a4,80(sp)
   12184:	00098e93          	mv	t4,s3
   12188:	00100a93          	li	s5,1
   1218c:	08e10f23          	sb	a4,158(sp)
   12190:	ffff0737          	lui	a4,0xffff0
   12194:	fff70713          	addi	a4,a4,-1 # fffffffffffeffff <__BSS_END__+0xfffffffffffd77f7>
   12198:	00ed7d33          	and	s10,s10,a4
   1219c:	09f10613          	addi	a2,sp,159
   121a0:	09e10b93          	addi	s7,sp,158
   121a4:	e2dff06f          	j	11fd0 <printf_core+0x554>
   121a8:	7bc000ef          	jal	ra,12964 <__errno_location>
   121ac:	00052503          	lw	a0,0(a0)
   121b0:	00d000ef          	jal	ra,129bc <strerror>
   121b4:	00050b93          	mv	s7,a0
   121b8:	000a8593          	mv	a1,s5
   121bc:	000ad663          	bgez	s5,121c8 <printf_core+0x74c>
   121c0:	800005b7          	lui	a1,0x80000
   121c4:	fff5c593          	not	a1,a1
   121c8:	000b8513          	mv	a0,s7
   121cc:	448000ef          	jal	ra,12614 <strnlen>
   121d0:	00ab8633          	add	a2,s7,a0
   121d4:	000ad663          	bgez	s5,121e0 <printf_core+0x764>
   121d8:	00064703          	lbu	a4,0(a2)
   121dc:	0c071e63          	bnez	a4,122b8 <printf_core+0x83c>
   121e0:	ffff0737          	lui	a4,0xffff0
   121e4:	fff70713          	addi	a4,a4,-1 # fffffffffffeffff <__BSS_END__+0xfffffffffffd77f7>
   121e8:	00050a9b          	sext.w	s5,a0
   121ec:	00ed7d33          	and	s10,s10,a4
   121f0:	00098e93          	mv	t4,s3
   121f4:	dddff06f          	j	11fd0 <printf_core+0x554>
   121f8:	05013b83          	ld	s7,80(sp)
   121fc:	fa0b9ee3          	bnez	s7,121b8 <printf_core+0x73c>
   12200:	00016e37          	lui	t3,0x16
   12204:	f98e0b93          	addi	s7,t3,-104 # 15f98 <__clzdi2+0x118>
   12208:	fb1ff06f          	j	121b8 <printf_core+0x73c>
   1220c:	05013703          	ld	a4,80(sp)
   12210:	04012623          	sw	zero,76(sp)
   12214:	fff00a93          	li	s5,-1
   12218:	04e12423          	sw	a4,72(sp)
   1221c:	04810713          	addi	a4,sp,72
   12220:	04e13823          	sd	a4,80(sp)
   12224:	05013d83          	ld	s11,80(sp)
   12228:	00000c93          	li	s9,0
   1222c:	035cf263          	bgeu	s9,s5,12250 <printf_core+0x7d4>
   12230:	000da583          	lw	a1,0(s11)
   12234:	00058e63          	beqz	a1,12250 <printf_core+0x7d4>
   12238:	03810513          	addi	a0,sp,56
   1223c:	004d8d93          	addi	s11,s11,4
   12240:	119000ef          	jal	ra,12b58 <wctomb>
   12244:	b00548e3          	bltz	a0,11d54 <printf_core+0x2d8>
   12248:	419a8733          	sub	a4,s5,s9
   1224c:	06a77c63          	bgeu	a4,a0,122c4 <printf_core+0x848>
   12250:	80000737          	lui	a4,0x80000
   12254:	fff74713          	not	a4,a4
   12258:	07976063          	bltu	a4,s9,122b8 <printf_core+0x83c>
   1225c:	000c8d9b          	sext.w	s11,s9
   12260:	000d0713          	mv	a4,s10
   12264:	000d8693          	mv	a3,s11
   12268:	00048613          	mv	a2,s1
   1226c:	02000593          	li	a1,32
   12270:	00040513          	mv	a0,s0
   12274:	ec4fe0ef          	jal	ra,10938 <pad>
   12278:	05013b83          	ld	s7,80(sp)
   1227c:	00000a93          	li	s5,0
   12280:	059ae663          	bltu	s5,s9,122cc <printf_core+0x850>
   12284:	00002737          	lui	a4,0x2
   12288:	00ed4733          	xor	a4,s10,a4
   1228c:	000d8693          	mv	a3,s11
   12290:	00048613          	mv	a2,s1
   12294:	02000593          	li	a1,32
   12298:	00040513          	mv	a0,s0
   1229c:	e9cfe0ef          	jal	ra,10938 <pad>
   122a0:	00048313          	mv	t1,s1
   122a4:	01b4d463          	bge	s1,s11,122ac <printf_core+0x830>
   122a8:	000d8313          	mv	t1,s11
   122ac:	00030c9b          	sext.w	s9,t1
   122b0:	00813783          	ld	a5,8(sp)
   122b4:	8597d0e3          	bge	a5,s9,11af4 <printf_core+0x78>
   122b8:	6ac000ef          	jal	ra,12964 <__errno_location>
   122bc:	04b00793          	li	a5,75
   122c0:	a91ff06f          	j	11d50 <printf_core+0x2d4>
   122c4:	00ac8cb3          	add	s9,s9,a0
   122c8:	f65ff06f          	j	1222c <printf_core+0x7b0>
   122cc:	000ba583          	lw	a1,0(s7)
   122d0:	fa058ae3          	beqz	a1,12284 <printf_core+0x808>
   122d4:	03810513          	addi	a0,sp,56
   122d8:	081000ef          	jal	ra,12b58 <wctomb>
   122dc:	00aa8ab3          	add	s5,s5,a0
   122e0:	004b8b93          	addi	s7,s7,4
   122e4:	00050613          	mv	a2,a0
   122e8:	f95ceee3          	bltu	s9,s5,12284 <printf_core+0x808>
   122ec:	03810593          	addi	a1,sp,56
   122f0:	00040513          	mv	a0,s0
   122f4:	e20fe0ef          	jal	ra,10914 <out>
   122f8:	f89ff06f          	j	12280 <printf_core+0x804>
   122fc:	000f0463          	beqz	t5,12304 <printf_core+0x888>
   12300:	fa0acce3          	bltz	s5,122b8 <printf_core+0x83c>
   12304:	05013583          	ld	a1,80(sp)
   12308:	05813603          	ld	a2,88(sp)
   1230c:	000d0793          	mv	a5,s10
   12310:	000a8713          	mv	a4,s5
   12314:	00048693          	mv	a3,s1
   12318:	00040513          	mv	a0,s0
   1231c:	ec4fe0ef          	jal	ra,109e0 <fmt_fp>
   12320:	00050c93          	mv	s9,a0
   12324:	f80556e3          	bgez	a0,122b0 <printf_core+0x834>
   12328:	f91ff06f          	j	122b8 <printf_core+0x83c>
   1232c:	00098e93          	mv	t4,s3
   12330:	09f10613          	addi	a2,sp,159
   12334:	c9dff06f          	j	11fd0 <printf_core+0x554>
   12338:	00060b93          	mv	s7,a2
   1233c:	c95ff06f          	j	11fd0 <printf_core+0x554>
   12340:	ae0418e3          	bnez	s0,11e30 <printf_core+0x3b4>
   12344:	ae0c04e3          	beqz	s8,11e2c <printf_core+0x3b0>
   12348:	00100413          	li	s0,1
   1234c:	00a00493          	li	s1,10
   12350:	00241793          	slli	a5,s0,0x2
   12354:	00fb07b3          	add	a5,s6,a5
   12358:	0007a583          	lw	a1,0(a5)
   1235c:	02059263          	bnez	a1,12380 <printf_core+0x904>
   12360:	00a00713          	li	a4,10
   12364:	00241793          	slli	a5,s0,0x2
   12368:	00fb07b3          	add	a5,s6,a5
   1236c:	0007a783          	lw	a5,0(a5)
   12370:	9c079ce3          	bnez	a5,11d48 <printf_core+0x2cc>
   12374:	00140413          	addi	s0,s0,1
   12378:	fee416e3          	bne	s0,a4,12364 <printf_core+0x8e8>
   1237c:	0200006f          	j	1239c <printf_core+0x920>
   12380:	00013783          	ld	a5,0(sp)
   12384:	00441513          	slli	a0,s0,0x4
   12388:	000a0613          	mv	a2,s4
   1238c:	00a78533          	add	a0,a5,a0
   12390:	00140413          	addi	s0,s0,1
   12394:	be8fe0ef          	jal	ra,1077c <pop_arg>
   12398:	fa941ce3          	bne	s0,s1,12350 <printf_core+0x8d4>
   1239c:	00100913          	li	s2,1
   123a0:	a91ff06f          	j	11e30 <printf_core+0x3b4>

00000000000123a4 <vfprintf>:
   123a4:	ea010113          	addi	sp,sp,-352
   123a8:	14813823          	sd	s0,336(sp)
   123ac:	00c13023          	sd	a2,0(sp)
   123b0:	00050413          	mv	s0,a0
   123b4:	00810713          	addi	a4,sp,8
   123b8:	08010693          	addi	a3,sp,128
   123bc:	00010613          	mv	a2,sp
   123c0:	00000513          	li	a0,0
   123c4:	14913423          	sd	s1,328(sp)
   123c8:	13513423          	sd	s5,296(sp)
   123cc:	14113c23          	sd	ra,344(sp)
   123d0:	15213023          	sd	s2,320(sp)
   123d4:	13313c23          	sd	s3,312(sp)
   123d8:	13413823          	sd	s4,304(sp)
   123dc:	00058a93          	mv	s5,a1
   123e0:	00013423          	sd	zero,8(sp)
   123e4:	00013823          	sd	zero,16(sp)
   123e8:	00013c23          	sd	zero,24(sp)
   123ec:	02013023          	sd	zero,32(sp)
   123f0:	02013423          	sd	zero,40(sp)
   123f4:	e88ff0ef          	jal	ra,11a7c <printf_core>
   123f8:	fff00493          	li	s1,-1
   123fc:	0e054463          	bltz	a0,124e4 <vfprintf+0x140>
   12400:	08c42783          	lw	a5,140(s0)
   12404:	00000993          	li	s3,0
   12408:	0007879b          	sext.w	a5,a5
   1240c:	0007c863          	bltz	a5,1241c <vfprintf+0x78>
   12410:	00040513          	mv	a0,s0
   12414:	1c9000ef          	jal	ra,12ddc <__lockfile>
   12418:	00050993          	mv	s3,a0
   1241c:	00042783          	lw	a5,0(s0)
   12420:	08842703          	lw	a4,136(s0)
   12424:	0207fa13          	andi	s4,a5,32
   12428:	00e04663          	bgtz	a4,12434 <vfprintf+0x90>
   1242c:	fdf7f793          	andi	a5,a5,-33
   12430:	00f42023          	sw	a5,0(s0)
   12434:	06043783          	ld	a5,96(s0)
   12438:	0c079a63          	bnez	a5,1250c <vfprintf+0x168>
   1243c:	03010793          	addi	a5,sp,48
   12440:	05843903          	ld	s2,88(s0)
   12444:	04f43c23          	sd	a5,88(s0)
   12448:	05000793          	li	a5,80
   1244c:	06f43023          	sd	a5,96(s0)
   12450:	02043023          	sd	zero,32(s0)
   12454:	02043c23          	sd	zero,56(s0)
   12458:	02043423          	sd	zero,40(s0)
   1245c:	00040513          	mv	a0,s0
   12460:	355000ef          	jal	ra,12fb4 <__towrite>
   12464:	fff00493          	li	s1,-1
   12468:	02051063          	bnez	a0,12488 <vfprintf+0xe4>
   1246c:	00810713          	addi	a4,sp,8
   12470:	08010693          	addi	a3,sp,128
   12474:	00010613          	mv	a2,sp
   12478:	000a8593          	mv	a1,s5
   1247c:	00040513          	mv	a0,s0
   12480:	dfcff0ef          	jal	ra,11a7c <printf_core>
   12484:	00050493          	mv	s1,a0
   12488:	02090c63          	beqz	s2,124c0 <vfprintf+0x11c>
   1248c:	04843783          	ld	a5,72(s0)
   12490:	00000613          	li	a2,0
   12494:	00000593          	li	a1,0
   12498:	00040513          	mv	a0,s0
   1249c:	000780e7          	jalr	a5
   124a0:	02843783          	ld	a5,40(s0)
   124a4:	00079463          	bnez	a5,124ac <vfprintf+0x108>
   124a8:	fff00493          	li	s1,-1
   124ac:	05243c23          	sd	s2,88(s0)
   124b0:	06043023          	sd	zero,96(s0)
   124b4:	02043023          	sd	zero,32(s0)
   124b8:	02043c23          	sd	zero,56(s0)
   124bc:	02043423          	sd	zero,40(s0)
   124c0:	00042783          	lw	a5,0(s0)
   124c4:	0207f713          	andi	a4,a5,32
   124c8:	00070463          	beqz	a4,124d0 <vfprintf+0x12c>
   124cc:	fff00493          	li	s1,-1
   124d0:	00fa6a33          	or	s4,s4,a5
   124d4:	01442023          	sw	s4,0(s0)
   124d8:	00098663          	beqz	s3,124e4 <vfprintf+0x140>
   124dc:	00040513          	mv	a0,s0
   124e0:	1d1000ef          	jal	ra,12eb0 <__unlockfile>
   124e4:	15813083          	ld	ra,344(sp)
   124e8:	15013403          	ld	s0,336(sp)
   124ec:	14013903          	ld	s2,320(sp)
   124f0:	13813983          	ld	s3,312(sp)
   124f4:	13013a03          	ld	s4,304(sp)
   124f8:	12813a83          	ld	s5,296(sp)
   124fc:	00048513          	mv	a0,s1
   12500:	14813483          	ld	s1,328(sp)
   12504:	16010113          	addi	sp,sp,352
   12508:	00008067          	ret
   1250c:	02043903          	ld	s2,32(s0)
   12510:	f40906e3          	beqz	s2,1245c <vfprintf+0xb8>
   12514:	00000913          	li	s2,0
   12518:	f55ff06f          	j	1246c <vfprintf+0xc8>

000000000001251c <memset>:
   1251c:	0c060e63          	beqz	a2,125f8 <memset+0xdc>
   12520:	0ff5f793          	andi	a5,a1,255
   12524:	00f50023          	sb	a5,0(a0)
   12528:	00c50733          	add	a4,a0,a2
   1252c:	fef70fa3          	sb	a5,-1(a4) # 1fff <exit-0xe121>
   12530:	00200693          	li	a3,2
   12534:	0cc6f263          	bgeu	a3,a2,125f8 <memset+0xdc>
   12538:	00f500a3          	sb	a5,1(a0)
   1253c:	00f50123          	sb	a5,2(a0)
   12540:	fef70f23          	sb	a5,-2(a4)
   12544:	fef70ea3          	sb	a5,-3(a4)
   12548:	00600693          	li	a3,6
   1254c:	0ac6f663          	bgeu	a3,a2,125f8 <memset+0xdc>
   12550:	00f501a3          	sb	a5,3(a0)
   12554:	fef70e23          	sb	a5,-4(a4)
   12558:	00800693          	li	a3,8
   1255c:	08c6fe63          	bgeu	a3,a2,125f8 <memset+0xdc>
   12560:	40a00733          	neg	a4,a0
   12564:	00377713          	andi	a4,a4,3
   12568:	00e507b3          	add	a5,a0,a4
   1256c:	40e60633          	sub	a2,a2,a4
   12570:	01010737          	lui	a4,0x1010
   12574:	1017071b          	addiw	a4,a4,257
   12578:	0ff5f593          	andi	a1,a1,255
   1257c:	02e585bb          	mulw	a1,a1,a4
   12580:	ffc67613          	andi	a2,a2,-4
   12584:	00c78733          	add	a4,a5,a2
   12588:	00b7a023          	sw	a1,0(a5)
   1258c:	feb72e23          	sw	a1,-4(a4) # 100fffc <__BSS_END__+0xff77f4>
   12590:	06c6f463          	bgeu	a3,a2,125f8 <memset+0xdc>
   12594:	00b7a223          	sw	a1,4(a5)
   12598:	00b7a423          	sw	a1,8(a5)
   1259c:	feb72a23          	sw	a1,-12(a4)
   125a0:	feb72c23          	sw	a1,-8(a4)
   125a4:	01800693          	li	a3,24
   125a8:	04c6f863          	bgeu	a3,a2,125f8 <memset+0xdc>
   125ac:	00b7a623          	sw	a1,12(a5)
   125b0:	00b7a823          	sw	a1,16(a5)
   125b4:	00b7aa23          	sw	a1,20(a5)
   125b8:	00b7ac23          	sw	a1,24(a5)
   125bc:	feb72223          	sw	a1,-28(a4)
   125c0:	feb72423          	sw	a1,-24(a4)
   125c4:	feb72623          	sw	a1,-20(a4)
   125c8:	feb72823          	sw	a1,-16(a4)
   125cc:	02059693          	slli	a3,a1,0x20
   125d0:	0047f713          	andi	a4,a5,4
   125d4:	01870713          	addi	a4,a4,24
   125d8:	0206d693          	srli	a3,a3,0x20
   125dc:	02059593          	slli	a1,a1,0x20
   125e0:	00e78733          	add	a4,a5,a4
   125e4:	00c78633          	add	a2,a5,a2
   125e8:	00d5e5b3          	or	a1,a1,a3
   125ec:	01f00793          	li	a5,31
   125f0:	40e606b3          	sub	a3,a2,a4
   125f4:	00d7e463          	bltu	a5,a3,125fc <memset+0xe0>
   125f8:	00008067          	ret
   125fc:	00b73023          	sd	a1,0(a4)
   12600:	00b73423          	sd	a1,8(a4)
   12604:	00b73823          	sd	a1,16(a4)
   12608:	00b73c23          	sd	a1,24(a4)
   1260c:	02070713          	addi	a4,a4,32
   12610:	fe1ff06f          	j	125f0 <memset+0xd4>

0000000000012614 <strnlen>:
   12614:	fe010113          	addi	sp,sp,-32
   12618:	00813823          	sd	s0,16(sp)
   1261c:	00058613          	mv	a2,a1
   12620:	00058413          	mv	s0,a1
   12624:	00000593          	li	a1,0
   12628:	00913423          	sd	s1,8(sp)
   1262c:	00113c23          	sd	ra,24(sp)
   12630:	00050493          	mv	s1,a0
   12634:	38d000ef          	jal	ra,131c0 <memchr>
   12638:	00050463          	beqz	a0,12640 <strnlen+0x2c>
   1263c:	40950433          	sub	s0,a0,s1
   12640:	01813083          	ld	ra,24(sp)
   12644:	00040513          	mv	a0,s0
   12648:	01013403          	ld	s0,16(sp)
   1264c:	00813483          	ld	s1,8(sp)
   12650:	02010113          	addi	sp,sp,32
   12654:	00008067          	ret

0000000000012658 <getpid>:
   12658:	0ac00893          	li	a7,172
   1265c:	00000073          	ecall
   12660:	0005051b          	sext.w	a0,a0
   12664:	00008067          	ret

0000000000012668 <__init_tp>:
   12668:	ff010113          	addi	sp,sp,-16
   1266c:	00813023          	sd	s0,0(sp)
   12670:	00113423          	sd	ra,8(sp)
   12674:	00050413          	mv	s0,a0
   12678:	00a53023          	sd	a0,0(a0)
   1267c:	0e050513          	addi	a0,a0,224
   12680:	184010ef          	jal	ra,13804 <__set_thread_area>
   12684:	04054e63          	bltz	a0,126e0 <__init_tp+0x78>
   12688:	00051663          	bnez	a0,12694 <__init_tp+0x2c>
   1268c:	00100713          	li	a4,1
   12690:	9ee1ac23          	sw	a4,-1544(gp) # 181f8 <__libc>
   12694:	00100793          	li	a5,1
   12698:	04f42023          	sw	a5,64(s0)
   1269c:	06000893          	li	a7,96
   126a0:	98818513          	addi	a0,gp,-1656 # 18188 <__thread_list_lock>
   126a4:	00000073          	ecall
   126a8:	a3818793          	addi	a5,gp,-1480 # 18238 <__libc+0x40>
   126ac:	0af43823          	sd	a5,176(s0)
   126b0:	09040793          	addi	a5,s0,144
   126b4:	08f43823          	sd	a5,144(s0)
   126b8:	9601b783          	ld	a5,-1696(gp) # 18160 <__sysinfo>
   126bc:	02a42c23          	sw	a0,56(s0)
   126c0:	00843823          	sd	s0,16(s0)
   126c4:	02f43023          	sd	a5,32(s0)
   126c8:	00843c23          	sd	s0,24(s0)
   126cc:	00000513          	li	a0,0
   126d0:	00813083          	ld	ra,8(sp)
   126d4:	00013403          	ld	s0,0(sp)
   126d8:	01010113          	addi	sp,sp,16
   126dc:	00008067          	ret
   126e0:	fff00513          	li	a0,-1
   126e4:	fedff06f          	j	126d0 <__init_tp+0x68>

00000000000126e8 <__copy_tls>:
   126e8:	fc010113          	addi	sp,sp,-64
   126ec:	9f818713          	addi	a4,gp,-1544 # 181f8 <__libc>
   126f0:	03073683          	ld	a3,48(a4)
   126f4:	02913423          	sd	s1,40(sp)
   126f8:	02073483          	ld	s1,32(a4)
   126fc:	02813823          	sd	s0,48(sp)
   12700:	02873403          	ld	s0,40(a4)
   12704:	00369693          	slli	a3,a3,0x3
   12708:	ff848493          	addi	s1,s1,-8
   1270c:	40d484b3          	sub	s1,s1,a3
   12710:	f2000693          	li	a3,-224
   12714:	fff40413          	addi	s0,s0,-1
   12718:	40a686b3          	sub	a3,a3,a0
   1271c:	03213023          	sd	s2,32(sp)
   12720:	00d47433          	and	s0,s0,a3
   12724:	01873903          	ld	s2,24(a4)
   12728:	01513423          	sd	s5,8(sp)
   1272c:	009504b3          	add	s1,a0,s1
   12730:	00850433          	add	s0,a0,s0
   12734:	00001ab7          	lui	s5,0x1
   12738:	01313c23          	sd	s3,24(sp)
   1273c:	01413823          	sd	s4,16(sp)
   12740:	01613023          	sd	s6,0(sp)
   12744:	02113c23          	sd	ra,56(sp)
   12748:	0e040b13          	addi	s6,s0,224
   1274c:	00848a13          	addi	s4,s1,8
   12750:	9f818993          	addi	s3,gp,-1544 # 181f8 <__libc>
   12754:	800a8a93          	addi	s5,s5,-2048 # 800 <exit-0xf920>
   12758:	04091063          	bnez	s2,12798 <__copy_tls+0xb0>
   1275c:	0309b783          	ld	a5,48(s3)
   12760:	03813083          	ld	ra,56(sp)
   12764:	00040513          	mv	a0,s0
   12768:	00f4b023          	sd	a5,0(s1)
   1276c:	0c943c23          	sd	s1,216(s0)
   12770:	00943423          	sd	s1,8(s0)
   12774:	03013403          	ld	s0,48(sp)
   12778:	02813483          	ld	s1,40(sp)
   1277c:	02013903          	ld	s2,32(sp)
   12780:	01813983          	ld	s3,24(sp)
   12784:	01013a03          	ld	s4,16(sp)
   12788:	00813a83          	ld	s5,8(sp)
   1278c:	00013b03          	ld	s6,0(sp)
   12790:	04010113          	addi	sp,sp,64
   12794:	00008067          	ret
   12798:	02893783          	ld	a5,40(s2)
   1279c:	00893583          	ld	a1,8(s2)
   127a0:	008a0a13          	addi	s4,s4,8
   127a4:	00fb07b3          	add	a5,s6,a5
   127a8:	015787b3          	add	a5,a5,s5
   127ac:	fefa3c23          	sd	a5,-8(s4)
   127b0:	02893503          	ld	a0,40(s2)
   127b4:	01093603          	ld	a2,16(s2)
   127b8:	00ab0533          	add	a0,s6,a0
   127bc:	299000ef          	jal	ra,13254 <memcpy>
   127c0:	00093903          	ld	s2,0(s2)
   127c4:	f95ff06f          	j	12758 <__copy_tls+0x70>

00000000000127c8 <__init_tls>:
   127c8:	01853f83          	ld	t6,24(a0)
   127cc:	02853e03          	ld	t3,40(a0)
   127d0:	94c1a303          	lw	t1,-1716(gp) # 1814c <__default_stacksize>
   127d4:	ff010113          	addi	sp,sp,-16
   127d8:	6474eeb7          	lui	t4,0x6474e
   127dc:	00113423          	sd	ra,8(sp)
   127e0:	00813023          	sd	s0,0(sp)
   127e4:	000f8793          	mv	a5,t6
   127e8:	00000f13          	li	t5,0
   127ec:	00000713          	li	a4,0
   127f0:	00000613          	li	a2,0
   127f4:	00600293          	li	t0,6
   127f8:	00200393          	li	t2,2
   127fc:	00000813          	li	a6,0
   12800:	00700093          	li	ra,7
   12804:	551e8e93          	addi	t4,t4,1361 # 6474e551 <__BSS_END__+0x64735d49>
   12808:	00800437          	lui	s0,0x800
   1280c:	0c0e1c63          	bnez	t3,128e4 <__init_tls+0x11c>
   12810:	000f0463          	beqz	t5,12818 <__init_tls+0x50>
   12814:	9461a623          	sw	t1,-1716(gp) # 1814c <__default_stacksize>
   12818:	fd818793          	addi	a5,gp,-40 # 187d8 <main_tls>
   1281c:	9f818693          	addi	a3,gp,-1544 # 181f8 <__libc>
   12820:	02060a63          	beqz	a2,12854 <__init_tls+0x8c>
   12824:	01063583          	ld	a1,16(a2)
   12828:	00f6bc23          	sd	a5,24(a3)
   1282c:	00e58733          	add	a4,a1,a4
   12830:	00e7b423          	sd	a4,8(a5)
   12834:	02063703          	ld	a4,32(a2)
   12838:	00e7b823          	sd	a4,16(a5)
   1283c:	02863703          	ld	a4,40(a2)
   12840:	00e7bc23          	sd	a4,24(a5)
   12844:	03063703          	ld	a4,48(a2)
   12848:	02e7b023          	sd	a4,32(a5)
   1284c:	00100713          	li	a4,1
   12850:	02e6b823          	sd	a4,48(a3)
   12854:	0187b803          	ld	a6,24(a5)
   12858:	0087b583          	ld	a1,8(a5)
   1285c:	0207b503          	ld	a0,32(a5)
   12860:	00b80733          	add	a4,a6,a1
   12864:	fff50613          	addi	a2,a0,-1
   12868:	40e00733          	neg	a4,a4
   1286c:	00c77733          	and	a4,a4,a2
   12870:	01070733          	add	a4,a4,a6
   12874:	00c5f633          	and	a2,a1,a2
   12878:	00e7bc23          	sd	a4,24(a5)
   1287c:	02c7b423          	sd	a2,40(a5)
   12880:	00700593          	li	a1,7
   12884:	00a5e663          	bltu	a1,a0,12890 <__init_tls+0xc8>
   12888:	00800593          	li	a1,8
   1288c:	02b7b023          	sd	a1,32(a5)
   12890:	0207b783          	ld	a5,32(a5)
   12894:	0f778593          	addi	a1,a5,247
   12898:	00c585b3          	add	a1,a1,a2
   1289c:	00e585b3          	add	a1,a1,a4
   128a0:	ff85f593          	andi	a1,a1,-8
   128a4:	02f6b423          	sd	a5,40(a3)
   128a8:	02b6b023          	sd	a1,32(a3)
   128ac:	16800793          	li	a5,360
   128b0:	08b7fe63          	bgeu	a5,a1,1294c <__init_tls+0x184>
   128b4:	0de00893          	li	a7,222
   128b8:	00000513          	li	a0,0
   128bc:	00300613          	li	a2,3
   128c0:	02200693          	li	a3,34
   128c4:	fff00713          	li	a4,-1
   128c8:	00000793          	li	a5,0
   128cc:	00000073          	ecall
   128d0:	e19ff0ef          	jal	ra,126e8 <__copy_tls>
   128d4:	d95ff0ef          	jal	ra,12668 <__init_tp>
   128d8:	06055e63          	bgez	a0,12954 <__init_tls+0x18c>
   128dc:	00000023          	sb	zero,0(zero) # 0 <exit-0x10120>
   128e0:	00100073          	ebreak
   128e4:	0007a883          	lw	a7,0(a5)
   128e8:	00589e63          	bne	a7,t0,12904 <__init_tls+0x13c>
   128ec:	0107b703          	ld	a4,16(a5)
   128f0:	40ef8733          	sub	a4,t6,a4
   128f4:	02053683          	ld	a3,32(a0)
   128f8:	fffe0e13          	addi	t3,t3,-1
   128fc:	00d787b3          	add	a5,a5,a3
   12900:	f0dff06f          	j	1280c <__init_tls+0x44>
   12904:	02788863          	beq	a7,t2,12934 <__init_tls+0x16c>
   12908:	02188e63          	beq	a7,ra,12944 <__init_tls+0x17c>
   1290c:	ffd894e3          	bne	a7,t4,128f4 <__init_tls+0x12c>
   12910:	0287b683          	ld	a3,40(a5)
   12914:	02031893          	slli	a7,t1,0x20
   12918:	0208d893          	srli	a7,a7,0x20
   1291c:	fcd8fce3          	bgeu	a7,a3,128f4 <__init_tls+0x12c>
   12920:	00d47463          	bgeu	s0,a3,12928 <__init_tls+0x160>
   12924:	008006b7          	lui	a3,0x800
   12928:	0006831b          	sext.w	t1,a3
   1292c:	00100f13          	li	t5,1
   12930:	fc5ff06f          	j	128f4 <__init_tls+0x12c>
   12934:	fc0800e3          	beqz	a6,128f4 <__init_tls+0x12c>
   12938:	0107b703          	ld	a4,16(a5)
   1293c:	40e80733          	sub	a4,a6,a4
   12940:	fb5ff06f          	j	128f4 <__init_tls+0x12c>
   12944:	00078613          	mv	a2,a5
   12948:	fadff06f          	j	128f4 <__init_tls+0x12c>
   1294c:	e7018513          	addi	a0,gp,-400 # 18670 <builtin_tls>
   12950:	f81ff06f          	j	128d0 <__init_tls+0x108>
   12954:	00813083          	ld	ra,8(sp)
   12958:	00013403          	ld	s0,0(sp)
   1295c:	01010113          	addi	sp,sp,16
   12960:	00008067          	ret

0000000000012964 <__errno_location>:
   12964:	00020513          	mv	a0,tp
   12968:	f5c50513          	addi	a0,a0,-164
   1296c:	00008067          	ret

0000000000012970 <__strerror_l>:
   12970:	000166b7          	lui	a3,0x16
   12974:	00000793          	li	a5,0
   12978:	30068693          	addi	a3,a3,768 # 16300 <errid>
   1297c:	00f68633          	add	a2,a3,a5
   12980:	00064603          	lbu	a2,0(a2)
   12984:	0007871b          	sext.w	a4,a5
   12988:	00060663          	beqz	a2,12994 <__strerror_l+0x24>
   1298c:	00178793          	addi	a5,a5,1
   12990:	fea616e3          	bne	a2,a0,1297c <__strerror_l+0xc>
   12994:	00016537          	lui	a0,0x16
   12998:	36050513          	addi	a0,a0,864 # 16360 <errmsg>
   1299c:	0140006f          	j	129b0 <__strerror_l+0x40>
   129a0:	00054783          	lbu	a5,0(a0)
   129a4:	00150513          	addi	a0,a0,1
   129a8:	fe079ce3          	bnez	a5,129a0 <__strerror_l+0x30>
   129ac:	fff7071b          	addiw	a4,a4,-1
   129b0:	fe0718e3          	bnez	a4,129a0 <__strerror_l+0x30>
   129b4:	0285b583          	ld	a1,40(a1) # ffffffff80000028 <__BSS_END__+0xffffffff7ffe7820>
   129b8:	0640006f          	j	12a1c <__lctrans>

00000000000129bc <strerror>:
   129bc:	00020793          	mv	a5,tp
   129c0:	fd07b583          	ld	a1,-48(a5)
   129c4:	fadff06f          	j	12970 <__strerror_l>

00000000000129c8 <_Exit>:
   129c8:	00050793          	mv	a5,a0
   129cc:	05e00893          	li	a7,94
   129d0:	00000073          	ecall
   129d4:	05d00893          	li	a7,93
   129d8:	00078513          	mv	a0,a5
   129dc:	ff5ff06f          	j	129d0 <_Exit+0x8>

00000000000129e0 <__syscall_ret>:
   129e0:	ff010113          	addi	sp,sp,-16
   129e4:	00813023          	sd	s0,0(sp)
   129e8:	00113423          	sd	ra,8(sp)
   129ec:	fffff7b7          	lui	a5,0xfffff
   129f0:	00050413          	mv	s0,a0
   129f4:	00a7fa63          	bgeu	a5,a0,12a08 <__syscall_ret+0x28>
   129f8:	f6dff0ef          	jal	ra,12964 <__errno_location>
   129fc:	4080043b          	negw	s0,s0
   12a00:	00852023          	sw	s0,0(a0)
   12a04:	fff00513          	li	a0,-1
   12a08:	00813083          	ld	ra,8(sp)
   12a0c:	00013403          	ld	s0,0(sp)
   12a10:	01010113          	addi	sp,sp,16
   12a14:	00008067          	ret

0000000000012a18 <__lctrans_impl>:
   12a18:	00008067          	ret

0000000000012a1c <__lctrans>:
   12a1c:	ffdff06f          	j	12a18 <__lctrans_impl>

0000000000012a20 <__lctrans_cur>:
   12a20:	00020793          	mv	a5,tp
   12a24:	fd07b783          	ld	a5,-48(a5) # ffffffffffffefd0 <__BSS_END__+0xfffffffffffe67c8>
   12a28:	0287b583          	ld	a1,40(a5)
   12a2c:	fedff06f          	j	12a18 <__lctrans_impl>

0000000000012a30 <__fpclassifyl>:
   12a30:	00008737          	lui	a4,0x8
   12a34:	0305d693          	srli	a3,a1,0x30
   12a38:	fff70713          	addi	a4,a4,-1 # 7fff <exit-0x8121>
   12a3c:	01059593          	slli	a1,a1,0x10
   12a40:	00e6f6b3          	and	a3,a3,a4
   12a44:	00050793          	mv	a5,a0
   12a48:	0105d593          	srli	a1,a1,0x10
   12a4c:	00069a63          	bnez	a3,12a60 <__fpclassifyl+0x30>
   12a50:	00a5e5b3          	or	a1,a1,a0
   12a54:	00b03533          	snez	a0,a1
   12a58:	00250513          	addi	a0,a0,2
   12a5c:	00008067          	ret
   12a60:	00400513          	li	a0,4
   12a64:	00e69663          	bne	a3,a4,12a70 <__fpclassifyl+0x40>
   12a68:	00f5e5b3          	or	a1,a1,a5
   12a6c:	0015b513          	seqz	a0,a1
   12a70:	00008067          	ret

0000000000012a74 <__signbitl>:
   12a74:	03f5d513          	srli	a0,a1,0x3f
   12a78:	00008067          	ret

0000000000012a7c <frexpl>:
   12a7c:	00008737          	lui	a4,0x8
   12a80:	fe010113          	addi	sp,sp,-32
   12a84:	0305d793          	srli	a5,a1,0x30
   12a88:	fff70713          	addi	a4,a4,-1 # 7fff <exit-0x8121>
   12a8c:	00813823          	sd	s0,16(sp)
   12a90:	00913423          	sd	s1,8(sp)
   12a94:	01213023          	sd	s2,0(sp)
   12a98:	00113c23          	sd	ra,24(sp)
   12a9c:	00060913          	mv	s2,a2
   12aa0:	00e7f633          	and	a2,a5,a4
   12aa4:	00050493          	mv	s1,a0
   12aa8:	00058413          	mv	s0,a1
   12aac:	06061663          	bnez	a2,12b18 <frexpl+0x9c>
   12ab0:	00000693          	li	a3,0
   12ab4:	3f5010ef          	jal	ra,146a8 <__eqtf2>
   12ab8:	04050c63          	beqz	a0,12b10 <frexpl+0x94>
   12abc:	000177b7          	lui	a5,0x17
   12ac0:	a887b683          	ld	a3,-1400(a5) # 16a88 <errmsg+0x728>
   12ac4:	00000613          	li	a2,0
   12ac8:	00048513          	mv	a0,s1
   12acc:	00040593          	mv	a1,s0
   12ad0:	4a9010ef          	jal	ra,14778 <__multf3>
   12ad4:	00090613          	mv	a2,s2
   12ad8:	fa5ff0ef          	jal	ra,12a7c <frexpl>
   12adc:	00092783          	lw	a5,0(s2)
   12ae0:	00050493          	mv	s1,a0
   12ae4:	00058413          	mv	s0,a1
   12ae8:	f887879b          	addiw	a5,a5,-120
   12aec:	00f92023          	sw	a5,0(s2)
   12af0:	01813083          	ld	ra,24(sp)
   12af4:	00040593          	mv	a1,s0
   12af8:	01013403          	ld	s0,16(sp)
   12afc:	00013903          	ld	s2,0(sp)
   12b00:	00048513          	mv	a0,s1
   12b04:	00813483          	ld	s1,8(sp)
   12b08:	02010113          	addi	sp,sp,32
   12b0c:	00008067          	ret
   12b10:	00092023          	sw	zero,0(s2)
   12b14:	fddff06f          	j	12af0 <frexpl+0x74>
   12b18:	0006069b          	sext.w	a3,a2
   12b1c:	fce60ae3          	beq	a2,a4,12af0 <frexpl+0x74>
   12b20:	ffffc737          	lui	a4,0xffffc
   12b24:	0027071b          	addiw	a4,a4,2
   12b28:	00e6873b          	addw	a4,a3,a4
   12b2c:	00e92023          	sw	a4,0(s2)
   12b30:	ffff8737          	lui	a4,0xffff8
   12b34:	00e7f7b3          	and	a5,a5,a4
   12b38:	00004737          	lui	a4,0x4
   12b3c:	ffe70713          	addi	a4,a4,-2 # 3ffe <exit-0xc122>
   12b40:	00e7e7b3          	or	a5,a5,a4
   12b44:	01059413          	slli	s0,a1,0x10
   12b48:	03079793          	slli	a5,a5,0x30
   12b4c:	01045413          	srli	s0,s0,0x10
   12b50:	00f46433          	or	s0,s0,a5
   12b54:	f9dff06f          	j	12af0 <frexpl+0x74>

0000000000012b58 <wctomb>:
   12b58:	02050263          	beqz	a0,12b7c <wctomb+0x24>
   12b5c:	ff010113          	addi	sp,sp,-16
   12b60:	00000613          	li	a2,0
   12b64:	00113423          	sd	ra,8(sp)
   12b68:	4b5000ef          	jal	ra,1381c <wcrtomb>
   12b6c:	00813083          	ld	ra,8(sp)
   12b70:	0005051b          	sext.w	a0,a0
   12b74:	01010113          	addi	sp,sp,16
   12b78:	00008067          	ret
   12b7c:	00000513          	li	a0,0
   12b80:	00008067          	ret

0000000000012b84 <a_fetch_or.isra.0>:
   12b84:	00052783          	lw	a5,0(a0)
   12b88:	0007879b          	sext.w	a5,a5
   12b8c:	00b7e6b3          	or	a3,a5,a1
   12b90:	1605272f          	lr.w.aqrl	a4,(a0)
   12b94:	00f71663          	bne	a4,a5,12ba0 <a_fetch_or.isra.0+0x1c>
   12b98:	1ed5262f          	sc.w.aqrl	a2,a3,(a0)
   12b9c:	fe061ae3          	bnez	a2,12b90 <a_fetch_or.isra.0+0xc>
   12ba0:	0007071b          	sext.w	a4,a4
   12ba4:	fee790e3          	bne	a5,a4,12b84 <a_fetch_or.isra.0>
   12ba8:	00008067          	ret

0000000000012bac <__get_handler_set>:
   12bac:	00800613          	li	a2,8
   12bb0:	99818593          	addi	a1,gp,-1640 # 18198 <handler_set>
   12bb4:	6a00006f          	j	13254 <memcpy>

0000000000012bb8 <__libc_sigaction>:
   12bb8:	f8010113          	addi	sp,sp,-128
   12bbc:	06813823          	sd	s0,112(sp)
   12bc0:	07213023          	sd	s2,96(sp)
   12bc4:	05313c23          	sd	s3,88(sp)
   12bc8:	06113c23          	sd	ra,120(sp)
   12bcc:	06913423          	sd	s1,104(sp)
   12bd0:	05413823          	sd	s4,80(sp)
   12bd4:	00050993          	mv	s3,a0
   12bd8:	00058413          	mv	s0,a1
   12bdc:	00060913          	mv	s2,a2
   12be0:	10058a63          	beqz	a1,12cf4 <__libc_sigaction+0x13c>
   12be4:	0005b783          	ld	a5,0(a1)
   12be8:	00100493          	li	s1,1
   12bec:	0af4f263          	bgeu	s1,a5,12c90 <__libc_sigaction+0xd8>
   12bf0:	fff5051b          	addiw	a0,a0,-1
   12bf4:	00050593          	mv	a1,a0
   12bf8:	00655513          	srli	a0,a0,0x6
   12bfc:	00b494b3          	sll	s1,s1,a1
   12c00:	00351513          	slli	a0,a0,0x3
   12c04:	99818a13          	addi	s4,gp,-1640 # 18198 <handler_set>
   12c08:	0004859b          	sext.w	a1,s1
   12c0c:	00aa0a33          	add	s4,s4,a0
   12c10:	00058663          	beqz	a1,12c1c <__libc_sigaction+0x64>
   12c14:	000a0513          	mv	a0,s4
   12c18:	f6dff0ef          	jal	ra,12b84 <a_fetch_or.isra.0>
   12c1c:	4204d593          	srai	a1,s1,0x20
   12c20:	00058663          	beqz	a1,12c2c <__libc_sigaction+0x74>
   12c24:	004a0513          	addi	a0,s4,4
   12c28:	f5dff0ef          	jal	ra,12b84 <a_fetch_or.isra.0>
   12c2c:	9a018713          	addi	a4,gp,-1632 # 181a0 <unmask_done>
   12c30:	9fc1a783          	lw	a5,-1540(gp) # 181fc <__libc+0x4>
   12c34:	00072683          	lw	a3,0(a4)
   12c38:	00d7e7b3          	or	a5,a5,a3
   12c3c:	0007879b          	sext.w	a5,a5
   12c40:	02079863          	bnez	a5,12c70 <__libc_sigaction+0xb8>
   12c44:	00300793          	li	a5,3
   12c48:	02079793          	slli	a5,a5,0x20
   12c4c:	02f13823          	sd	a5,48(sp)
   12c50:	08700893          	li	a7,135
   12c54:	00100513          	li	a0,1
   12c58:	03010593          	addi	a1,sp,48
   12c5c:	00000613          	li	a2,0
   12c60:	00800693          	li	a3,8
   12c64:	00000073          	ecall
   12c68:	00100793          	li	a5,1
   12c6c:	00f72023          	sw	a5,0(a4)
   12c70:	08842783          	lw	a5,136(s0) # 800088 <__BSS_END__+0x7e7880>
   12c74:	10000737          	lui	a4,0x10000
   12c78:	00e7f7b3          	and	a5,a5,a4
   12c7c:	00079a63          	bnez	a5,12c90 <__libc_sigaction+0xd8>
   12c80:	0330000f          	fence	rw,rw
   12c84:	00100713          	li	a4,1
   12c88:	98e1a623          	sw	a4,-1652(gp) # 1818c <__eintr_valid_flag>
   12c8c:	0330000f          	fence	rw,rw
   12c90:	00043783          	ld	a5,0(s0)
   12c94:	00078e63          	beqz	a5,12cb0 <__libc_sigaction+0xf8>
   12c98:	00600793          	li	a5,6
   12c9c:	00f99a63          	bne	s3,a5,12cb0 <__libc_sigaction+0xf8>
   12ca0:	00810513          	addi	a0,sp,8
   12ca4:	4a9000ef          	jal	ra,1394c <__block_all_sigs>
   12ca8:	99018513          	addi	a0,gp,-1648 # 18190 <__abort_lock>
   12cac:	1ed000ef          	jal	ra,13698 <__lock>
   12cb0:	00043783          	ld	a5,0(s0)
   12cb4:	04000737          	lui	a4,0x4000
   12cb8:	00f13823          	sd	a5,16(sp)
   12cbc:	08842783          	lw	a5,136(s0)
   12cc0:	00e7e733          	or	a4,a5,a4
   12cc4:	00e13c23          	sd	a4,24(sp)
   12cc8:	0047f793          	andi	a5,a5,4
   12ccc:	0c079263          	bnez	a5,12d90 <__libc_sigaction+0x1d8>
   12cd0:	000147b7          	lui	a5,0x14
   12cd4:	9a078793          	addi	a5,a5,-1632 # 139a0 <__restore>
   12cd8:	00840593          	addi	a1,s0,8
   12cdc:	00800613          	li	a2,8
   12ce0:	02810513          	addi	a0,sp,40
   12ce4:	02f13023          	sd	a5,32(sp)
   12ce8:	56c000ef          	jal	ra,13254 <memcpy>
   12cec:	01010593          	addi	a1,sp,16
   12cf0:	00041463          	bnez	s0,12cf8 <__libc_sigaction+0x140>
   12cf4:	00000593          	li	a1,0
   12cf8:	00000613          	li	a2,0
   12cfc:	00090463          	beqz	s2,12d04 <__libc_sigaction+0x14c>
   12d00:	03010613          	addi	a2,sp,48
   12d04:	08600893          	li	a7,134
   12d08:	00098513          	mv	a0,s3
   12d0c:	00800693          	li	a3,8
   12d10:	00000073          	ecall
   12d14:	00600793          	li	a5,6
   12d18:	0005049b          	sext.w	s1,a0
   12d1c:	02f99063          	bne	s3,a5,12d3c <__libc_sigaction+0x184>
   12d20:	00040e63          	beqz	s0,12d3c <__libc_sigaction+0x184>
   12d24:	00043783          	ld	a5,0(s0)
   12d28:	00078a63          	beqz	a5,12d3c <__libc_sigaction+0x184>
   12d2c:	99018513          	addi	a0,gp,-1648 # 18190 <__abort_lock>
   12d30:	261000ef          	jal	ra,13790 <__unlock>
   12d34:	00810513          	addi	a0,sp,8
   12d38:	44d000ef          	jal	ra,13984 <__restore_sigs>
   12d3c:	02090463          	beqz	s2,12d64 <__libc_sigaction+0x1ac>
   12d40:	02049263          	bnez	s1,12d64 <__libc_sigaction+0x1ac>
   12d44:	03013783          	ld	a5,48(sp)
   12d48:	00800613          	li	a2,8
   12d4c:	04810593          	addi	a1,sp,72
   12d50:	00f93023          	sd	a5,0(s2)
   12d54:	03813783          	ld	a5,56(sp)
   12d58:	00890513          	addi	a0,s2,8
   12d5c:	08f92423          	sw	a5,136(s2)
   12d60:	4f4000ef          	jal	ra,13254 <memcpy>
   12d64:	00048513          	mv	a0,s1
   12d68:	c79ff0ef          	jal	ra,129e0 <__syscall_ret>
   12d6c:	07813083          	ld	ra,120(sp)
   12d70:	07013403          	ld	s0,112(sp)
   12d74:	06813483          	ld	s1,104(sp)
   12d78:	06013903          	ld	s2,96(sp)
   12d7c:	05813983          	ld	s3,88(sp)
   12d80:	05013a03          	ld	s4,80(sp)
   12d84:	0005051b          	sext.w	a0,a0
   12d88:	08010113          	addi	sp,sp,128
   12d8c:	00008067          	ret
   12d90:	000147b7          	lui	a5,0x14
   12d94:	9a078793          	addi	a5,a5,-1632 # 139a0 <__restore>
   12d98:	f41ff06f          	j	12cd8 <__libc_sigaction+0x120>

0000000000012d9c <__sigaction>:
   12d9c:	fe05069b          	addiw	a3,a0,-32
   12da0:	00200713          	li	a4,2
   12da4:	00d77a63          	bgeu	a4,a3,12db8 <__sigaction+0x1c>
   12da8:	fff5079b          	addiw	a5,a0,-1
   12dac:	03f00713          	li	a4,63
   12db0:	00f76463          	bltu	a4,a5,12db8 <__sigaction+0x1c>
   12db4:	e05ff06f          	j	12bb8 <__libc_sigaction>
   12db8:	ff010113          	addi	sp,sp,-16
   12dbc:	00113423          	sd	ra,8(sp)
   12dc0:	ba5ff0ef          	jal	ra,12964 <__errno_location>
   12dc4:	00813083          	ld	ra,8(sp)
   12dc8:	01600793          	li	a5,22
   12dcc:	00f52023          	sw	a5,0(a0)
   12dd0:	fff00513          	li	a0,-1
   12dd4:	01010113          	addi	sp,sp,16
   12dd8:	00008067          	ret

0000000000012ddc <__lockfile>:
   12ddc:	08c52783          	lw	a5,140(a0)
   12de0:	0007879b          	sext.w	a5,a5
   12de4:	00020713          	mv	a4,tp
   12de8:	c00006b7          	lui	a3,0xc0000
   12dec:	f5872703          	lw	a4,-168(a4) # 3ffff58 <__BSS_END__+0x3fe7750>
   12df0:	fff68693          	addi	a3,a3,-1 # ffffffffbfffffff <__BSS_END__+0xffffffffbffe77f7>
   12df4:	00d7f7b3          	and	a5,a5,a3
   12df8:	0ae78863          	beq	a5,a4,12ea8 <__lockfile+0xcc>
   12dfc:	08c50793          	addi	a5,a0,140
   12e00:	00000613          	li	a2,0
   12e04:	1607a6af          	lr.w.aqrl	a3,(a5)
   12e08:	00c69663          	bne	a3,a2,12e14 <__lockfile+0x38>
   12e0c:	1ee7a5af          	sc.w.aqrl	a1,a4,(a5)
   12e10:	fe059ae3          	bnez	a1,12e04 <__lockfile+0x28>
   12e14:	0006869b          	sext.w	a3,a3
   12e18:	02068863          	beqz	a3,12e48 <__lockfile+0x6c>
   12e1c:	40000837          	lui	a6,0x40000
   12e20:	01076733          	or	a4,a4,a6
   12e24:	00000313          	li	t1,0
   12e28:	0007071b          	sext.w	a4,a4
   12e2c:	fda00e13          	li	t3,-38
   12e30:	1607a6af          	lr.w.aqrl	a3,(a5)
   12e34:	00669663          	bne	a3,t1,12e40 <__lockfile+0x64>
   12e38:	1ee7a62f          	sc.w.aqrl	a2,a4,(a5)
   12e3c:	fe061ae3          	bnez	a2,12e30 <__lockfile+0x54>
   12e40:	0006869b          	sext.w	a3,a3
   12e44:	00069663          	bnez	a3,12e50 <__lockfile+0x74>
   12e48:	00100513          	li	a0,1
   12e4c:	00008067          	ret
   12e50:	0106f5b3          	and	a1,a3,a6
   12e54:	0106e633          	or	a2,a3,a6
   12e58:	0005859b          	sext.w	a1,a1
   12e5c:	0006061b          	sext.w	a2,a2
   12e60:	02058663          	beqz	a1,12e8c <__lockfile+0xb0>
   12e64:	06200893          	li	a7,98
   12e68:	00078513          	mv	a0,a5
   12e6c:	08000593          	li	a1,128
   12e70:	00000693          	li	a3,0
   12e74:	00000073          	ecall
   12e78:	fbc51ce3          	bne	a0,t3,12e30 <__lockfile+0x54>
   12e7c:	00078513          	mv	a0,a5
   12e80:	00000593          	li	a1,0
   12e84:	00000073          	ecall
   12e88:	fa9ff06f          	j	12e30 <__lockfile+0x54>
   12e8c:	1607a5af          	lr.w.aqrl	a1,(a5)
   12e90:	00d59663          	bne	a1,a3,12e9c <__lockfile+0xc0>
   12e94:	1ec7a52f          	sc.w.aqrl	a0,a2,(a5)
   12e98:	fe051ae3          	bnez	a0,12e8c <__lockfile+0xb0>
   12e9c:	0005859b          	sext.w	a1,a1
   12ea0:	f8d598e3          	bne	a1,a3,12e30 <__lockfile+0x54>
   12ea4:	fc1ff06f          	j	12e64 <__lockfile+0x88>
   12ea8:	00000513          	li	a0,0
   12eac:	00008067          	ret

0000000000012eb0 <__unlockfile>:
   12eb0:	08c50713          	addi	a4,a0,140
   12eb4:	00000613          	li	a2,0
   12eb8:	08c52783          	lw	a5,140(a0)
   12ebc:	0007879b          	sext.w	a5,a5
   12ec0:	160726af          	lr.w.aqrl	a3,(a4)
   12ec4:	00f69663          	bne	a3,a5,12ed0 <__unlockfile+0x20>
   12ec8:	1ec725af          	sc.w.aqrl	a1,a2,(a4)
   12ecc:	fe059ae3          	bnez	a1,12ec0 <__unlockfile+0x10>
   12ed0:	0006869b          	sext.w	a3,a3
   12ed4:	fed792e3          	bne	a5,a3,12eb8 <__unlockfile+0x8>
   12ed8:	01e7d793          	srli	a5,a5,0x1e
   12edc:	0017f793          	andi	a5,a5,1
   12ee0:	02078663          	beqz	a5,12f0c <__unlockfile+0x5c>
   12ee4:	06200893          	li	a7,98
   12ee8:	00070513          	mv	a0,a4
   12eec:	08100593          	li	a1,129
   12ef0:	00100613          	li	a2,1
   12ef4:	00000073          	ecall
   12ef8:	fda00793          	li	a5,-38
   12efc:	00f51863          	bne	a0,a5,12f0c <__unlockfile+0x5c>
   12f00:	00070513          	mv	a0,a4
   12f04:	00100593          	li	a1,1
   12f08:	00000073          	ecall
   12f0c:	00008067          	ret

0000000000012f10 <__aio_close>:
   12f10:	00008067          	ret

0000000000012f14 <__stdio_close>:
   12f14:	07852503          	lw	a0,120(a0)
   12f18:	ff010113          	addi	sp,sp,-16
   12f1c:	00113423          	sd	ra,8(sp)
   12f20:	ff1ff0ef          	jal	ra,12f10 <__aio_close>
   12f24:	03900893          	li	a7,57
   12f28:	00000073          	ecall
   12f2c:	ab5ff0ef          	jal	ra,129e0 <__syscall_ret>
   12f30:	00813083          	ld	ra,8(sp)
   12f34:	0005051b          	sext.w	a0,a0
   12f38:	01010113          	addi	sp,sp,16
   12f3c:	00008067          	ret

0000000000012f40 <__stdio_seek>:
   12f40:	07852503          	lw	a0,120(a0)
   12f44:	0cd0006f          	j	13810 <__lseek>

0000000000012f48 <__stdout_write>:
   12f48:	fe010113          	addi	sp,sp,-32
   12f4c:	00060693          	mv	a3,a2
   12f50:	00014637          	lui	a2,0x14
   12f54:	a7860613          	addi	a2,a2,-1416 # 13a78 <__stdio_write>
   12f58:	00113c23          	sd	ra,24(sp)
   12f5c:	04c53423          	sd	a2,72(a0)
   12f60:	00052603          	lw	a2,0(a0)
   12f64:	00050793          	mv	a5,a0
   12f68:	00058713          	mv	a4,a1
   12f6c:	04067613          	andi	a2,a2,64
   12f70:	02061463          	bnez	a2,12f98 <__stdout_write+0x50>
   12f74:	000055b7          	lui	a1,0x5
   12f78:	07852503          	lw	a0,120(a0)
   12f7c:	01d00893          	li	a7,29
   12f80:	41358593          	addi	a1,a1,1043 # 5413 <exit-0xad0d>
   12f84:	00810613          	addi	a2,sp,8
   12f88:	00000073          	ecall
   12f8c:	00050663          	beqz	a0,12f98 <__stdout_write+0x50>
   12f90:	fff00613          	li	a2,-1
   12f94:	08c7a823          	sw	a2,144(a5)
   12f98:	00068613          	mv	a2,a3
   12f9c:	00070593          	mv	a1,a4
   12fa0:	00078513          	mv	a0,a5
   12fa4:	2d5000ef          	jal	ra,13a78 <__stdio_write>
   12fa8:	01813083          	ld	ra,24(sp)
   12fac:	02010113          	addi	sp,sp,32
   12fb0:	00008067          	ret

0000000000012fb4 <__towrite>:
   12fb4:	08852783          	lw	a5,136(a0)
   12fb8:	fff7871b          	addiw	a4,a5,-1
   12fbc:	00e7e7b3          	or	a5,a5,a4
   12fc0:	08f52423          	sw	a5,136(a0)
   12fc4:	00052783          	lw	a5,0(a0)
   12fc8:	0087f713          	andi	a4,a5,8
   12fcc:	00070a63          	beqz	a4,12fe0 <__towrite+0x2c>
   12fd0:	0207e793          	ori	a5,a5,32
   12fd4:	00f52023          	sw	a5,0(a0)
   12fd8:	fff00513          	li	a0,-1
   12fdc:	00008067          	ret
   12fe0:	05853783          	ld	a5,88(a0)
   12fe4:	06053703          	ld	a4,96(a0)
   12fe8:	00053823          	sd	zero,16(a0)
   12fec:	02f53c23          	sd	a5,56(a0)
   12ff0:	02f53423          	sd	a5,40(a0)
   12ff4:	00e787b3          	add	a5,a5,a4
   12ff8:	00053423          	sd	zero,8(a0)
   12ffc:	02f53023          	sd	a5,32(a0)
   13000:	00000513          	li	a0,0
   13004:	00008067          	ret

0000000000013008 <__towrite_needs_stdio_exit>:
   13008:	2250006f          	j	13a2c <__stdio_exit>

000000000001300c <__fwritex>:
   1300c:	02063783          	ld	a5,32(a2)
   13010:	fd010113          	addi	sp,sp,-48
   13014:	02813023          	sd	s0,32(sp)
   13018:	01213823          	sd	s2,16(sp)
   1301c:	01313423          	sd	s3,8(sp)
   13020:	02113423          	sd	ra,40(sp)
   13024:	00913c23          	sd	s1,24(sp)
   13028:	00050993          	mv	s3,a0
   1302c:	00058913          	mv	s2,a1
   13030:	00060413          	mv	s0,a2
   13034:	04078063          	beqz	a5,13074 <__fwritex+0x68>
   13038:	02043783          	ld	a5,32(s0)
   1303c:	02843703          	ld	a4,40(s0)
   13040:	40e787b3          	sub	a5,a5,a4
   13044:	0527f263          	bgeu	a5,s2,13088 <__fwritex+0x7c>
   13048:	04843783          	ld	a5,72(s0)
   1304c:	00040513          	mv	a0,s0
   13050:	02013403          	ld	s0,32(sp)
   13054:	02813083          	ld	ra,40(sp)
   13058:	01813483          	ld	s1,24(sp)
   1305c:	00090613          	mv	a2,s2
   13060:	00098593          	mv	a1,s3
   13064:	01013903          	ld	s2,16(sp)
   13068:	00813983          	ld	s3,8(sp)
   1306c:	03010113          	addi	sp,sp,48
   13070:	00078067          	jr	a5
   13074:	00060513          	mv	a0,a2
   13078:	f3dff0ef          	jal	ra,12fb4 <__towrite>
   1307c:	fa050ee3          	beqz	a0,13038 <__fwritex+0x2c>
   13080:	00000513          	li	a0,0
   13084:	0340006f          	j	130b8 <__fwritex+0xac>
   13088:	09042783          	lw	a5,144(s0)
   1308c:	00090493          	mv	s1,s2
   13090:	00a00713          	li	a4,10
   13094:	0407d263          	bgez	a5,130d8 <__fwritex+0xcc>
   13098:	02843503          	ld	a0,40(s0)
   1309c:	00048613          	mv	a2,s1
   130a0:	00098593          	mv	a1,s3
   130a4:	1b0000ef          	jal	ra,13254 <memcpy>
   130a8:	02843603          	ld	a2,40(s0)
   130ac:	00090513          	mv	a0,s2
   130b0:	009604b3          	add	s1,a2,s1
   130b4:	02943423          	sd	s1,40(s0)
   130b8:	02813083          	ld	ra,40(sp)
   130bc:	02013403          	ld	s0,32(sp)
   130c0:	01813483          	ld	s1,24(sp)
   130c4:	01013903          	ld	s2,16(sp)
   130c8:	00813983          	ld	s3,8(sp)
   130cc:	03010113          	addi	sp,sp,48
   130d0:	00008067          	ret
   130d4:	00078493          	mv	s1,a5
   130d8:	02048c63          	beqz	s1,13110 <__fwritex+0x104>
   130dc:	fff48793          	addi	a5,s1,-1
   130e0:	00f986b3          	add	a3,s3,a5
   130e4:	0006c683          	lbu	a3,0(a3)
   130e8:	fee696e3          	bne	a3,a4,130d4 <__fwritex+0xc8>
   130ec:	04843783          	ld	a5,72(s0)
   130f0:	00048613          	mv	a2,s1
   130f4:	00098593          	mv	a1,s3
   130f8:	00040513          	mv	a0,s0
   130fc:	000780e7          	jalr	a5
   13100:	fa956ce3          	bltu	a0,s1,130b8 <__fwritex+0xac>
   13104:	009989b3          	add	s3,s3,s1
   13108:	409904b3          	sub	s1,s2,s1
   1310c:	f8dff06f          	j	13098 <__fwritex+0x8c>
   13110:	00090493          	mv	s1,s2
   13114:	f85ff06f          	j	13098 <__fwritex+0x8c>

0000000000013118 <fwrite>:
   13118:	fc010113          	addi	sp,sp,-64
   1311c:	02813823          	sd	s0,48(sp)
   13120:	02913423          	sd	s1,40(sp)
   13124:	03213023          	sd	s2,32(sp)
   13128:	01313c23          	sd	s3,24(sp)
   1312c:	01513423          	sd	s5,8(sp)
   13130:	02113c23          	sd	ra,56(sp)
   13134:	01413823          	sd	s4,16(sp)
   13138:	02c58ab3          	mul	s5,a1,a2
   1313c:	00050993          	mv	s3,a0
   13140:	00058913          	mv	s2,a1
   13144:	00068493          	mv	s1,a3
   13148:	00000413          	li	s0,0
   1314c:	00058463          	beqz	a1,13154 <fwrite+0x3c>
   13150:	00060413          	mv	s0,a2
   13154:	08c4a783          	lw	a5,140(s1)
   13158:	00000a13          	li	s4,0
   1315c:	0007879b          	sext.w	a5,a5
   13160:	0007c863          	bltz	a5,13170 <fwrite+0x58>
   13164:	00048513          	mv	a0,s1
   13168:	c75ff0ef          	jal	ra,12ddc <__lockfile>
   1316c:	00050a13          	mv	s4,a0
   13170:	00098513          	mv	a0,s3
   13174:	00048613          	mv	a2,s1
   13178:	000a8593          	mv	a1,s5
   1317c:	e91ff0ef          	jal	ra,1300c <__fwritex>
   13180:	00050993          	mv	s3,a0
   13184:	000a0663          	beqz	s4,13190 <fwrite+0x78>
   13188:	00048513          	mv	a0,s1
   1318c:	d25ff0ef          	jal	ra,12eb0 <__unlockfile>
   13190:	013a8463          	beq	s5,s3,13198 <fwrite+0x80>
   13194:	0329d433          	divu	s0,s3,s2
   13198:	03813083          	ld	ra,56(sp)
   1319c:	00040513          	mv	a0,s0
   131a0:	03013403          	ld	s0,48(sp)
   131a4:	02813483          	ld	s1,40(sp)
   131a8:	02013903          	ld	s2,32(sp)
   131ac:	01813983          	ld	s3,24(sp)
   131b0:	01013a03          	ld	s4,16(sp)
   131b4:	00813a83          	ld	s5,8(sp)
   131b8:	04010113          	addi	sp,sp,64
   131bc:	00008067          	ret

00000000000131c0 <memchr>:
   131c0:	0ff5f593          	andi	a1,a1,255
   131c4:	00757793          	andi	a5,a0,7
   131c8:	02078863          	beqz	a5,131f8 <memchr+0x38>
   131cc:	08060063          	beqz	a2,1324c <memchr+0x8c>
   131d0:	00054783          	lbu	a5,0(a0)
   131d4:	00b79c63          	bne	a5,a1,131ec <memchr+0x2c>
   131d8:	00c50633          	add	a2,a0,a2
   131dc:	06c50863          	beq	a0,a2,1324c <memchr+0x8c>
   131e0:	00054783          	lbu	a5,0(a0)
   131e4:	06b79063          	bne	a5,a1,13244 <memchr+0x84>
   131e8:	00008067          	ret
   131ec:	00150513          	addi	a0,a0,1
   131f0:	fff60613          	addi	a2,a2,-1
   131f4:	fd1ff06f          	j	131c4 <memchr+0x4>
   131f8:	04060a63          	beqz	a2,1324c <memchr+0x8c>
   131fc:	00054783          	lbu	a5,0(a0)
   13200:	fcb78ce3          	beq	a5,a1,131d8 <memchr+0x18>
   13204:	9081b683          	ld	a3,-1784(gp) # 18108 <__SDATA_BEGIN__>
   13208:	9101b883          	ld	a7,-1776(gp) # 18110 <__SDATA_BEGIN__+0x8>
   1320c:	02d586b3          	mul	a3,a1,a3
   13210:	9181b803          	ld	a6,-1768(gp) # 18118 <__SDATA_BEGIN__+0x10>
   13214:	00700313          	li	t1,7
   13218:	fcc370e3          	bgeu	t1,a2,131d8 <memchr+0x18>
   1321c:	00053783          	ld	a5,0(a0)
   13220:	00f6c7b3          	xor	a5,a3,a5
   13224:	01178733          	add	a4,a5,a7
   13228:	fff7c793          	not	a5,a5
   1322c:	00f777b3          	and	a5,a4,a5
   13230:	0107f7b3          	and	a5,a5,a6
   13234:	fa0792e3          	bnez	a5,131d8 <memchr+0x18>
   13238:	00850513          	addi	a0,a0,8
   1323c:	ff860613          	addi	a2,a2,-8
   13240:	fd9ff06f          	j	13218 <memchr+0x58>
   13244:	00150513          	addi	a0,a0,1
   13248:	f95ff06f          	j	131dc <memchr+0x1c>
   1324c:	00000513          	li	a0,0
   13250:	00008067          	ret

0000000000013254 <memcpy>:
   13254:	00050793          	mv	a5,a0
   13258:	0035f713          	andi	a4,a1,3
   1325c:	00070463          	beqz	a4,13264 <memcpy+0x10>
   13260:	02061063          	bnez	a2,13280 <memcpy+0x2c>
   13264:	0037f693          	andi	a3,a5,3
   13268:	0e069463          	bnez	a3,13350 <memcpy+0xfc>
   1326c:	00058713          	mv	a4,a1
   13270:	00078693          	mv	a3,a5
   13274:	00c58333          	add	t1,a1,a2
   13278:	00f00813          	li	a6,15
   1327c:	0440006f          	j	132c0 <memcpy+0x6c>
   13280:	0005c703          	lbu	a4,0(a1)
   13284:	00158593          	addi	a1,a1,1
   13288:	00178793          	addi	a5,a5,1
   1328c:	fff60613          	addi	a2,a2,-1
   13290:	fee78fa3          	sb	a4,-1(a5)
   13294:	fc5ff06f          	j	13258 <memcpy+0x4>
   13298:	00072883          	lw	a7,0(a4)
   1329c:	01068693          	addi	a3,a3,16
   132a0:	01070713          	addi	a4,a4,16
   132a4:	ff16a823          	sw	a7,-16(a3)
   132a8:	ff472883          	lw	a7,-12(a4)
   132ac:	ff16aa23          	sw	a7,-12(a3)
   132b0:	ff872883          	lw	a7,-8(a4)
   132b4:	ff16ac23          	sw	a7,-8(a3)
   132b8:	ffc72883          	lw	a7,-4(a4)
   132bc:	ff16ae23          	sw	a7,-4(a3)
   132c0:	40e308b3          	sub	a7,t1,a4
   132c4:	fd186ae3          	bltu	a6,a7,13298 <memcpy+0x44>
   132c8:	00465713          	srli	a4,a2,0x4
   132cc:	ff000693          	li	a3,-16
   132d0:	02d706b3          	mul	a3,a4,a3
   132d4:	00471713          	slli	a4,a4,0x4
   132d8:	00e787b3          	add	a5,a5,a4
   132dc:	00e585b3          	add	a1,a1,a4
   132e0:	00c686b3          	add	a3,a3,a2
   132e4:	00867613          	andi	a2,a2,8
   132e8:	00060e63          	beqz	a2,13304 <memcpy+0xb0>
   132ec:	0005a703          	lw	a4,0(a1)
   132f0:	00878793          	addi	a5,a5,8
   132f4:	00858593          	addi	a1,a1,8
   132f8:	fee7ac23          	sw	a4,-8(a5)
   132fc:	ffc5a703          	lw	a4,-4(a1)
   13300:	fee7ae23          	sw	a4,-4(a5)
   13304:	0046f713          	andi	a4,a3,4
   13308:	00070a63          	beqz	a4,1331c <memcpy+0xc8>
   1330c:	0005a703          	lw	a4,0(a1)
   13310:	00478793          	addi	a5,a5,4
   13314:	00458593          	addi	a1,a1,4
   13318:	fee7ae23          	sw	a4,-4(a5)
   1331c:	0026f713          	andi	a4,a3,2
   13320:	00070e63          	beqz	a4,1333c <memcpy+0xe8>
   13324:	0005c703          	lbu	a4,0(a1)
   13328:	00278793          	addi	a5,a5,2
   1332c:	00258593          	addi	a1,a1,2
   13330:	fee78f23          	sb	a4,-2(a5)
   13334:	fff5c703          	lbu	a4,-1(a1)
   13338:	fee78fa3          	sb	a4,-1(a5)
   1333c:	0016f693          	andi	a3,a3,1
   13340:	20068e63          	beqz	a3,1355c <memcpy+0x308>
   13344:	0005c703          	lbu	a4,0(a1)
   13348:	00e78023          	sb	a4,0(a5)
   1334c:	2100006f          	j	1355c <memcpy+0x308>
   13350:	01f00713          	li	a4,31
   13354:	0cc77863          	bgeu	a4,a2,13424 <memcpy+0x1d0>
   13358:	00200313          	li	t1,2
   1335c:	0005c703          	lbu	a4,0(a1)
   13360:	0005a803          	lw	a6,0(a1)
   13364:	00c788b3          	add	a7,a5,a2
   13368:	1e668c63          	beq	a3,t1,13560 <memcpy+0x30c>
   1336c:	00300313          	li	t1,3
   13370:	28668863          	beq	a3,t1,13600 <memcpy+0x3ac>
   13374:	00e78023          	sb	a4,0(a5)
   13378:	0015c703          	lbu	a4,1(a1)
   1337c:	00358e13          	addi	t3,a1,3
   13380:	00378313          	addi	t1,a5,3
   13384:	00e780a3          	sb	a4,1(a5)
   13388:	0025c703          	lbu	a4,2(a1)
   1338c:	01000e93          	li	t4,16
   13390:	00e78123          	sb	a4,2(a5)
   13394:	000e0713          	mv	a4,t3
   13398:	00030793          	mv	a5,t1
   1339c:	00172683          	lw	a3,1(a4)
   133a0:	0188581b          	srliw	a6,a6,0x18
   133a4:	01078793          	addi	a5,a5,16
   133a8:	0086959b          	slliw	a1,a3,0x8
   133ac:	00b86833          	or	a6,a6,a1
   133b0:	00572583          	lw	a1,5(a4)
   133b4:	ff07a823          	sw	a6,-16(a5)
   133b8:	0186d69b          	srliw	a3,a3,0x18
   133bc:	0085981b          	slliw	a6,a1,0x8
   133c0:	0106e6b3          	or	a3,a3,a6
   133c4:	fed7aa23          	sw	a3,-12(a5)
   133c8:	00972683          	lw	a3,9(a4)
   133cc:	0185d59b          	srliw	a1,a1,0x18
   133d0:	01070713          	addi	a4,a4,16
   133d4:	0086981b          	slliw	a6,a3,0x8
   133d8:	0105e5b3          	or	a1,a1,a6
   133dc:	ffd72803          	lw	a6,-3(a4)
   133e0:	feb7ac23          	sw	a1,-8(a5)
   133e4:	0186d69b          	srliw	a3,a3,0x18
   133e8:	0088159b          	slliw	a1,a6,0x8
   133ec:	00b6e6b3          	or	a3,a3,a1
   133f0:	fed7ae23          	sw	a3,-4(a5)
   133f4:	40f886b3          	sub	a3,a7,a5
   133f8:	fadee2e3          	bltu	t4,a3,1339c <memcpy+0x148>
   133fc:	fec60713          	addi	a4,a2,-20
   13400:	00475713          	srli	a4,a4,0x4
   13404:	00170793          	addi	a5,a4,1
   13408:	00479793          	slli	a5,a5,0x4
   1340c:	00fe05b3          	add	a1,t3,a5
   13410:	fed60613          	addi	a2,a2,-19
   13414:	00f307b3          	add	a5,t1,a5
   13418:	ff000693          	li	a3,-16
   1341c:	02d70733          	mul	a4,a4,a3
   13420:	00c70633          	add	a2,a4,a2
   13424:	01067713          	andi	a4,a2,16
   13428:	08070663          	beqz	a4,134b4 <memcpy+0x260>
   1342c:	0005c703          	lbu	a4,0(a1)
   13430:	01078793          	addi	a5,a5,16
   13434:	01058593          	addi	a1,a1,16
   13438:	fee78823          	sb	a4,-16(a5)
   1343c:	ff15c703          	lbu	a4,-15(a1)
   13440:	fee788a3          	sb	a4,-15(a5)
   13444:	ff25c703          	lbu	a4,-14(a1)
   13448:	fee78923          	sb	a4,-14(a5)
   1344c:	ff35c703          	lbu	a4,-13(a1)
   13450:	fee789a3          	sb	a4,-13(a5)
   13454:	ff45c703          	lbu	a4,-12(a1)
   13458:	fee78a23          	sb	a4,-12(a5)
   1345c:	ff55c703          	lbu	a4,-11(a1)
   13460:	fee78aa3          	sb	a4,-11(a5)
   13464:	ff65c703          	lbu	a4,-10(a1)
   13468:	fee78b23          	sb	a4,-10(a5)
   1346c:	ff75c703          	lbu	a4,-9(a1)
   13470:	fee78ba3          	sb	a4,-9(a5)
   13474:	ff85c703          	lbu	a4,-8(a1)
   13478:	fee78c23          	sb	a4,-8(a5)
   1347c:	ff95c703          	lbu	a4,-7(a1)
   13480:	fee78ca3          	sb	a4,-7(a5)
   13484:	ffa5c703          	lbu	a4,-6(a1)
   13488:	fee78d23          	sb	a4,-6(a5)
   1348c:	ffb5c703          	lbu	a4,-5(a1)
   13490:	fee78da3          	sb	a4,-5(a5)
   13494:	ffc5c703          	lbu	a4,-4(a1)
   13498:	fee78e23          	sb	a4,-4(a5)
   1349c:	ffd5c703          	lbu	a4,-3(a1)
   134a0:	fee78ea3          	sb	a4,-3(a5)
   134a4:	ffe5c703          	lbu	a4,-2(a1)
   134a8:	fee78f23          	sb	a4,-2(a5)
   134ac:	fff5c703          	lbu	a4,-1(a1)
   134b0:	fee78fa3          	sb	a4,-1(a5)
   134b4:	00867713          	andi	a4,a2,8
   134b8:	04070663          	beqz	a4,13504 <memcpy+0x2b0>
   134bc:	0005c703          	lbu	a4,0(a1)
   134c0:	00878793          	addi	a5,a5,8
   134c4:	00858593          	addi	a1,a1,8
   134c8:	fee78c23          	sb	a4,-8(a5)
   134cc:	ff95c703          	lbu	a4,-7(a1)
   134d0:	fee78ca3          	sb	a4,-7(a5)
   134d4:	ffa5c703          	lbu	a4,-6(a1)
   134d8:	fee78d23          	sb	a4,-6(a5)
   134dc:	ffb5c703          	lbu	a4,-5(a1)
   134e0:	fee78da3          	sb	a4,-5(a5)
   134e4:	ffc5c703          	lbu	a4,-4(a1)
   134e8:	fee78e23          	sb	a4,-4(a5)
   134ec:	ffd5c703          	lbu	a4,-3(a1)
   134f0:	fee78ea3          	sb	a4,-3(a5)
   134f4:	ffe5c703          	lbu	a4,-2(a1)
   134f8:	fee78f23          	sb	a4,-2(a5)
   134fc:	fff5c703          	lbu	a4,-1(a1)
   13500:	fee78fa3          	sb	a4,-1(a5)
   13504:	00467713          	andi	a4,a2,4
   13508:	02070663          	beqz	a4,13534 <memcpy+0x2e0>
   1350c:	0005c703          	lbu	a4,0(a1)
   13510:	00478793          	addi	a5,a5,4
   13514:	00458593          	addi	a1,a1,4
   13518:	fee78e23          	sb	a4,-4(a5)
   1351c:	ffd5c703          	lbu	a4,-3(a1)
   13520:	fee78ea3          	sb	a4,-3(a5)
   13524:	ffe5c703          	lbu	a4,-2(a1)
   13528:	fee78f23          	sb	a4,-2(a5)
   1352c:	fff5c703          	lbu	a4,-1(a1)
   13530:	fee78fa3          	sb	a4,-1(a5)
   13534:	00267713          	andi	a4,a2,2
   13538:	00070e63          	beqz	a4,13554 <memcpy+0x300>
   1353c:	0005c703          	lbu	a4,0(a1)
   13540:	00278793          	addi	a5,a5,2
   13544:	00258593          	addi	a1,a1,2
   13548:	fee78f23          	sb	a4,-2(a5)
   1354c:	fff5c703          	lbu	a4,-1(a1)
   13550:	fee78fa3          	sb	a4,-1(a5)
   13554:	00167613          	andi	a2,a2,1
   13558:	de0616e3          	bnez	a2,13344 <memcpy+0xf0>
   1355c:	00008067          	ret
   13560:	00e78023          	sb	a4,0(a5)
   13564:	0015c703          	lbu	a4,1(a1)
   13568:	00258e13          	addi	t3,a1,2
   1356c:	00278313          	addi	t1,a5,2
   13570:	00e780a3          	sb	a4,1(a5)
   13574:	01100e93          	li	t4,17
   13578:	000e0713          	mv	a4,t3
   1357c:	00030793          	mv	a5,t1
   13580:	00272683          	lw	a3,2(a4)
   13584:	0108581b          	srliw	a6,a6,0x10
   13588:	01078793          	addi	a5,a5,16
   1358c:	0106959b          	slliw	a1,a3,0x10
   13590:	00b86833          	or	a6,a6,a1
   13594:	00672583          	lw	a1,6(a4)
   13598:	ff07a823          	sw	a6,-16(a5)
   1359c:	0106d69b          	srliw	a3,a3,0x10
   135a0:	0105981b          	slliw	a6,a1,0x10
   135a4:	0106e6b3          	or	a3,a3,a6
   135a8:	fed7aa23          	sw	a3,-12(a5)
   135ac:	00a72683          	lw	a3,10(a4)
   135b0:	0105d59b          	srliw	a1,a1,0x10
   135b4:	01070713          	addi	a4,a4,16
   135b8:	0106981b          	slliw	a6,a3,0x10
   135bc:	0105e5b3          	or	a1,a1,a6
   135c0:	ffe72803          	lw	a6,-2(a4)
   135c4:	feb7ac23          	sw	a1,-8(a5)
   135c8:	0106d69b          	srliw	a3,a3,0x10
   135cc:	0108159b          	slliw	a1,a6,0x10
   135d0:	00b6e6b3          	or	a3,a3,a1
   135d4:	fed7ae23          	sw	a3,-4(a5)
   135d8:	40f886b3          	sub	a3,a7,a5
   135dc:	fadee2e3          	bltu	t4,a3,13580 <memcpy+0x32c>
   135e0:	fec60713          	addi	a4,a2,-20
   135e4:	00475713          	srli	a4,a4,0x4
   135e8:	00170793          	addi	a5,a4,1
   135ec:	00479793          	slli	a5,a5,0x4
   135f0:	00fe05b3          	add	a1,t3,a5
   135f4:	fee60613          	addi	a2,a2,-18
   135f8:	00f307b3          	add	a5,t1,a5
   135fc:	e1dff06f          	j	13418 <memcpy+0x1c4>
   13600:	00158593          	addi	a1,a1,1
   13604:	00178313          	addi	t1,a5,1
   13608:	00e78023          	sb	a4,0(a5)
   1360c:	01200e13          	li	t3,18
   13610:	00058713          	mv	a4,a1
   13614:	00030793          	mv	a5,t1
   13618:	00372683          	lw	a3,3(a4)
   1361c:	0088581b          	srliw	a6,a6,0x8
   13620:	01078793          	addi	a5,a5,16
   13624:	01869e9b          	slliw	t4,a3,0x18
   13628:	01d86833          	or	a6,a6,t4
   1362c:	ff07a823          	sw	a6,-16(a5)
   13630:	00772803          	lw	a6,7(a4)
   13634:	0086d69b          	srliw	a3,a3,0x8
   13638:	01070713          	addi	a4,a4,16
   1363c:	01881e9b          	slliw	t4,a6,0x18
   13640:	01d6e6b3          	or	a3,a3,t4
   13644:	fed7aa23          	sw	a3,-12(a5)
   13648:	ffb72683          	lw	a3,-5(a4)
   1364c:	0088581b          	srliw	a6,a6,0x8
   13650:	01869e9b          	slliw	t4,a3,0x18
   13654:	01d86833          	or	a6,a6,t4
   13658:	ff07ac23          	sw	a6,-8(a5)
   1365c:	fff72803          	lw	a6,-1(a4)
   13660:	0086d69b          	srliw	a3,a3,0x8
   13664:	01881e9b          	slliw	t4,a6,0x18
   13668:	01d6e6b3          	or	a3,a3,t4
   1366c:	fed7ae23          	sw	a3,-4(a5)
   13670:	40f886b3          	sub	a3,a7,a5
   13674:	fade62e3          	bltu	t3,a3,13618 <memcpy+0x3c4>
   13678:	fec60713          	addi	a4,a2,-20
   1367c:	00475713          	srli	a4,a4,0x4
   13680:	00170793          	addi	a5,a4,1
   13684:	00479793          	slli	a5,a5,0x4
   13688:	00f585b3          	add	a1,a1,a5
   1368c:	fef60613          	addi	a2,a2,-17
   13690:	00f307b3          	add	a5,t1,a5
   13694:	d85ff06f          	j	13418 <memcpy+0x1c4>

0000000000013698 <__lock>:
   13698:	9f818793          	addi	a5,gp,-1544 # 181f8 <__libc>
   1369c:	00c7a783          	lw	a5,12(a5)
   136a0:	00050713          	mv	a4,a0
   136a4:	0007879b          	sext.w	a5,a5
   136a8:	0e078263          	beqz	a5,1378c <__lock+0xf4>
   136ac:	800006b7          	lui	a3,0x80000
   136b0:	00168693          	addi	a3,a3,1 # ffffffff80000001 <__BSS_END__+0xffffffff7ffe77f9>
   136b4:	00000613          	li	a2,0
   136b8:	160527af          	lr.w.aqrl	a5,(a0)
   136bc:	00c79663          	bne	a5,a2,136c8 <__lock+0x30>
   136c0:	1ed525af          	sc.w.aqrl	a1,a3,(a0)
   136c4:	fe059ae3          	bnez	a1,136b8 <__lock+0x20>
   136c8:	0007879b          	sext.w	a5,a5
   136cc:	0c078063          	beqz	a5,1378c <__lock+0xf4>
   136d0:	80000537          	lui	a0,0x80000
   136d4:	00a00593          	li	a1,10
   136d8:	fff54513          	not	a0,a0
   136dc:	00078613          	mv	a2,a5
   136e0:	0007d463          	bgez	a5,136e8 <__lock+0x50>
   136e4:	00f5063b          	addw	a2,a0,a5
   136e8:	00c6883b          	addw	a6,a3,a2
   136ec:	160727af          	lr.w.aqrl	a5,(a4)
   136f0:	00c79663          	bne	a5,a2,136fc <__lock+0x64>
   136f4:	1f0728af          	sc.w.aqrl	a7,a6,(a4)
   136f8:	fe089ae3          	bnez	a7,136ec <__lock+0x54>
   136fc:	0007879b          	sext.w	a5,a5
   13700:	08f60663          	beq	a2,a5,1378c <__lock+0xf4>
   13704:	fff5859b          	addiw	a1,a1,-1
   13708:	fc059ae3          	bnez	a1,136dc <__lock+0x44>
   1370c:	00072783          	lw	a5,0(a4)
   13710:	0007861b          	sext.w	a2,a5
   13714:	0017879b          	addiw	a5,a5,1
   13718:	160726af          	lr.w.aqrl	a3,(a4)
   1371c:	00c69663          	bne	a3,a2,13728 <__lock+0x90>
   13720:	1ef725af          	sc.w.aqrl	a1,a5,(a4)
   13724:	fe059ae3          	bnez	a1,13718 <__lock+0x80>
   13728:	0006869b          	sext.w	a3,a3
   1372c:	fed610e3          	bne	a2,a3,1370c <__lock+0x74>
   13730:	80000837          	lui	a6,0x80000
   13734:	fda00313          	li	t1,-38
   13738:	fff84e13          	not	t3,a6
   1373c:	00078693          	mv	a3,a5
   13740:	0207d863          	bgez	a5,13770 <__lock+0xd8>
   13744:	06200893          	li	a7,98
   13748:	00070513          	mv	a0,a4
   1374c:	08000593          	li	a1,128
   13750:	00078613          	mv	a2,a5
   13754:	00000693          	li	a3,0
   13758:	00000073          	ecall
   1375c:	00651863          	bne	a0,t1,1376c <__lock+0xd4>
   13760:	00070513          	mv	a0,a4
   13764:	00000593          	li	a1,0
   13768:	00000073          	ecall
   1376c:	00fe06bb          	addw	a3,t3,a5
   13770:	00d8063b          	addw	a2,a6,a3
   13774:	160727af          	lr.w.aqrl	a5,(a4)
   13778:	00d79663          	bne	a5,a3,13784 <__lock+0xec>
   1377c:	1ec725af          	sc.w.aqrl	a1,a2,(a4)
   13780:	fe059ae3          	bnez	a1,13774 <__lock+0xdc>
   13784:	0007879b          	sext.w	a5,a5
   13788:	faf69ae3          	bne	a3,a5,1373c <__lock+0xa4>
   1378c:	00008067          	ret

0000000000013790 <__unlock>:
   13790:	00052703          	lw	a4,0(a0) # ffffffff80000000 <__BSS_END__+0xffffffff7ffe77f8>
   13794:	00050793          	mv	a5,a0
   13798:	0007071b          	sext.w	a4,a4
   1379c:	06075263          	bgez	a4,13800 <__unlock+0x70>
   137a0:	800005b7          	lui	a1,0x80000
   137a4:	fff5c593          	not	a1,a1
   137a8:	0007a703          	lw	a4,0(a5)
   137ac:	0007069b          	sext.w	a3,a4
   137b0:	00b7073b          	addw	a4,a4,a1
   137b4:	1607a62f          	lr.w.aqrl	a2,(a5)
   137b8:	00d61663          	bne	a2,a3,137c4 <__unlock+0x34>
   137bc:	1ee7a52f          	sc.w.aqrl	a0,a4,(a5)
   137c0:	fe051ae3          	bnez	a0,137b4 <__unlock+0x24>
   137c4:	0006071b          	sext.w	a4,a2
   137c8:	fee690e3          	bne	a3,a4,137a8 <__unlock+0x18>
   137cc:	80000737          	lui	a4,0x80000
   137d0:	00170713          	addi	a4,a4,1 # ffffffff80000001 <__BSS_END__+0xffffffff7ffe77f9>
   137d4:	02e68663          	beq	a3,a4,13800 <__unlock+0x70>
   137d8:	06200893          	li	a7,98
   137dc:	00078513          	mv	a0,a5
   137e0:	08100593          	li	a1,129
   137e4:	00100613          	li	a2,1
   137e8:	00000073          	ecall
   137ec:	fda00713          	li	a4,-38
   137f0:	00e51863          	bne	a0,a4,13800 <__unlock+0x70>
   137f4:	00078513          	mv	a0,a5
   137f8:	00100593          	li	a1,1
   137fc:	00000073          	ecall
   13800:	00008067          	ret

0000000000013804 <__set_thread_area>:
   13804:	00050213          	mv	tp,a0
   13808:	00000513          	li	a0,0
   1380c:	00008067          	ret

0000000000013810 <__lseek>:
   13810:	03e00893          	li	a7,62
   13814:	00000073          	ecall
   13818:	9c8ff06f          	j	129e0 <__syscall_ret>

000000000001381c <wcrtomb>:
   1381c:	00050a63          	beqz	a0,13830 <wcrtomb+0x14>
   13820:	07f00693          	li	a3,127
   13824:	0005879b          	sext.w	a5,a1
   13828:	02b6e263          	bltu	a3,a1,1384c <wcrtomb+0x30>
   1382c:	00b50023          	sb	a1,0(a0)
   13830:	00100513          	li	a0,1
   13834:	00008067          	ret
   13838:	00b50023          	sb	a1,0(a0)
   1383c:	00100513          	li	a0,1
   13840:	00813083          	ld	ra,8(sp)
   13844:	01010113          	addi	sp,sp,16
   13848:	00008067          	ret
   1384c:	ff010113          	addi	sp,sp,-16
   13850:	00113423          	sd	ra,8(sp)
   13854:	00020713          	mv	a4,tp
   13858:	fd073703          	ld	a4,-48(a4)
   1385c:	00073703          	ld	a4,0(a4)
   13860:	02071463          	bnez	a4,13888 <wcrtomb+0x6c>
   13864:	ffff2737          	lui	a4,0xffff2
   13868:	0807071b          	addiw	a4,a4,128
   1386c:	00f707bb          	addw	a5,a4,a5
   13870:	fcf6f4e3          	bgeu	a3,a5,13838 <wcrtomb+0x1c>
   13874:	8f0ff0ef          	jal	ra,12964 <__errno_location>
   13878:	05400793          	li	a5,84
   1387c:	00f52023          	sw	a5,0(a0)
   13880:	fff00513          	li	a0,-1
   13884:	fbdff06f          	j	13840 <wcrtomb+0x24>
   13888:	7ff00713          	li	a4,2047
   1388c:	02f76263          	bltu	a4,a5,138b0 <wcrtomb+0x94>
   13890:	4065d79b          	sraiw	a5,a1,0x6
   13894:	03f5f593          	andi	a1,a1,63
   13898:	fc07e793          	ori	a5,a5,-64
   1389c:	f805e593          	ori	a1,a1,-128
   138a0:	00f50023          	sb	a5,0(a0)
   138a4:	00b500a3          	sb	a1,1(a0)
   138a8:	00200513          	li	a0,2
   138ac:	f95ff06f          	j	13840 <wcrtomb+0x24>
   138b0:	0000d737          	lui	a4,0xd
   138b4:	7ff70713          	addi	a4,a4,2047 # d7ff <exit-0x2921>
   138b8:	00f77a63          	bgeu	a4,a5,138cc <wcrtomb+0xb0>
   138bc:	ffff2737          	lui	a4,0xffff2
   138c0:	00f7073b          	addw	a4,a4,a5
   138c4:	000026b7          	lui	a3,0x2
   138c8:	02d77a63          	bgeu	a4,a3,138fc <wcrtomb+0xe0>
   138cc:	40c5d79b          	sraiw	a5,a1,0xc
   138d0:	fe07e793          	ori	a5,a5,-32
   138d4:	00f50023          	sb	a5,0(a0)
   138d8:	4065d79b          	sraiw	a5,a1,0x6
   138dc:	03f7f793          	andi	a5,a5,63
   138e0:	03f5f593          	andi	a1,a1,63
   138e4:	f807e793          	ori	a5,a5,-128
   138e8:	f805e593          	ori	a1,a1,-128
   138ec:	00f500a3          	sb	a5,1(a0)
   138f0:	00b50123          	sb	a1,2(a0)
   138f4:	00300513          	li	a0,3
   138f8:	f49ff06f          	j	13840 <wcrtomb+0x24>
   138fc:	ffff0737          	lui	a4,0xffff0
   13900:	00f707bb          	addw	a5,a4,a5
   13904:	00100737          	lui	a4,0x100
   13908:	f6e7f6e3          	bgeu	a5,a4,13874 <wcrtomb+0x58>
   1390c:	4125d79b          	sraiw	a5,a1,0x12
   13910:	ff07e793          	ori	a5,a5,-16
   13914:	00f50023          	sb	a5,0(a0)
   13918:	40c5d79b          	sraiw	a5,a1,0xc
   1391c:	03f7f793          	andi	a5,a5,63
   13920:	f807e793          	ori	a5,a5,-128
   13924:	00f500a3          	sb	a5,1(a0)
   13928:	4065d79b          	sraiw	a5,a1,0x6
   1392c:	03f7f793          	andi	a5,a5,63
   13930:	03f5f593          	andi	a1,a1,63
   13934:	f807e793          	ori	a5,a5,-128
   13938:	f805e593          	ori	a1,a1,-128
   1393c:	00f50123          	sb	a5,2(a0)
   13940:	00b501a3          	sb	a1,3(a0)
   13944:	00400513          	li	a0,4
   13948:	ef9ff06f          	j	13840 <wcrtomb+0x24>

000000000001394c <__block_all_sigs>:
   1394c:	00050613          	mv	a2,a0
   13950:	08700893          	li	a7,135
   13954:	00000513          	li	a0,0
   13958:	92818593          	addi	a1,gp,-1752 # 18128 <all_mask>
   1395c:	00800693          	li	a3,8
   13960:	00000073          	ecall
   13964:	00008067          	ret

0000000000013968 <__block_app_sigs>:
   13968:	00050613          	mv	a2,a0
   1396c:	08700893          	li	a7,135
   13970:	00000513          	li	a0,0
   13974:	93018593          	addi	a1,gp,-1744 # 18130 <app_mask>
   13978:	00800693          	li	a3,8
   1397c:	00000073          	ecall
   13980:	00008067          	ret

0000000000013984 <__restore_sigs>:
   13984:	00050593          	mv	a1,a0
   13988:	08700893          	li	a7,135
   1398c:	00200513          	li	a0,2
   13990:	00000613          	li	a2,0
   13994:	00800693          	li	a3,8
   13998:	00000073          	ecall
   1399c:	00008067          	ret

00000000000139a0 <__restore>:
   139a0:	08b00893          	li	a7,139
   139a4:	00000073          	ecall

00000000000139a8 <close_file>:
   139a8:	08050063          	beqz	a0,13a28 <close_file+0x80>
   139ac:	08c52783          	lw	a5,140(a0)
   139b0:	ff010113          	addi	sp,sp,-16
   139b4:	00813023          	sd	s0,0(sp)
   139b8:	00113423          	sd	ra,8(sp)
   139bc:	0007879b          	sext.w	a5,a5
   139c0:	00050413          	mv	s0,a0
   139c4:	0007c463          	bltz	a5,139cc <close_file+0x24>
   139c8:	c14ff0ef          	jal	ra,12ddc <__lockfile>
   139cc:	02843703          	ld	a4,40(s0)
   139d0:	03843783          	ld	a5,56(s0)
   139d4:	00f70c63          	beq	a4,a5,139ec <close_file+0x44>
   139d8:	04843783          	ld	a5,72(s0)
   139dc:	00000613          	li	a2,0
   139e0:	00000593          	li	a1,0
   139e4:	00040513          	mv	a0,s0
   139e8:	000780e7          	jalr	a5
   139ec:	00843583          	ld	a1,8(s0)
   139f0:	01043783          	ld	a5,16(s0)
   139f4:	02f58263          	beq	a1,a5,13a18 <close_file+0x70>
   139f8:	05043703          	ld	a4,80(s0)
   139fc:	00040513          	mv	a0,s0
   13a00:	00013403          	ld	s0,0(sp)
   13a04:	00813083          	ld	ra,8(sp)
   13a08:	00100613          	li	a2,1
   13a0c:	40f585b3          	sub	a1,a1,a5
   13a10:	01010113          	addi	sp,sp,16
   13a14:	00070067          	jr	a4 # 100000 <__BSS_END__+0xe77f8>
   13a18:	00813083          	ld	ra,8(sp)
   13a1c:	00013403          	ld	s0,0(sp)
   13a20:	01010113          	addi	sp,sp,16
   13a24:	00008067          	ret
   13a28:	00008067          	ret

0000000000013a2c <__stdio_exit>:
   13a2c:	ff010113          	addi	sp,sp,-16
   13a30:	00813023          	sd	s0,0(sp)
   13a34:	00113423          	sd	ra,8(sp)
   13a38:	150000ef          	jal	ra,13b88 <__ofl_lock>
   13a3c:	00053403          	ld	s0,0(a0)
   13a40:	02041463          	bnez	s0,13a68 <__stdio_exit+0x3c>
   13a44:	9a81b503          	ld	a0,-1624(gp) # 181a8 <__stderr_used>
   13a48:	f61ff0ef          	jal	ra,139a8 <close_file>
   13a4c:	9401b503          	ld	a0,-1728(gp) # 18140 <__stdout_used>
   13a50:	f59ff0ef          	jal	ra,139a8 <close_file>
   13a54:	00013403          	ld	s0,0(sp)
   13a58:	00813083          	ld	ra,8(sp)
   13a5c:	9a81b503          	ld	a0,-1624(gp) # 181a8 <__stderr_used>
   13a60:	01010113          	addi	sp,sp,16
   13a64:	f45ff06f          	j	139a8 <close_file>
   13a68:	00040513          	mv	a0,s0
   13a6c:	f3dff0ef          	jal	ra,139a8 <close_file>
   13a70:	07043403          	ld	s0,112(s0)
   13a74:	fcdff06f          	j	13a40 <__stdio_exit+0x14>

0000000000013a78 <__stdio_write>:
   13a78:	fb010113          	addi	sp,sp,-80
   13a7c:	03853783          	ld	a5,56(a0)
   13a80:	03313423          	sd	s3,40(sp)
   13a84:	02853983          	ld	s3,40(a0)
   13a88:	04813023          	sd	s0,64(sp)
   13a8c:	02913c23          	sd	s1,56(sp)
   13a90:	40f989b3          	sub	s3,s3,a5
   13a94:	03213823          	sd	s2,48(sp)
   13a98:	03413023          	sd	s4,32(sp)
   13a9c:	01313423          	sd	s3,8(sp)
   13aa0:	04113423          	sd	ra,72(sp)
   13aa4:	00050413          	mv	s0,a0
   13aa8:	00060913          	mv	s2,a2
   13aac:	00f13023          	sd	a5,0(sp)
   13ab0:	00b13823          	sd	a1,16(sp)
   13ab4:	00c13c23          	sd	a2,24(sp)
   13ab8:	00c989b3          	add	s3,s3,a2
   13abc:	00200a13          	li	s4,2
   13ac0:	00010493          	mv	s1,sp
   13ac4:	07842503          	lw	a0,120(s0)
   13ac8:	04200893          	li	a7,66
   13acc:	00048593          	mv	a1,s1
   13ad0:	000a0613          	mv	a2,s4
   13ad4:	00000073          	ecall
   13ad8:	f09fe0ef          	jal	ra,129e0 <__syscall_ret>
   13adc:	04a99063          	bne	s3,a0,13b1c <__stdio_write+0xa4>
   13ae0:	05843783          	ld	a5,88(s0)
   13ae4:	06043703          	ld	a4,96(s0)
   13ae8:	02f43c23          	sd	a5,56(s0)
   13aec:	00e78733          	add	a4,a5,a4
   13af0:	02e43023          	sd	a4,32(s0)
   13af4:	02f43423          	sd	a5,40(s0)
   13af8:	04813083          	ld	ra,72(sp)
   13afc:	04013403          	ld	s0,64(sp)
   13b00:	03813483          	ld	s1,56(sp)
   13b04:	02813983          	ld	s3,40(sp)
   13b08:	02013a03          	ld	s4,32(sp)
   13b0c:	00090513          	mv	a0,s2
   13b10:	03013903          	ld	s2,48(sp)
   13b14:	05010113          	addi	sp,sp,80
   13b18:	00008067          	ret
   13b1c:	02055863          	bgez	a0,13b4c <__stdio_write+0xd4>
   13b20:	00042783          	lw	a5,0(s0)
   13b24:	02043023          	sd	zero,32(s0)
   13b28:	02043c23          	sd	zero,56(s0)
   13b2c:	0207e793          	ori	a5,a5,32
   13b30:	00f42023          	sw	a5,0(s0)
   13b34:	02043423          	sd	zero,40(s0)
   13b38:	00200793          	li	a5,2
   13b3c:	04fa0263          	beq	s4,a5,13b80 <__stdio_write+0x108>
   13b40:	0084b783          	ld	a5,8(s1)
   13b44:	40f90933          	sub	s2,s2,a5
   13b48:	fb1ff06f          	j	13af8 <__stdio_write+0x80>
   13b4c:	0084b783          	ld	a5,8(s1)
   13b50:	40a989b3          	sub	s3,s3,a0
   13b54:	00a7f863          	bgeu	a5,a0,13b64 <__stdio_write+0xec>
   13b58:	40f50533          	sub	a0,a0,a5
   13b5c:	01048493          	addi	s1,s1,16
   13b60:	fffa0a1b          	addiw	s4,s4,-1
   13b64:	0004b783          	ld	a5,0(s1)
   13b68:	00a787b3          	add	a5,a5,a0
   13b6c:	00f4b023          	sd	a5,0(s1)
   13b70:	0084b783          	ld	a5,8(s1)
   13b74:	40a78533          	sub	a0,a5,a0
   13b78:	00a4b423          	sd	a0,8(s1)
   13b7c:	f49ff06f          	j	13ac4 <__stdio_write+0x4c>
   13b80:	00000913          	li	s2,0
   13b84:	f75ff06f          	j	13af8 <__stdio_write+0x80>

0000000000013b88 <__ofl_lock>:
   13b88:	ff010113          	addi	sp,sp,-16
   13b8c:	9b818513          	addi	a0,gp,-1608 # 181b8 <ofl_lock>
   13b90:	00113423          	sd	ra,8(sp)
   13b94:	b05ff0ef          	jal	ra,13698 <__lock>
   13b98:	00813083          	ld	ra,8(sp)
   13b9c:	9b018513          	addi	a0,gp,-1616 # 181b0 <ofl_head>
   13ba0:	01010113          	addi	sp,sp,16
   13ba4:	00008067          	ret

0000000000013ba8 <__ofl_unlock>:
   13ba8:	9b818513          	addi	a0,gp,-1608 # 181b8 <ofl_lock>
   13bac:	be5ff06f          	j	13790 <__unlock>

0000000000013bb0 <__addtf3>:
   13bb0:	fd010113          	addi	sp,sp,-48
   13bb4:	02113423          	sd	ra,40(sp)
   13bb8:	02813023          	sd	s0,32(sp)
   13bbc:	00913c23          	sd	s1,24(sp)
   13bc0:	01213823          	sd	s2,16(sp)
   13bc4:	01313423          	sd	s3,8(sp)
   13bc8:	01413023          	sd	s4,0(sp)
   13bcc:	002024f3          	frrm	s1
   13bd0:	fff00e93          	li	t4,-1
   13bd4:	010ed713          	srli	a4,t4,0x10
   13bd8:	0305d413          	srli	s0,a1,0x30
   13bdc:	03f5d913          	srli	s2,a1,0x3f
   13be0:	00e5f5b3          	and	a1,a1,a4
   13be4:	00359593          	slli	a1,a1,0x3
   13be8:	03d55793          	srli	a5,a0,0x3d
   13bec:	00008337          	lui	t1,0x8
   13bf0:	fff30e13          	addi	t3,t1,-1 # 7fff <exit-0x8121>
   13bf4:	00b7e7b3          	or	a5,a5,a1
   13bf8:	03f6d893          	srli	a7,a3,0x3f
   13bfc:	0306d593          	srli	a1,a3,0x30
   13c00:	00e6f6b3          	and	a3,a3,a4
   13c04:	00369693          	slli	a3,a3,0x3
   13c08:	03d65713          	srli	a4,a2,0x3d
   13c0c:	01c47433          	and	s0,s0,t3
   13c10:	01c5f5b3          	and	a1,a1,t3
   13c14:	00d76733          	or	a4,a4,a3
   13c18:	40b406bb          	subw	a3,s0,a1
   13c1c:	0004849b          	sext.w	s1,s1
   13c20:	00351513          	slli	a0,a0,0x3
   13c24:	00361613          	slli	a2,a2,0x3
   13c28:	0006881b          	sext.w	a6,a3
   13c2c:	4d191a63          	bne	s2,a7,14100 <__addtf3+0x550>
   13c30:	13005e63          	blez	a6,13d6c <__addtf3+0x1bc>
   13c34:	0a059863          	bnez	a1,13ce4 <__addtf3+0x134>
   13c38:	00c765b3          	or	a1,a4,a2
   13c3c:	02059263          	bnez	a1,13c60 <__addtf3+0xb0>
   13c40:	4dc41a63          	bne	s0,t3,14114 <__addtf3+0x564>
   13c44:	00a7e733          	or	a4,a5,a0
   13c48:	18070ee3          	beqz	a4,145e4 <__addtf3+0xa34>
   13c4c:	0327d713          	srli	a4,a5,0x32
   13c50:	00040593          	mv	a1,s0
   13c54:	00000813          	li	a6,0
   13c58:	3e071a63          	bnez	a4,1404c <__addtf3+0x49c>
   13c5c:	1440006f          	j	13da0 <__addtf3+0x1f0>
   13c60:	fff6881b          	addiw	a6,a3,-1
   13c64:	06081463          	bnez	a6,13ccc <__addtf3+0x11c>
   13c68:	00a60633          	add	a2,a2,a0
   13c6c:	00a63533          	sltu	a0,a2,a0
   13c70:	00f70733          	add	a4,a4,a5
   13c74:	00a70733          	add	a4,a4,a0
   13c78:	00040593          	mv	a1,s0
   13c7c:	00060513          	mv	a0,a2
   13c80:	03375793          	srli	a5,a4,0x33
   13c84:	0017f793          	andi	a5,a5,1
   13c88:	0c078ee3          	beqz	a5,14564 <__addtf3+0x9b4>
   13c8c:	000086b7          	lui	a3,0x8
   13c90:	00158593          	addi	a1,a1,1 # ffffffff80000001 <__BSS_END__+0xffffffff7ffe77f9>
   13c94:	fff68793          	addi	a5,a3,-1 # 7fff <exit-0x8121>
   13c98:	42f58463          	beq	a1,a5,140c0 <__addtf3+0x510>
   13c9c:	fff00793          	li	a5,-1
   13ca0:	03379793          	slli	a5,a5,0x33
   13ca4:	fff78793          	addi	a5,a5,-1
   13ca8:	00f777b3          	and	a5,a4,a5
   13cac:	00155713          	srli	a4,a0,0x1
   13cb0:	00157513          	andi	a0,a0,1
   13cb4:	00a76533          	or	a0,a4,a0
   13cb8:	03f79713          	slli	a4,a5,0x3f
   13cbc:	00a76533          	or	a0,a4,a0
   13cc0:	0017d793          	srli	a5,a5,0x1
   13cc4:	00000813          	li	a6,0
   13cc8:	3840006f          	j	1404c <__addtf3+0x49c>
   13ccc:	03c41463          	bne	s0,t3,13cf4 <__addtf3+0x144>
   13cd0:	00a7e733          	or	a4,a5,a0
   13cd4:	100708e3          	beqz	a4,145e4 <__addtf3+0xa34>
   13cd8:	0327d713          	srli	a4,a5,0x32
   13cdc:	00177713          	andi	a4,a4,1
   13ce0:	f71ff06f          	j	13c50 <__addtf3+0xa0>
   13ce4:	ffc406e3          	beq	s0,t3,13cd0 <__addtf3+0x120>
   13ce8:	00100693          	li	a3,1
   13cec:	03369693          	slli	a3,a3,0x33
   13cf0:	00d76733          	or	a4,a4,a3
   13cf4:	07400693          	li	a3,116
   13cf8:	0706c463          	blt	a3,a6,13d60 <__addtf3+0x1b0>
   13cfc:	03f00693          	li	a3,63
   13d00:	0306c663          	blt	a3,a6,13d2c <__addtf3+0x17c>
   13d04:	04000593          	li	a1,64
   13d08:	410585bb          	subw	a1,a1,a6
   13d0c:	00b716b3          	sll	a3,a4,a1
   13d10:	010658b3          	srl	a7,a2,a6
   13d14:	00b61633          	sll	a2,a2,a1
   13d18:	0116e6b3          	or	a3,a3,a7
   13d1c:	00c03633          	snez	a2,a2
   13d20:	00c6e633          	or	a2,a3,a2
   13d24:	01075733          	srl	a4,a4,a6
   13d28:	f41ff06f          	j	13c68 <__addtf3+0xb8>
   13d2c:	fc08069b          	addiw	a3,a6,-64
   13d30:	04000893          	li	a7,64
   13d34:	00d756b3          	srl	a3,a4,a3
   13d38:	00000593          	li	a1,0
   13d3c:	01180863          	beq	a6,a7,13d4c <__addtf3+0x19c>
   13d40:	08000593          	li	a1,128
   13d44:	410585bb          	subw	a1,a1,a6
   13d48:	00b715b3          	sll	a1,a4,a1
   13d4c:	00c5e633          	or	a2,a1,a2
   13d50:	00c03633          	snez	a2,a2
   13d54:	00c6e633          	or	a2,a3,a2
   13d58:	00000713          	li	a4,0
   13d5c:	f0dff06f          	j	13c68 <__addtf3+0xb8>
   13d60:	00c76633          	or	a2,a4,a2
   13d64:	00c03633          	snez	a2,a2
   13d68:	ff1ff06f          	j	13d58 <__addtf3+0x1a8>
   13d6c:	12080063          	beqz	a6,13e8c <__addtf3+0x2dc>
   13d70:	08041063          	bnez	s0,13df0 <__addtf3+0x240>
   13d74:	00a7e833          	or	a6,a5,a0
   13d78:	02081863          	bnez	a6,13da8 <__addtf3+0x1f8>
   13d7c:	00060513          	mv	a0,a2
   13d80:	7fc59263          	bne	a1,t3,14564 <__addtf3+0x9b4>
   13d84:	00c76533          	or	a0,a4,a2
   13d88:	040502e3          	beqz	a0,145cc <__addtf3+0xa1c>
   13d8c:	03275793          	srli	a5,a4,0x32
   13d90:	0017f793          	andi	a5,a5,1
   13d94:	040792e3          	bnez	a5,145d8 <__addtf3+0xa28>
   13d98:	00070793          	mv	a5,a4
   13d9c:	00060513          	mv	a0,a2
   13da0:	01000693          	li	a3,16
   13da4:	1400006f          	j	13ee4 <__addtf3+0x334>
   13da8:	fff6c693          	not	a3,a3
   13dac:	0006869b          	sext.w	a3,a3
   13db0:	00069c63          	bnez	a3,13dc8 <__addtf3+0x218>
   13db4:	00c50533          	add	a0,a0,a2
   13db8:	00e78733          	add	a4,a5,a4
   13dbc:	00c53633          	sltu	a2,a0,a2
   13dc0:	00c70733          	add	a4,a4,a2
   13dc4:	ebdff06f          	j	13c80 <__addtf3+0xd0>
   13dc8:	03c59e63          	bne	a1,t3,13e04 <__addtf3+0x254>
   13dcc:	00c76533          	or	a0,a4,a2
   13dd0:	7e050e63          	beqz	a0,145cc <__addtf3+0xa1c>
   13dd4:	03275793          	srli	a5,a4,0x32
   13dd8:	0017f793          	andi	a5,a5,1
   13ddc:	7e079e63          	bnez	a5,145d8 <__addtf3+0xa28>
   13de0:	00070793          	mv	a5,a4
   13de4:	00060513          	mv	a0,a2
   13de8:	00000813          	li	a6,0
   13dec:	fb5ff06f          	j	13da0 <__addtf3+0x1f0>
   13df0:	fdc58ee3          	beq	a1,t3,13dcc <__addtf3+0x21c>
   13df4:	00100813          	li	a6,1
   13df8:	03381813          	slli	a6,a6,0x33
   13dfc:	40d006bb          	negw	a3,a3
   13e00:	0107e7b3          	or	a5,a5,a6
   13e04:	07400813          	li	a6,116
   13e08:	06d84c63          	blt	a6,a3,13e80 <__addtf3+0x2d0>
   13e0c:	03f00813          	li	a6,63
   13e10:	02d84e63          	blt	a6,a3,13e4c <__addtf3+0x29c>
   13e14:	04000893          	li	a7,64
   13e18:	40d888bb          	subw	a7,a7,a3
   13e1c:	00d55333          	srl	t1,a0,a3
   13e20:	01179833          	sll	a6,a5,a7
   13e24:	01151533          	sll	a0,a0,a7
   13e28:	00686833          	or	a6,a6,t1
   13e2c:	00a03533          	snez	a0,a0
   13e30:	00a86533          	or	a0,a6,a0
   13e34:	00d7d6b3          	srl	a3,a5,a3
   13e38:	00c50533          	add	a0,a0,a2
   13e3c:	00e686b3          	add	a3,a3,a4
   13e40:	00c53633          	sltu	a2,a0,a2
   13e44:	00c68733          	add	a4,a3,a2
   13e48:	e39ff06f          	j	13c80 <__addtf3+0xd0>
   13e4c:	fc06881b          	addiw	a6,a3,-64
   13e50:	04000313          	li	t1,64
   13e54:	0107d833          	srl	a6,a5,a6
   13e58:	00000893          	li	a7,0
   13e5c:	00668863          	beq	a3,t1,13e6c <__addtf3+0x2bc>
   13e60:	08000893          	li	a7,128
   13e64:	40d886bb          	subw	a3,a7,a3
   13e68:	00d798b3          	sll	a7,a5,a3
   13e6c:	00a8e533          	or	a0,a7,a0
   13e70:	00a03533          	snez	a0,a0
   13e74:	00a86533          	or	a0,a6,a0
   13e78:	00000693          	li	a3,0
   13e7c:	fbdff06f          	j	13e38 <__addtf3+0x288>
   13e80:	00a7e533          	or	a0,a5,a0
   13e84:	00a03533          	snez	a0,a0
   13e88:	ff1ff06f          	j	13e78 <__addtf3+0x2c8>
   13e8c:	00140893          	addi	a7,s0,1
   13e90:	ffe30693          	addi	a3,t1,-2
   13e94:	00d8f333          	and	t1,a7,a3
   13e98:	18031663          	bnez	t1,14024 <__addtf3+0x474>
   13e9c:	00a7e8b3          	or	a7,a5,a0
   13ea0:	06041063          	bnez	s0,13f00 <__addtf3+0x350>
   13ea4:	6c088c63          	beqz	a7,1457c <__addtf3+0x9cc>
   13ea8:	00c766b3          	or	a3,a4,a2
   13eac:	48068c63          	beqz	a3,14344 <__addtf3+0x794>
   13eb0:	00c50633          	add	a2,a0,a2
   13eb4:	00e787b3          	add	a5,a5,a4
   13eb8:	00a63533          	sltu	a0,a2,a0
   13ebc:	00a787b3          	add	a5,a5,a0
   13ec0:	0337d713          	srli	a4,a5,0x33
   13ec4:	00177713          	andi	a4,a4,1
   13ec8:	6a070c63          	beqz	a4,14580 <__addtf3+0x9d0>
   13ecc:	033e9713          	slli	a4,t4,0x33
   13ed0:	fff70713          	addi	a4,a4,-1
   13ed4:	00e7f7b3          	and	a5,a5,a4
   13ed8:	00060513          	mv	a0,a2
   13edc:	00000693          	li	a3,0
   13ee0:	00100593          	li	a1,1
   13ee4:	00757713          	andi	a4,a0,7
   13ee8:	1a071063          	bnez	a4,14088 <__addtf3+0x4d8>
   13eec:	08080663          	beqz	a6,13f78 <__addtf3+0x3c8>
   13ef0:	0016f713          	andi	a4,a3,1
   13ef4:	08070263          	beqz	a4,13f78 <__addtf3+0x3c8>
   13ef8:	0026e693          	ori	a3,a3,2
   13efc:	07c0006f          	j	13f78 <__addtf3+0x3c8>
   13f00:	03c41c63          	bne	s0,t3,13f38 <__addtf3+0x388>
   13f04:	78088c63          	beqz	a7,1469c <__addtf3+0xaec>
   13f08:	0327d693          	srli	a3,a5,0x32
   13f0c:	0016f693          	andi	a3,a3,1
   13f10:	0016b693          	seqz	a3,a3
   13f14:	00469693          	slli	a3,a3,0x4
   13f18:	04859063          	bne	a1,s0,13f58 <__addtf3+0x3a8>
   13f1c:	00c765b3          	or	a1,a4,a2
   13f20:	02058063          	beqz	a1,13f40 <__addtf3+0x390>
   13f24:	03275593          	srli	a1,a4,0x32
   13f28:	0015f593          	andi	a1,a1,1
   13f2c:	00059a63          	bnez	a1,13f40 <__addtf3+0x390>
   13f30:	01000693          	li	a3,16
   13f34:	00c0006f          	j	13f40 <__addtf3+0x390>
   13f38:	00000693          	li	a3,0
   13f3c:	ffc580e3          	beq	a1,t3,13f1c <__addtf3+0x36c>
   13f40:	00089c63          	bnez	a7,13f58 <__addtf3+0x3a8>
   13f44:	00070793          	mv	a5,a4
   13f48:	00060513          	mv	a0,a2
   13f4c:	000085b7          	lui	a1,0x8
   13f50:	fff58593          	addi	a1,a1,-1 # 7fff <exit-0x8121>
   13f54:	f91ff06f          	j	13ee4 <__addtf3+0x334>
   13f58:	00c76633          	or	a2,a4,a2
   13f5c:	fe0608e3          	beqz	a2,13f4c <__addtf3+0x39c>
   13f60:	00100793          	li	a5,1
   13f64:	000085b7          	lui	a1,0x8
   13f68:	00000913          	li	s2,0
   13f6c:	03279793          	slli	a5,a5,0x32
   13f70:	00000513          	li	a0,0
   13f74:	fff58593          	addi	a1,a1,-1 # 7fff <exit-0x8121>
   13f78:	0337d713          	srli	a4,a5,0x33
   13f7c:	00177713          	andi	a4,a4,1
   13f80:	02070263          	beqz	a4,13fa4 <__addtf3+0x3f4>
   13f84:	00008737          	lui	a4,0x8
   13f88:	00158593          	addi	a1,a1,1
   13f8c:	fff70613          	addi	a2,a4,-1 # 7fff <exit-0x8121>
   13f90:	6cc58463          	beq	a1,a2,14658 <__addtf3+0xaa8>
   13f94:	fff00713          	li	a4,-1
   13f98:	03371713          	slli	a4,a4,0x33
   13f9c:	fff70713          	addi	a4,a4,-1
   13fa0:	00e7f7b3          	and	a5,a5,a4
   13fa4:	00355713          	srli	a4,a0,0x3
   13fa8:	03d79513          	slli	a0,a5,0x3d
   13fac:	00e56533          	or	a0,a0,a4
   13fb0:	00008737          	lui	a4,0x8
   13fb4:	fff70713          	addi	a4,a4,-1 # 7fff <exit-0x8121>
   13fb8:	0037d793          	srli	a5,a5,0x3
   13fbc:	02e59063          	bne	a1,a4,13fdc <__addtf3+0x42c>
   13fc0:	00f56533          	or	a0,a0,a5
   13fc4:	00000793          	li	a5,0
   13fc8:	00050a63          	beqz	a0,13fdc <__addtf3+0x42c>
   13fcc:	00100793          	li	a5,1
   13fd0:	02f79793          	slli	a5,a5,0x2f
   13fd4:	00000513          	li	a0,0
   13fd8:	00000913          	li	s2,0
   13fdc:	03159593          	slli	a1,a1,0x31
   13fe0:	00f91913          	slli	s2,s2,0xf
   13fe4:	0315d593          	srli	a1,a1,0x31
   13fe8:	00b965b3          	or	a1,s2,a1
   13fec:	01079793          	slli	a5,a5,0x10
   13ff0:	03059913          	slli	s2,a1,0x30
   13ff4:	0107d593          	srli	a1,a5,0x10
   13ff8:	0125e5b3          	or	a1,a1,s2
   13ffc:	00068463          	beqz	a3,14004 <__addtf3+0x454>
   14000:	0016a073          	csrs	fflags,a3
   14004:	02813083          	ld	ra,40(sp)
   14008:	02013403          	ld	s0,32(sp)
   1400c:	01813483          	ld	s1,24(sp)
   14010:	01013903          	ld	s2,16(sp)
   14014:	00813983          	ld	s3,8(sp)
   14018:	00013a03          	ld	s4,0(sp)
   1401c:	03010113          	addi	sp,sp,48
   14020:	00008067          	ret
   14024:	03c88863          	beq	a7,t3,14054 <__addtf3+0x4a4>
   14028:	00c50633          	add	a2,a0,a2
   1402c:	00a63533          	sltu	a0,a2,a0
   14030:	00e787b3          	add	a5,a5,a4
   14034:	00a787b3          	add	a5,a5,a0
   14038:	03f79513          	slli	a0,a5,0x3f
   1403c:	00165613          	srli	a2,a2,0x1
   14040:	00c56533          	or	a0,a0,a2
   14044:	0017d793          	srli	a5,a5,0x1
   14048:	00088593          	mv	a1,a7
   1404c:	00000693          	li	a3,0
   14050:	e95ff06f          	j	13ee4 <__addtf3+0x334>
   14054:	00048863          	beqz	s1,14064 <__addtf3+0x4b4>
   14058:	00300793          	li	a5,3
   1405c:	00f49863          	bne	s1,a5,1406c <__addtf3+0x4bc>
   14060:	00091c63          	bnez	s2,14078 <__addtf3+0x4c8>
   14064:	00088593          	mv	a1,a7
   14068:	0680006f          	j	140d0 <__addtf3+0x520>
   1406c:	00200793          	li	a5,2
   14070:	00f49463          	bne	s1,a5,14078 <__addtf3+0x4c8>
   14074:	fe0918e3          	bnez	s2,14064 <__addtf3+0x4b4>
   14078:	fff00793          	li	a5,-1
   1407c:	fff00513          	li	a0,-1
   14080:	00068593          	mv	a1,a3
   14084:	00500693          	li	a3,5
   14088:	00200713          	li	a4,2
   1408c:	0016e693          	ori	a3,a3,1
   14090:	5ae48e63          	beq	s1,a4,1464c <__addtf3+0xa9c>
   14094:	00300713          	li	a4,3
   14098:	5ae48463          	beq	s1,a4,14640 <__addtf3+0xa90>
   1409c:	5a049a63          	bnez	s1,14650 <__addtf3+0xaa0>
   140a0:	00f57713          	andi	a4,a0,15
   140a4:	00400613          	li	a2,4
   140a8:	5ac70463          	beq	a4,a2,14650 <__addtf3+0xaa0>
   140ac:	00450713          	addi	a4,a0,4
   140b0:	00a73533          	sltu	a0,a4,a0
   140b4:	00a787b3          	add	a5,a5,a0
   140b8:	00070513          	mv	a0,a4
   140bc:	5940006f          	j	14650 <__addtf3+0xaa0>
   140c0:	00048863          	beqz	s1,140d0 <__addtf3+0x520>
   140c4:	00300793          	li	a5,3
   140c8:	00f49c63          	bne	s1,a5,140e0 <__addtf3+0x530>
   140cc:	02091063          	bnez	s2,140ec <__addtf3+0x53c>
   140d0:	00000793          	li	a5,0
   140d4:	00000513          	li	a0,0
   140d8:	00500693          	li	a3,5
   140dc:	e9dff06f          	j	13f78 <__addtf3+0x3c8>
   140e0:	00200793          	li	a5,2
   140e4:	00f49463          	bne	s1,a5,140ec <__addtf3+0x53c>
   140e8:	fe0914e3          	bnez	s2,140d0 <__addtf3+0x520>
   140ec:	fff00793          	li	a5,-1
   140f0:	fff00513          	li	a0,-1
   140f4:	ffe68593          	addi	a1,a3,-2
   140f8:	00000813          	li	a6,0
   140fc:	f89ff06f          	j	14084 <__addtf3+0x4d4>
   14100:	0f005863          	blez	a6,141f0 <__addtf3+0x640>
   14104:	08059c63          	bnez	a1,1419c <__addtf3+0x5ec>
   14108:	00c765b3          	or	a1,a4,a2
   1410c:	00059a63          	bnez	a1,14120 <__addtf3+0x570>
   14110:	bdc400e3          	beq	s0,t3,13cd0 <__addtf3+0x120>
   14114:	00078713          	mv	a4,a5
   14118:	00040593          	mv	a1,s0
   1411c:	4480006f          	j	14564 <__addtf3+0x9b4>
   14120:	fff6881b          	addiw	a6,a3,-1
   14124:	02081e63          	bnez	a6,14160 <__addtf3+0x5b0>
   14128:	40c50633          	sub	a2,a0,a2
   1412c:	00c53533          	sltu	a0,a0,a2
   14130:	40e78733          	sub	a4,a5,a4
   14134:	40a70733          	sub	a4,a4,a0
   14138:	00040593          	mv	a1,s0
   1413c:	00060513          	mv	a0,a2
   14140:	03375793          	srli	a5,a4,0x33
   14144:	0017f793          	andi	a5,a5,1
   14148:	40078e63          	beqz	a5,14564 <__addtf3+0x9b4>
   1414c:	00d71713          	slli	a4,a4,0xd
   14150:	00d75993          	srli	s3,a4,0xd
   14154:	00050a13          	mv	s4,a0
   14158:	00058413          	mv	s0,a1
   1415c:	3240006f          	j	14480 <__addtf3+0x8d0>
   14160:	b7c408e3          	beq	s0,t3,13cd0 <__addtf3+0x120>
   14164:	07400693          	li	a3,116
   14168:	0706ce63          	blt	a3,a6,141e4 <__addtf3+0x634>
   1416c:	03f00693          	li	a3,63
   14170:	0506c063          	blt	a3,a6,141b0 <__addtf3+0x600>
   14174:	04000593          	li	a1,64
   14178:	410585bb          	subw	a1,a1,a6
   1417c:	00b716b3          	sll	a3,a4,a1
   14180:	010658b3          	srl	a7,a2,a6
   14184:	00b61633          	sll	a2,a2,a1
   14188:	0116e6b3          	or	a3,a3,a7
   1418c:	00c03633          	snez	a2,a2
   14190:	00c6e633          	or	a2,a3,a2
   14194:	01075733          	srl	a4,a4,a6
   14198:	f91ff06f          	j	14128 <__addtf3+0x578>
   1419c:	b3c40ae3          	beq	s0,t3,13cd0 <__addtf3+0x120>
   141a0:	00100693          	li	a3,1
   141a4:	03369693          	slli	a3,a3,0x33
   141a8:	00d76733          	or	a4,a4,a3
   141ac:	fb9ff06f          	j	14164 <__addtf3+0x5b4>
   141b0:	fc08069b          	addiw	a3,a6,-64
   141b4:	04000893          	li	a7,64
   141b8:	00d756b3          	srl	a3,a4,a3
   141bc:	00000593          	li	a1,0
   141c0:	01180863          	beq	a6,a7,141d0 <__addtf3+0x620>
   141c4:	08000593          	li	a1,128
   141c8:	410585bb          	subw	a1,a1,a6
   141cc:	00b715b3          	sll	a1,a4,a1
   141d0:	00c5e633          	or	a2,a1,a2
   141d4:	00c03633          	snez	a2,a2
   141d8:	00c6e633          	or	a2,a3,a2
   141dc:	00000713          	li	a4,0
   141e0:	f49ff06f          	j	14128 <__addtf3+0x578>
   141e4:	00c76633          	or	a2,a4,a2
   141e8:	00c03633          	snez	a2,a2
   141ec:	ff1ff06f          	j	141dc <__addtf3+0x62c>
   141f0:	12080063          	beqz	a6,14310 <__addtf3+0x760>
   141f4:	08041063          	bnez	s0,14274 <__addtf3+0x6c4>
   141f8:	00a7e833          	or	a6,a5,a0
   141fc:	02081663          	bnez	a6,14228 <__addtf3+0x678>
   14200:	37c59863          	bne	a1,t3,14570 <__addtf3+0x9c0>
   14204:	00c76533          	or	a0,a4,a2
   14208:	3e050663          	beqz	a0,145f4 <__addtf3+0xa44>
   1420c:	03275793          	srli	a5,a4,0x32
   14210:	0017f793          	andi	a5,a5,1
   14214:	3e079663          	bnez	a5,14600 <__addtf3+0xa50>
   14218:	00070793          	mv	a5,a4
   1421c:	00060513          	mv	a0,a2
   14220:	00088913          	mv	s2,a7
   14224:	b7dff06f          	j	13da0 <__addtf3+0x1f0>
   14228:	fff6c693          	not	a3,a3
   1422c:	0006869b          	sext.w	a3,a3
   14230:	00069e63          	bnez	a3,1424c <__addtf3+0x69c>
   14234:	40a60533          	sub	a0,a2,a0
   14238:	40f70733          	sub	a4,a4,a5
   1423c:	00a63633          	sltu	a2,a2,a0
   14240:	40c70733          	sub	a4,a4,a2
   14244:	00088913          	mv	s2,a7
   14248:	ef9ff06f          	j	14140 <__addtf3+0x590>
   1424c:	03c59e63          	bne	a1,t3,14288 <__addtf3+0x6d8>
   14250:	00c76533          	or	a0,a4,a2
   14254:	3a050063          	beqz	a0,145f4 <__addtf3+0xa44>
   14258:	03275793          	srli	a5,a4,0x32
   1425c:	0017f793          	andi	a5,a5,1
   14260:	3a079063          	bnez	a5,14600 <__addtf3+0xa50>
   14264:	00070793          	mv	a5,a4
   14268:	00060513          	mv	a0,a2
   1426c:	00088913          	mv	s2,a7
   14270:	b79ff06f          	j	13de8 <__addtf3+0x238>
   14274:	fdc58ee3          	beq	a1,t3,14250 <__addtf3+0x6a0>
   14278:	00100813          	li	a6,1
   1427c:	03381813          	slli	a6,a6,0x33
   14280:	40d006bb          	negw	a3,a3
   14284:	0107e7b3          	or	a5,a5,a6
   14288:	07400813          	li	a6,116
   1428c:	06d84c63          	blt	a6,a3,14304 <__addtf3+0x754>
   14290:	03f00813          	li	a6,63
   14294:	02d84e63          	blt	a6,a3,142d0 <__addtf3+0x720>
   14298:	04000313          	li	t1,64
   1429c:	40d3033b          	subw	t1,t1,a3
   142a0:	00679833          	sll	a6,a5,t1
   142a4:	00d55e33          	srl	t3,a0,a3
   142a8:	00651533          	sll	a0,a0,t1
   142ac:	01c86833          	or	a6,a6,t3
   142b0:	00a03533          	snez	a0,a0
   142b4:	00a86533          	or	a0,a6,a0
   142b8:	00d7d7b3          	srl	a5,a5,a3
   142bc:	40a60533          	sub	a0,a2,a0
   142c0:	40f707b3          	sub	a5,a4,a5
   142c4:	00a63633          	sltu	a2,a2,a0
   142c8:	40c78733          	sub	a4,a5,a2
   142cc:	f79ff06f          	j	14244 <__addtf3+0x694>
   142d0:	fc06881b          	addiw	a6,a3,-64
   142d4:	04000e13          	li	t3,64
   142d8:	0107d833          	srl	a6,a5,a6
   142dc:	00000313          	li	t1,0
   142e0:	01c68863          	beq	a3,t3,142f0 <__addtf3+0x740>
   142e4:	08000313          	li	t1,128
   142e8:	40d306bb          	subw	a3,t1,a3
   142ec:	00d79333          	sll	t1,a5,a3
   142f0:	00a36533          	or	a0,t1,a0
   142f4:	00a03533          	snez	a0,a0
   142f8:	00a86533          	or	a0,a6,a0
   142fc:	00000793          	li	a5,0
   14300:	fbdff06f          	j	142bc <__addtf3+0x70c>
   14304:	00a7e533          	or	a0,a5,a0
   14308:	00a03533          	snez	a0,a0
   1430c:	ff1ff06f          	j	142fc <__addtf3+0x74c>
   14310:	00140e93          	addi	t4,s0,1
   14314:	ffe30693          	addi	a3,t1,-2
   14318:	00def6b3          	and	a3,t4,a3
   1431c:	12069a63          	bnez	a3,14450 <__addtf3+0x8a0>
   14320:	00a7eeb3          	or	t4,a5,a0
   14324:	00c76333          	or	t1,a4,a2
   14328:	0c041663          	bnez	s0,143f4 <__addtf3+0x844>
   1432c:	060e9c63          	bnez	t4,143a4 <__addtf3+0x7f4>
   14330:	24031c63          	bnez	t1,14588 <__addtf3+0x9d8>
   14334:	ffe48913          	addi	s2,s1,-2
   14338:	00193913          	seqz	s2,s2
   1433c:	00000793          	li	a5,0
   14340:	00000513          	li	a0,0
   14344:	00f56733          	or	a4,a0,a5
   14348:	2e070463          	beqz	a4,14630 <__addtf3+0xa80>
   1434c:	03f55713          	srli	a4,a0,0x3f
   14350:	00179813          	slli	a6,a5,0x1
   14354:	00e80833          	add	a6,a6,a4
   14358:	00151713          	slli	a4,a0,0x1
   1435c:	00777613          	andi	a2,a4,7
   14360:	00000693          	li	a3,0
   14364:	02060663          	beqz	a2,14390 <__addtf3+0x7e0>
   14368:	00200693          	li	a3,2
   1436c:	24d48a63          	beq	s1,a3,145c0 <__addtf3+0xa10>
   14370:	00300693          	li	a3,3
   14374:	22d48663          	beq	s1,a3,145a0 <__addtf3+0x9f0>
   14378:	00100693          	li	a3,1
   1437c:	00049a63          	bnez	s1,14390 <__addtf3+0x7e0>
   14380:	00f77613          	andi	a2,a4,15
   14384:	00400593          	li	a1,4
   14388:	ffc73713          	sltiu	a4,a4,-4
   1438c:	22b61063          	bne	a2,a1,145ac <__addtf3+0x9fc>
   14390:	03485813          	srli	a6,a6,0x34
   14394:	00184813          	xori	a6,a6,1
   14398:	00187813          	andi	a6,a6,1
   1439c:	00000593          	li	a1,0
   143a0:	b45ff06f          	j	13ee4 <__addtf3+0x334>
   143a4:	fa0300e3          	beqz	t1,14344 <__addtf3+0x794>
   143a8:	40c505b3          	sub	a1,a0,a2
   143ac:	00b53833          	sltu	a6,a0,a1
   143b0:	40e786b3          	sub	a3,a5,a4
   143b4:	410686b3          	sub	a3,a3,a6
   143b8:	0336d813          	srli	a6,a3,0x33
   143bc:	00187813          	andi	a6,a6,1
   143c0:	00080e63          	beqz	a6,143dc <__addtf3+0x82c>
   143c4:	40a60533          	sub	a0,a2,a0
   143c8:	40f707b3          	sub	a5,a4,a5
   143cc:	00a63633          	sltu	a2,a2,a0
   143d0:	40c787b3          	sub	a5,a5,a2
   143d4:	00088913          	mv	s2,a7
   143d8:	f6dff06f          	j	14344 <__addtf3+0x794>
   143dc:	00d5e533          	or	a0,a1,a3
   143e0:	1a051a63          	bnez	a0,14594 <__addtf3+0x9e4>
   143e4:	ffe48913          	addi	s2,s1,-2
   143e8:	00193913          	seqz	s2,s2
   143ec:	00000793          	li	a5,0
   143f0:	f55ff06f          	j	14344 <__addtf3+0x794>
   143f4:	03c41a63          	bne	s0,t3,14428 <__addtf3+0x878>
   143f8:	280e8c63          	beqz	t4,14690 <__addtf3+0xae0>
   143fc:	0327d693          	srli	a3,a5,0x32
   14400:	0016f693          	andi	a3,a3,1
   14404:	0016b693          	seqz	a3,a3
   14408:	00469693          	slli	a3,a3,0x4
   1440c:	02859e63          	bne	a1,s0,14448 <__addtf3+0x898>
   14410:	02030063          	beqz	t1,14430 <__addtf3+0x880>
   14414:	03275593          	srli	a1,a4,0x32
   14418:	0015f593          	andi	a1,a1,1
   1441c:	00059a63          	bnez	a1,14430 <__addtf3+0x880>
   14420:	01000693          	li	a3,16
   14424:	00c0006f          	j	14430 <__addtf3+0x880>
   14428:	00000693          	li	a3,0
   1442c:	ffc582e3          	beq	a1,t3,14410 <__addtf3+0x860>
   14430:	000e9c63          	bnez	t4,14448 <__addtf3+0x898>
   14434:	1c030e63          	beqz	t1,14610 <__addtf3+0xa60>
   14438:	00070793          	mv	a5,a4
   1443c:	00060513          	mv	a0,a2
   14440:	00088913          	mv	s2,a7
   14444:	b09ff06f          	j	13f4c <__addtf3+0x39c>
   14448:	b00302e3          	beqz	t1,13f4c <__addtf3+0x39c>
   1444c:	b15ff06f          	j	13f60 <__addtf3+0x3b0>
   14450:	40c50a33          	sub	s4,a0,a2
   14454:	014536b3          	sltu	a3,a0,s4
   14458:	40e789b3          	sub	s3,a5,a4
   1445c:	40d989b3          	sub	s3,s3,a3
   14460:	0339d693          	srli	a3,s3,0x33
   14464:	0016f693          	andi	a3,a3,1
   14468:	08068663          	beqz	a3,144f4 <__addtf3+0x944>
   1446c:	40a60a33          	sub	s4,a2,a0
   14470:	40f70733          	sub	a4,a4,a5
   14474:	01463633          	sltu	a2,a2,s4
   14478:	40c709b3          	sub	s3,a4,a2
   1447c:	00088913          	mv	s2,a7
   14480:	08098063          	beqz	s3,14500 <__addtf3+0x950>
   14484:	00098513          	mv	a0,s3
   14488:	1f9010ef          	jal	ra,15e80 <__clzdi2>
   1448c:	0005051b          	sext.w	a0,a0
   14490:	ff45059b          	addiw	a1,a0,-12
   14494:	03f00793          	li	a5,63
   14498:	00058693          	mv	a3,a1
   1449c:	06b7ca63          	blt	a5,a1,14510 <__addtf3+0x960>
   144a0:	04000713          	li	a4,64
   144a4:	40b7073b          	subw	a4,a4,a1
   144a8:	00b999b3          	sll	s3,s3,a1
   144ac:	00ea5733          	srl	a4,s4,a4
   144b0:	01376733          	or	a4,a4,s3
   144b4:	00ba1533          	sll	a0,s4,a1
   144b8:	0885cc63          	blt	a1,s0,14550 <__addtf3+0x9a0>
   144bc:	4086843b          	subw	s0,a3,s0
   144c0:	0014079b          	addiw	a5,s0,1
   144c4:	03f00693          	li	a3,63
   144c8:	04f6cc63          	blt	a3,a5,14520 <__addtf3+0x970>
   144cc:	04000693          	li	a3,64
   144d0:	40f686bb          	subw	a3,a3,a5
   144d4:	00f555b3          	srl	a1,a0,a5
   144d8:	00d71633          	sll	a2,a4,a3
   144dc:	00d51533          	sll	a0,a0,a3
   144e0:	00b66633          	or	a2,a2,a1
   144e4:	00a03533          	snez	a0,a0
   144e8:	00a66533          	or	a0,a2,a0
   144ec:	00f757b3          	srl	a5,a4,a5
   144f0:	e55ff06f          	j	14344 <__addtf3+0x794>
   144f4:	013a6533          	or	a0,s4,s3
   144f8:	f80514e3          	bnez	a0,14480 <__addtf3+0x8d0>
   144fc:	ee9ff06f          	j	143e4 <__addtf3+0x834>
   14500:	000a0513          	mv	a0,s4
   14504:	17d010ef          	jal	ra,15e80 <__clzdi2>
   14508:	0405051b          	addiw	a0,a0,64
   1450c:	f85ff06f          	j	14490 <__addtf3+0x8e0>
   14510:	fb45071b          	addiw	a4,a0,-76
   14514:	00ea1733          	sll	a4,s4,a4
   14518:	00000513          	li	a0,0
   1451c:	f9dff06f          	j	144b8 <__addtf3+0x908>
   14520:	fc14041b          	addiw	s0,s0,-63
   14524:	04000593          	li	a1,64
   14528:	00875433          	srl	s0,a4,s0
   1452c:	00000693          	li	a3,0
   14530:	00b78863          	beq	a5,a1,14540 <__addtf3+0x990>
   14534:	08000693          	li	a3,128
   14538:	40f686bb          	subw	a3,a3,a5
   1453c:	00d716b3          	sll	a3,a4,a3
   14540:	00d56533          	or	a0,a0,a3
   14544:	00a03533          	snez	a0,a0
   14548:	00a46533          	or	a0,s0,a0
   1454c:	ea1ff06f          	j	143ec <__addtf3+0x83c>
   14550:	fff00793          	li	a5,-1
   14554:	03379793          	slli	a5,a5,0x33
   14558:	fff78793          	addi	a5,a5,-1
   1455c:	40b405b3          	sub	a1,s0,a1
   14560:	00f77733          	and	a4,a4,a5
   14564:	00070793          	mv	a5,a4
   14568:	dc058ee3          	beqz	a1,14344 <__addtf3+0x794>
   1456c:	f58ff06f          	j	13cc4 <__addtf3+0x114>
   14570:	00060513          	mv	a0,a2
   14574:	00088913          	mv	s2,a7
   14578:	fedff06f          	j	14564 <__addtf3+0x9b4>
   1457c:	00070793          	mv	a5,a4
   14580:	00060513          	mv	a0,a2
   14584:	dc1ff06f          	j	14344 <__addtf3+0x794>
   14588:	00070793          	mv	a5,a4
   1458c:	00060513          	mv	a0,a2
   14590:	e45ff06f          	j	143d4 <__addtf3+0x824>
   14594:	00068793          	mv	a5,a3
   14598:	00058513          	mv	a0,a1
   1459c:	da9ff06f          	j	14344 <__addtf3+0x794>
   145a0:	00100693          	li	a3,1
   145a4:	de0916e3          	bnez	s2,14390 <__addtf3+0x7e0>
   145a8:	ff873713          	sltiu	a4,a4,-8
   145ac:	00174713          	xori	a4,a4,1
   145b0:	02071713          	slli	a4,a4,0x20
   145b4:	02075713          	srli	a4,a4,0x20
   145b8:	00e80833          	add	a6,a6,a4
   145bc:	dd5ff06f          	j	14390 <__addtf3+0x7e0>
   145c0:	00100693          	li	a3,1
   145c4:	dc0906e3          	beqz	s2,14390 <__addtf3+0x7e0>
   145c8:	fe1ff06f          	j	145a8 <__addtf3+0x9f8>
   145cc:	00000793          	li	a5,0
   145d0:	00000693          	li	a3,0
   145d4:	9a5ff06f          	j	13f78 <__addtf3+0x3c8>
   145d8:	00070793          	mv	a5,a4
   145dc:	00060513          	mv	a0,a2
   145e0:	ee4ff06f          	j	13cc4 <__addtf3+0x114>
   145e4:	00000793          	li	a5,0
   145e8:	00000513          	li	a0,0
   145ec:	00040593          	mv	a1,s0
   145f0:	fe1ff06f          	j	145d0 <__addtf3+0xa20>
   145f4:	00000793          	li	a5,0
   145f8:	00088913          	mv	s2,a7
   145fc:	fd5ff06f          	j	145d0 <__addtf3+0xa20>
   14600:	00070793          	mv	a5,a4
   14604:	00060513          	mv	a0,a2
   14608:	00088913          	mv	s2,a7
   1460c:	eb8ff06f          	j	13cc4 <__addtf3+0x114>
   14610:	00100793          	li	a5,1
   14614:	000085b7          	lui	a1,0x8
   14618:	00000513          	li	a0,0
   1461c:	00000913          	li	s2,0
   14620:	03279793          	slli	a5,a5,0x32
   14624:	fff58593          	addi	a1,a1,-1 # 7fff <exit-0x8121>
   14628:	01000693          	li	a3,16
   1462c:	94dff06f          	j	13f78 <__addtf3+0x3c8>
   14630:	00000793          	li	a5,0
   14634:	00000513          	li	a0,0
   14638:	00000593          	li	a1,0
   1463c:	f95ff06f          	j	145d0 <__addtf3+0xa20>
   14640:	00091863          	bnez	s2,14650 <__addtf3+0xaa0>
   14644:	00850713          	addi	a4,a0,8
   14648:	a69ff06f          	j	140b0 <__addtf3+0x500>
   1464c:	fe091ce3          	bnez	s2,14644 <__addtf3+0xa94>
   14650:	8a0814e3          	bnez	a6,13ef8 <__addtf3+0x348>
   14654:	925ff06f          	j	13f78 <__addtf3+0x3c8>
   14658:	00000513          	li	a0,0
   1465c:	02048463          	beqz	s1,14684 <__addtf3+0xad4>
   14660:	00300793          	li	a5,3
   14664:	00f49a63          	bne	s1,a5,14678 <__addtf3+0xac8>
   14668:	00090e63          	beqz	s2,14684 <__addtf3+0xad4>
   1466c:	fff00513          	li	a0,-1
   14670:	ffe70593          	addi	a1,a4,-2
   14674:	0100006f          	j	14684 <__addtf3+0xad4>
   14678:	00200793          	li	a5,2
   1467c:	fef498e3          	bne	s1,a5,1466c <__addtf3+0xabc>
   14680:	fe0906e3          	beqz	s2,1466c <__addtf3+0xabc>
   14684:	0056e693          	ori	a3,a3,5
   14688:	00050793          	mv	a5,a0
   1468c:	919ff06f          	j	13fa4 <__addtf3+0x3f4>
   14690:	00000693          	li	a3,0
   14694:	da8590e3          	bne	a1,s0,14434 <__addtf3+0x884>
   14698:	d79ff06f          	j	14410 <__addtf3+0x860>
   1469c:	00000693          	li	a3,0
   146a0:	8a8592e3          	bne	a1,s0,13f44 <__addtf3+0x394>
   146a4:	879ff06f          	j	13f1c <__addtf3+0x36c>

00000000000146a8 <__eqtf2>:
   146a8:	00050813          	mv	a6,a0
   146ac:	002027f3          	frrm	a5
   146b0:	00008537          	lui	a0,0x8
   146b4:	0305d893          	srli	a7,a1,0x30
   146b8:	fff50513          	addi	a0,a0,-1 # 7fff <exit-0x8121>
   146bc:	fff00793          	li	a5,-1
   146c0:	0107d793          	srli	a5,a5,0x10
   146c4:	0306d313          	srli	t1,a3,0x30
   146c8:	00a8f8b3          	and	a7,a7,a0
   146cc:	00f5f733          	and	a4,a1,a5
   146d0:	00a37333          	and	t1,t1,a0
   146d4:	00f6f7b3          	and	a5,a3,a5
   146d8:	03f5d593          	srli	a1,a1,0x3f
   146dc:	03f6d693          	srli	a3,a3,0x3f
   146e0:	00a89a63          	bne	a7,a0,146f4 <__eqtf2+0x4c>
   146e4:	01076533          	or	a0,a4,a6
   146e8:	02051663          	bnez	a0,14714 <__eqtf2+0x6c>
   146ec:	05131863          	bne	t1,a7,1473c <__eqtf2+0x94>
   146f0:	0080006f          	j	146f8 <__eqtf2+0x50>
   146f4:	04a31863          	bne	t1,a0,14744 <__eqtf2+0x9c>
   146f8:	00c7e533          	or	a0,a5,a2
   146fc:	04050463          	beqz	a0,14744 <__eqtf2+0x9c>
   14700:	000086b7          	lui	a3,0x8
   14704:	fff68693          	addi	a3,a3,-1 # 7fff <exit-0x8121>
   14708:	02d89663          	bne	a7,a3,14734 <__eqtf2+0x8c>
   1470c:	01076533          	or	a0,a4,a6
   14710:	02050263          	beqz	a0,14734 <__eqtf2+0x8c>
   14714:	02f75713          	srli	a4,a4,0x2f
   14718:	04070c63          	beqz	a4,14770 <__eqtf2+0xc8>
   1471c:	00008737          	lui	a4,0x8
   14720:	fff70713          	addi	a4,a4,-1 # 7fff <exit-0x8121>
   14724:	00100513          	li	a0,1
   14728:	00e31c63          	bne	t1,a4,14740 <__eqtf2+0x98>
   1472c:	00c7e633          	or	a2,a5,a2
   14730:	00060863          	beqz	a2,14740 <__eqtf2+0x98>
   14734:	02f7d793          	srli	a5,a5,0x2f
   14738:	02078c63          	beqz	a5,14770 <__eqtf2+0xc8>
   1473c:	00100513          	li	a0,1
   14740:	00008067          	ret
   14744:	00100513          	li	a0,1
   14748:	fe689ce3          	bne	a7,t1,14740 <__eqtf2+0x98>
   1474c:	fef71ae3          	bne	a4,a5,14740 <__eqtf2+0x98>
   14750:	fec818e3          	bne	a6,a2,14740 <__eqtf2+0x98>
   14754:	00d58a63          	beq	a1,a3,14768 <__eqtf2+0xc0>
   14758:	fe0894e3          	bnez	a7,14740 <__eqtf2+0x98>
   1475c:	01076533          	or	a0,a4,a6
   14760:	00a03533          	snez	a0,a0
   14764:	00008067          	ret
   14768:	00000513          	li	a0,0
   1476c:	00008067          	ret
   14770:	00186073          	csrsi	fflags,16
   14774:	fc9ff06f          	j	1473c <__eqtf2+0x94>

0000000000014778 <__multf3>:
   14778:	fa010113          	addi	sp,sp,-96
   1477c:	03613023          	sd	s6,32(sp)
   14780:	01913423          	sd	s9,8(sp)
   14784:	04113c23          	sd	ra,88(sp)
   14788:	04813823          	sd	s0,80(sp)
   1478c:	04913423          	sd	s1,72(sp)
   14790:	05213023          	sd	s2,64(sp)
   14794:	03313c23          	sd	s3,56(sp)
   14798:	03413823          	sd	s4,48(sp)
   1479c:	03513423          	sd	s5,40(sp)
   147a0:	01713c23          	sd	s7,24(sp)
   147a4:	01813823          	sd	s8,16(sp)
   147a8:	00060c93          	mv	s9,a2
   147ac:	00068b13          	mv	s6,a3
   147b0:	00202973          	frrm	s2
   147b4:	00008737          	lui	a4,0x8
   147b8:	0305da93          	srli	s5,a1,0x30
   147bc:	fff70713          	addi	a4,a4,-1 # 7fff <exit-0x8121>
   147c0:	01059993          	slli	s3,a1,0x10
   147c4:	00eaf7b3          	and	a5,s5,a4
   147c8:	0009091b          	sext.w	s2,s2
   147cc:	0109d993          	srli	s3,s3,0x10
   147d0:	03f5db93          	srli	s7,a1,0x3f
   147d4:	04078263          	beqz	a5,14818 <__multf3+0xa0>
   147d8:	00050a13          	mv	s4,a0
   147dc:	00078a9b          	sext.w	s5,a5
   147e0:	0ae78663          	beq	a5,a4,1488c <__multf3+0x114>
   147e4:	00399993          	slli	s3,s3,0x3
   147e8:	03d55713          	srli	a4,a0,0x3d
   147ec:	01376733          	or	a4,a4,s3
   147f0:	ffffcab7          	lui	s5,0xffffc
   147f4:	00100993          	li	s3,1
   147f8:	03399993          	slli	s3,s3,0x33
   147fc:	001a8a93          	addi	s5,s5,1 # ffffffffffffc001 <__BSS_END__+0xfffffffffffe37f9>
   14800:	013769b3          	or	s3,a4,s3
   14804:	00351a13          	slli	s4,a0,0x3
   14808:	01578ab3          	add	s5,a5,s5
   1480c:	00000c13          	li	s8,0
   14810:	00000493          	li	s1,0
   14814:	0940006f          	j	148a8 <__multf3+0x130>
   14818:	00a9e7b3          	or	a5,s3,a0
   1481c:	00050413          	mv	s0,a0
   14820:	10078e63          	beqz	a5,1493c <__multf3+0x1c4>
   14824:	04098663          	beqz	s3,14870 <__multf3+0xf8>
   14828:	00098513          	mv	a0,s3
   1482c:	654010ef          	jal	ra,15e80 <__clzdi2>
   14830:	0005079b          	sext.w	a5,a0
   14834:	ff178693          	addi	a3,a5,-15
   14838:	03c00613          	li	a2,60
   1483c:	0006871b          	sext.w	a4,a3
   14840:	02d64e63          	blt	a2,a3,1487c <__multf3+0x104>
   14844:	03d00693          	li	a3,61
   14848:	00370a1b          	addiw	s4,a4,3
   1484c:	40e6873b          	subw	a4,a3,a4
   14850:	014999b3          	sll	s3,s3,s4
   14854:	00e45733          	srl	a4,s0,a4
   14858:	013769b3          	or	s3,a4,s3
   1485c:	01441a33          	sll	s4,s0,s4
   14860:	ffffcab7          	lui	s5,0xffffc
   14864:	011a8a93          	addi	s5,s5,17 # ffffffffffffc011 <__BSS_END__+0xfffffffffffe3809>
   14868:	40fa8ab3          	sub	s5,s5,a5
   1486c:	fa1ff06f          	j	1480c <__multf3+0x94>
   14870:	610010ef          	jal	ra,15e80 <__clzdi2>
   14874:	0405079b          	addiw	a5,a0,64
   14878:	fbdff06f          	j	14834 <__multf3+0xbc>
   1487c:	fc37071b          	addiw	a4,a4,-61
   14880:	00e419b3          	sll	s3,s0,a4
   14884:	00000a13          	li	s4,0
   14888:	fd9ff06f          	j	14860 <__multf3+0xe8>
   1488c:	00a9e7b3          	or	a5,s3,a0
   14890:	0c078063          	beqz	a5,14950 <__multf3+0x1d8>
   14894:	02f9d793          	srli	a5,s3,0x2f
   14898:	0017f793          	andi	a5,a5,1
   1489c:	00300c13          	li	s8,3
   148a0:	01000493          	li	s1,16
   148a4:	f60796e3          	bnez	a5,14810 <__multf3+0x98>
   148a8:	000086b7          	lui	a3,0x8
   148ac:	030b5513          	srli	a0,s6,0x30
   148b0:	fff68693          	addi	a3,a3,-1 # 7fff <exit-0x8121>
   148b4:	010b1413          	slli	s0,s6,0x10
   148b8:	00d57733          	and	a4,a0,a3
   148bc:	000c8793          	mv	a5,s9
   148c0:	01045413          	srli	s0,s0,0x10
   148c4:	03fb5b13          	srli	s6,s6,0x3f
   148c8:	08070c63          	beqz	a4,14960 <__multf3+0x1e8>
   148cc:	0007051b          	sext.w	a0,a4
   148d0:	10d70263          	beq	a4,a3,149d4 <__multf3+0x25c>
   148d4:	00341413          	slli	s0,s0,0x3
   148d8:	03dcd693          	srli	a3,s9,0x3d
   148dc:	0086e6b3          	or	a3,a3,s0
   148e0:	ffffc537          	lui	a0,0xffffc
   148e4:	00100413          	li	s0,1
   148e8:	03341413          	slli	s0,s0,0x33
   148ec:	00150513          	addi	a0,a0,1 # ffffffffffffc001 <__BSS_END__+0xfffffffffffe37f9>
   148f0:	0086e433          	or	s0,a3,s0
   148f4:	003c9793          	slli	a5,s9,0x3
   148f8:	00a70533          	add	a0,a4,a0
   148fc:	00000713          	li	a4,0
   14900:	002c1693          	slli	a3,s8,0x2
   14904:	00e6e6b3          	or	a3,a3,a4
   14908:	01550ab3          	add	s5,a0,s5
   1490c:	fff68693          	addi	a3,a3,-1
   14910:	00e00593          	li	a1,14
   14914:	016bc633          	xor	a2,s7,s6
   14918:	001a8813          	addi	a6,s5,1
   1491c:	16d5ee63          	bltu	a1,a3,14a98 <__multf3+0x320>
   14920:	00002597          	auipc	a1,0x2
   14924:	17058593          	addi	a1,a1,368 # 16a90 <errmsg+0x730>
   14928:	00269693          	slli	a3,a3,0x2
   1492c:	00b686b3          	add	a3,a3,a1
   14930:	0006a683          	lw	a3,0(a3)
   14934:	00b686b3          	add	a3,a3,a1
   14938:	00068067          	jr	a3
   1493c:	00000993          	li	s3,0
   14940:	00000a13          	li	s4,0
   14944:	00000a93          	li	s5,0
   14948:	00100c13          	li	s8,1
   1494c:	ec5ff06f          	j	14810 <__multf3+0x98>
   14950:	00000993          	li	s3,0
   14954:	00000a13          	li	s4,0
   14958:	00200c13          	li	s8,2
   1495c:	eb5ff06f          	j	14810 <__multf3+0x98>
   14960:	008ce7b3          	or	a5,s9,s0
   14964:	08078863          	beqz	a5,149f4 <__multf3+0x27c>
   14968:	04040663          	beqz	s0,149b4 <__multf3+0x23c>
   1496c:	00040513          	mv	a0,s0
   14970:	510010ef          	jal	ra,15e80 <__clzdi2>
   14974:	0005071b          	sext.w	a4,a0
   14978:	ff170793          	addi	a5,a4,-15
   1497c:	03c00613          	li	a2,60
   14980:	0007869b          	sext.w	a3,a5
   14984:	04f64063          	blt	a2,a5,149c4 <__multf3+0x24c>
   14988:	03d00613          	li	a2,61
   1498c:	0036879b          	addiw	a5,a3,3
   14990:	40d606bb          	subw	a3,a2,a3
   14994:	00f41433          	sll	s0,s0,a5
   14998:	00dcd6b3          	srl	a3,s9,a3
   1499c:	0086e433          	or	s0,a3,s0
   149a0:	00fc97b3          	sll	a5,s9,a5
   149a4:	ffffc537          	lui	a0,0xffffc
   149a8:	01150513          	addi	a0,a0,17 # ffffffffffffc011 <__BSS_END__+0xfffffffffffe3809>
   149ac:	40e50533          	sub	a0,a0,a4
   149b0:	f4dff06f          	j	148fc <__multf3+0x184>
   149b4:	000c8513          	mv	a0,s9
   149b8:	4c8010ef          	jal	ra,15e80 <__clzdi2>
   149bc:	0405071b          	addiw	a4,a0,64
   149c0:	fb9ff06f          	j	14978 <__multf3+0x200>
   149c4:	fc36869b          	addiw	a3,a3,-61
   149c8:	00dc9433          	sll	s0,s9,a3
   149cc:	00000793          	li	a5,0
   149d0:	fd5ff06f          	j	149a4 <__multf3+0x22c>
   149d4:	008ce733          	or	a4,s9,s0
   149d8:	02070863          	beqz	a4,14a08 <__multf3+0x290>
   149dc:	02f45713          	srli	a4,s0,0x2f
   149e0:	00177713          	andi	a4,a4,1
   149e4:	02071a63          	bnez	a4,14a18 <__multf3+0x2a0>
   149e8:	00300713          	li	a4,3
   149ec:	01000493          	li	s1,16
   149f0:	f11ff06f          	j	14900 <__multf3+0x188>
   149f4:	00000413          	li	s0,0
   149f8:	00000793          	li	a5,0
   149fc:	00000513          	li	a0,0
   14a00:	00100713          	li	a4,1
   14a04:	efdff06f          	j	14900 <__multf3+0x188>
   14a08:	00000413          	li	s0,0
   14a0c:	00000793          	li	a5,0
   14a10:	00200713          	li	a4,2
   14a14:	eedff06f          	j	14900 <__multf3+0x188>
   14a18:	00300713          	li	a4,3
   14a1c:	ee5ff06f          	j	14900 <__multf3+0x188>
   14a20:	00100413          	li	s0,1
   14a24:	00008737          	lui	a4,0x8
   14a28:	02f41413          	slli	s0,s0,0x2f
   14a2c:	00000513          	li	a0,0
   14a30:	fff70713          	addi	a4,a4,-1 # 7fff <exit-0x8121>
   14a34:	00000613          	li	a2,0
   14a38:	01000493          	li	s1,16
   14a3c:	03171713          	slli	a4,a4,0x31
   14a40:	03175713          	srli	a4,a4,0x31
   14a44:	00f61613          	slli	a2,a2,0xf
   14a48:	01041413          	slli	s0,s0,0x10
   14a4c:	00e66733          	or	a4,a2,a4
   14a50:	03071713          	slli	a4,a4,0x30
   14a54:	01045593          	srli	a1,s0,0x10
   14a58:	00e5e5b3          	or	a1,a1,a4
   14a5c:	00048463          	beqz	s1,14a64 <__multf3+0x2ec>
   14a60:	0014a073          	csrs	fflags,s1
   14a64:	05813083          	ld	ra,88(sp)
   14a68:	05013403          	ld	s0,80(sp)
   14a6c:	04813483          	ld	s1,72(sp)
   14a70:	04013903          	ld	s2,64(sp)
   14a74:	03813983          	ld	s3,56(sp)
   14a78:	03013a03          	ld	s4,48(sp)
   14a7c:	02813a83          	ld	s5,40(sp)
   14a80:	02013b03          	ld	s6,32(sp)
   14a84:	01813b83          	ld	s7,24(sp)
   14a88:	01013c03          	ld	s8,16(sp)
   14a8c:	00813c83          	ld	s9,8(sp)
   14a90:	06010113          	addi	sp,sp,96
   14a94:	00008067          	ret
   14a98:	fff00513          	li	a0,-1
   14a9c:	02055513          	srli	a0,a0,0x20
   14aa0:	014983b3          	add	t2,s3,s4
   14aa4:	00878f33          	add	t5,a5,s0
   14aa8:	020a5e93          	srli	t4,s4,0x20
   14aac:	0207d693          	srli	a3,a5,0x20
   14ab0:	0143bfb3          	sltu	t6,t2,s4
   14ab4:	00ff32b3          	sltu	t0,t5,a5
   14ab8:	00aa7a33          	and	s4,s4,a0
   14abc:	00a7f7b3          	and	a5,a5,a0
   14ac0:	03478533          	mul	a0,a5,s4
   14ac4:	000f8e13          	mv	t3,t6
   14ac8:	00028313          	mv	t1,t0
   14acc:	02fe87b3          	mul	a5,t4,a5
   14ad0:	02de8733          	mul	a4,t4,a3
   14ad4:	034686b3          	mul	a3,a3,s4
   14ad8:	02055a13          	srli	s4,a0,0x20
   14adc:	00f686b3          	add	a3,a3,a5
   14ae0:	00da0a33          	add	s4,s4,a3
   14ae4:	00fa7863          	bgeu	s4,a5,14af4 <__multf3+0x37c>
   14ae8:	00100793          	li	a5,1
   14aec:	02079793          	slli	a5,a5,0x20
   14af0:	00f70733          	add	a4,a4,a5
   14af4:	fff00693          	li	a3,-1
   14af8:	0206d693          	srli	a3,a3,0x20
   14afc:	00da75b3          	and	a1,s4,a3
   14b00:	00d57533          	and	a0,a0,a3
   14b04:	02059593          	slli	a1,a1,0x20
   14b08:	0203d893          	srli	a7,t2,0x20
   14b0c:	020f5793          	srli	a5,t5,0x20
   14b10:	020a5e93          	srli	t4,s4,0x20
   14b14:	00a585b3          	add	a1,a1,a0
   14b18:	00d3f533          	and	a0,t2,a3
   14b1c:	00df76b3          	and	a3,t5,a3
   14b20:	02f88b33          	mul	s6,a7,a5
   14b24:	00ee8eb3          	add	t4,t4,a4
   14b28:	02a787b3          	mul	a5,a5,a0
   14b2c:	02a68733          	mul	a4,a3,a0
   14b30:	02d886b3          	mul	a3,a7,a3
   14b34:	00d78533          	add	a0,a5,a3
   14b38:	02075793          	srli	a5,a4,0x20
   14b3c:	00a787b3          	add	a5,a5,a0
   14b40:	00d7f863          	bgeu	a5,a3,14b50 <__multf3+0x3d8>
   14b44:	00100693          	li	a3,1
   14b48:	02069693          	slli	a3,a3,0x20
   14b4c:	00db0b33          	add	s6,s6,a3
   14b50:	fff00893          	li	a7,-1
   14b54:	0208d893          	srli	a7,a7,0x20
   14b58:	0117f6b3          	and	a3,a5,a7
   14b5c:	02045b93          	srli	s7,s0,0x20
   14b60:	01177533          	and	a0,a4,a7
   14b64:	0207da13          	srli	s4,a5,0x20
   14b68:	0209d793          	srli	a5,s3,0x20
   14b6c:	0119f9b3          	and	s3,s3,a7
   14b70:	011478b3          	and	a7,s0,a7
   14b74:	03198433          	mul	s0,s3,a7
   14b78:	02069693          	slli	a3,a3,0x20
   14b7c:	00a68533          	add	a0,a3,a0
   14b80:	031788b3          	mul	a7,a5,a7
   14b84:	037787b3          	mul	a5,a5,s7
   14b88:	033b8bb3          	mul	s7,s7,s3
   14b8c:	02045993          	srli	s3,s0,0x20
   14b90:	011b8bb3          	add	s7,s7,a7
   14b94:	017989b3          	add	s3,s3,s7
   14b98:	0119f863          	bgeu	s3,a7,14ba8 <__multf3+0x430>
   14b9c:	00100713          	li	a4,1
   14ba0:	02071713          	slli	a4,a4,0x20
   14ba4:	00e787b3          	add	a5,a5,a4
   14ba8:	0209d693          	srli	a3,s3,0x20
   14bac:	00f687b3          	add	a5,a3,a5
   14bb0:	fff00693          	li	a3,-1
   14bb4:	0206d693          	srli	a3,a3,0x20
   14bb8:	00d9f733          	and	a4,s3,a3
   14bbc:	00d47433          	and	s0,s0,a3
   14bc0:	01d506b3          	add	a3,a0,t4
   14bc4:	01d6b533          	sltu	a0,a3,t4
   14bc8:	00aa08b3          	add	a7,s4,a0
   14bcc:	40600333          	neg	t1,t1
   14bd0:	016888b3          	add	a7,a7,s6
   14bd4:	007373b3          	and	t2,t1,t2
   14bd8:	41c00e33          	neg	t3,t3
   14bdc:	011383b3          	add	t2,t2,a7
   14be0:	01ee7f33          	and	t5,t3,t5
   14be4:	00a8b533          	sltu	a0,a7,a0
   14be8:	005fffb3          	and	t6,t6,t0
   14bec:	007f0f33          	add	t5,t5,t2
   14bf0:	0113b8b3          	sltu	a7,t2,a7
   14bf4:	01f50533          	add	a0,a0,t6
   14bf8:	40b68333          	sub	t1,a3,a1
   14bfc:	01150533          	add	a0,a0,a7
   14c00:	0066be33          	sltu	t3,a3,t1
   14c04:	41df08b3          	sub	a7,t5,t4
   14c08:	02071713          	slli	a4,a4,0x20
   14c0c:	007f33b3          	sltu	t2,t5,t2
   14c10:	011f3fb3          	sltu	t6,t5,a7
   14c14:	00870733          	add	a4,a4,s0
   14c18:	41c888b3          	sub	a7,a7,t3
   14c1c:	00750533          	add	a0,a0,t2
   14c20:	00000e13          	li	t3,0
   14c24:	0066f663          	bgeu	a3,t1,14c30 <__multf3+0x4b8>
   14c28:	41ee8f33          	sub	t5,t4,t5
   14c2c:	001f3e13          	seqz	t3,t5
   14c30:	01fe6f33          	or	t5,t3,t6
   14c34:	40e30e33          	sub	t3,t1,a4
   14c38:	40f886b3          	sub	a3,a7,a5
   14c3c:	00f50533          	add	a0,a0,a5
   14c40:	01c337b3          	sltu	a5,t1,t3
   14c44:	00d8b8b3          	sltu	a7,a7,a3
   14c48:	40f707b3          	sub	a5,a4,a5
   14c4c:	00000e93          	li	t4,0
   14c50:	01c37463          	bgeu	t1,t3,14c58 <__multf3+0x4e0>
   14c54:	0016be93          	seqz	t4,a3
   14c58:	00f686b3          	add	a3,a3,a5
   14c5c:	00e6b733          	sltu	a4,a3,a4
   14c60:	00a70733          	add	a4,a4,a0
   14c64:	41e70733          	sub	a4,a4,t5
   14c68:	011ee8b3          	or	a7,t4,a7
   14c6c:	41170733          	sub	a4,a4,a7
   14c70:	00de1793          	slli	a5,t3,0xd
   14c74:	00d71713          	slli	a4,a4,0xd
   14c78:	0336d413          	srli	s0,a3,0x33
   14c7c:	00b7e7b3          	or	a5,a5,a1
   14c80:	00876433          	or	s0,a4,s0
   14c84:	00f037b3          	snez	a5,a5
   14c88:	033e5513          	srli	a0,t3,0x33
   14c8c:	03475713          	srli	a4,a4,0x34
   14c90:	00a7e7b3          	or	a5,a5,a0
   14c94:	00d69693          	slli	a3,a3,0xd
   14c98:	00177713          	andi	a4,a4,1
   14c9c:	00d7e7b3          	or	a5,a5,a3
   14ca0:	0a070463          	beqz	a4,14d48 <__multf3+0x5d0>
   14ca4:	0017d713          	srli	a4,a5,0x1
   14ca8:	0017f793          	andi	a5,a5,1
   14cac:	03f41513          	slli	a0,s0,0x3f
   14cb0:	00f767b3          	or	a5,a4,a5
   14cb4:	00a7e7b3          	or	a5,a5,a0
   14cb8:	00145413          	srli	s0,s0,0x1
   14cbc:	00004737          	lui	a4,0x4
   14cc0:	fff70713          	addi	a4,a4,-1 # 3fff <exit-0xc121>
   14cc4:	00e80733          	add	a4,a6,a4
   14cc8:	12e05263          	blez	a4,14dec <__multf3+0x674>
   14ccc:	0077f693          	andi	a3,a5,7
   14cd0:	08068863          	beqz	a3,14d60 <__multf3+0x5e8>
   14cd4:	00200693          	li	a3,2
   14cd8:	0014e493          	ori	s1,s1,1
   14cdc:	08d90063          	beq	s2,a3,14d5c <__multf3+0x5e4>
   14ce0:	00300693          	li	a3,3
   14ce4:	06d90663          	beq	s2,a3,14d50 <__multf3+0x5d8>
   14ce8:	06091c63          	bnez	s2,14d60 <__multf3+0x5e8>
   14cec:	00f7f693          	andi	a3,a5,15
   14cf0:	00400593          	li	a1,4
   14cf4:	06b68663          	beq	a3,a1,14d60 <__multf3+0x5e8>
   14cf8:	00478693          	addi	a3,a5,4
   14cfc:	00f6b7b3          	sltu	a5,a3,a5
   14d00:	00f40433          	add	s0,s0,a5
   14d04:	00068793          	mv	a5,a3
   14d08:	0580006f          	j	14d60 <__multf3+0x5e8>
   14d0c:	000b8613          	mv	a2,s7
   14d10:	00098413          	mv	s0,s3
   14d14:	000a0793          	mv	a5,s4
   14d18:	000c0713          	mv	a4,s8
   14d1c:	00200693          	li	a3,2
   14d20:	2ad70063          	beq	a4,a3,14fc0 <__multf3+0x848>
   14d24:	00300693          	li	a3,3
   14d28:	2ad70663          	beq	a4,a3,14fd4 <__multf3+0x85c>
   14d2c:	00100693          	li	a3,1
   14d30:	f8d716e3          	bne	a4,a3,14cbc <__multf3+0x544>
   14d34:	00000413          	li	s0,0
   14d38:	00000513          	li	a0,0
   14d3c:	2600006f          	j	14f9c <__multf3+0x824>
   14d40:	000b0613          	mv	a2,s6
   14d44:	fd9ff06f          	j	14d1c <__multf3+0x5a4>
   14d48:	000a8813          	mv	a6,s5
   14d4c:	f71ff06f          	j	14cbc <__multf3+0x544>
   14d50:	00061863          	bnez	a2,14d60 <__multf3+0x5e8>
   14d54:	00878693          	addi	a3,a5,8
   14d58:	fa5ff06f          	j	14cfc <__multf3+0x584>
   14d5c:	fe061ce3          	bnez	a2,14d54 <__multf3+0x5dc>
   14d60:	03445693          	srli	a3,s0,0x34
   14d64:	0016f693          	andi	a3,a3,1
   14d68:	00068e63          	beqz	a3,14d84 <__multf3+0x60c>
   14d6c:	fff00713          	li	a4,-1
   14d70:	03471713          	slli	a4,a4,0x34
   14d74:	fff70713          	addi	a4,a4,-1
   14d78:	00e47433          	and	s0,s0,a4
   14d7c:	00004737          	lui	a4,0x4
   14d80:	00e80733          	add	a4,a6,a4
   14d84:	000086b7          	lui	a3,0x8
   14d88:	ffe68593          	addi	a1,a3,-2 # 7ffe <exit-0x8122>
   14d8c:	00e5cc63          	blt	a1,a4,14da4 <__multf3+0x62c>
   14d90:	03d41513          	slli	a0,s0,0x3d
   14d94:	0037d793          	srli	a5,a5,0x3
   14d98:	00f56533          	or	a0,a0,a5
   14d9c:	00345413          	srli	s0,s0,0x3
   14da0:	c9dff06f          	j	14a3c <__multf3+0x2c4>
   14da4:	00200793          	li	a5,2
   14da8:	02f90a63          	beq	s2,a5,14ddc <__multf3+0x664>
   14dac:	00300793          	li	a5,3
   14db0:	fff68713          	addi	a4,a3,-1
   14db4:	00f90863          	beq	s2,a5,14dc4 <__multf3+0x64c>
   14db8:	00091863          	bnez	s2,14dc8 <__multf3+0x650>
   14dbc:	00000513          	li	a0,0
   14dc0:	0100006f          	j	14dd0 <__multf3+0x658>
   14dc4:	fe060ce3          	beqz	a2,14dbc <__multf3+0x644>
   14dc8:	fff00513          	li	a0,-1
   14dcc:	00058713          	mv	a4,a1
   14dd0:	0054e493          	ori	s1,s1,5
   14dd4:	00050413          	mv	s0,a0
   14dd8:	c65ff06f          	j	14a3c <__multf3+0x2c4>
   14ddc:	fe0606e3          	beqz	a2,14dc8 <__multf3+0x650>
   14de0:	00000513          	li	a0,0
   14de4:	fff68713          	addi	a4,a3,-1
   14de8:	fe9ff06f          	j	14dd0 <__multf3+0x658>
   14dec:	00100693          	li	a3,1
   14df0:	06071463          	bnez	a4,14e58 <__multf3+0x6e0>
   14df4:	0077f593          	andi	a1,a5,7
   14df8:	00040693          	mv	a3,s0
   14dfc:	04058863          	beqz	a1,14e4c <__multf3+0x6d4>
   14e00:	00200593          	li	a1,2
   14e04:	0014e493          	ori	s1,s1,1
   14e08:	04b90063          	beq	s2,a1,14e48 <__multf3+0x6d0>
   14e0c:	00300593          	li	a1,3
   14e10:	02b90663          	beq	s2,a1,14e3c <__multf3+0x6c4>
   14e14:	02091c63          	bnez	s2,14e4c <__multf3+0x6d4>
   14e18:	00f7f593          	andi	a1,a5,15
   14e1c:	00400513          	li	a0,4
   14e20:	02a58663          	beq	a1,a0,14e4c <__multf3+0x6d4>
   14e24:	ffc7b693          	sltiu	a3,a5,-4
   14e28:	0016c693          	xori	a3,a3,1
   14e2c:	02069693          	slli	a3,a3,0x20
   14e30:	0206d693          	srli	a3,a3,0x20
   14e34:	008686b3          	add	a3,a3,s0
   14e38:	0140006f          	j	14e4c <__multf3+0x6d4>
   14e3c:	00061863          	bnez	a2,14e4c <__multf3+0x6d4>
   14e40:	ff87b693          	sltiu	a3,a5,-8
   14e44:	fe5ff06f          	j	14e28 <__multf3+0x6b0>
   14e48:	fe061ce3          	bnez	a2,14e40 <__multf3+0x6c8>
   14e4c:	0346d693          	srli	a3,a3,0x34
   14e50:	0016c693          	xori	a3,a3,1
   14e54:	0016f693          	andi	a3,a3,1
   14e58:	00100813          	li	a6,1
   14e5c:	40e80833          	sub	a6,a6,a4
   14e60:	07400713          	li	a4,116
   14e64:	11074263          	blt	a4,a6,14f68 <__multf3+0x7f0>
   14e68:	03f00593          	li	a1,63
   14e6c:	0008071b          	sext.w	a4,a6
   14e70:	0705c463          	blt	a1,a6,14ed8 <__multf3+0x760>
   14e74:	04000593          	li	a1,64
   14e78:	40e585bb          	subw	a1,a1,a4
   14e7c:	00b41533          	sll	a0,s0,a1
   14e80:	00e7d833          	srl	a6,a5,a4
   14e84:	00b797b3          	sll	a5,a5,a1
   14e88:	01056533          	or	a0,a0,a6
   14e8c:	00f037b3          	snez	a5,a5
   14e90:	00f56533          	or	a0,a0,a5
   14e94:	00e45433          	srl	s0,s0,a4
   14e98:	00757793          	andi	a5,a0,7
   14e9c:	08078063          	beqz	a5,14f1c <__multf3+0x7a4>
   14ea0:	00200793          	li	a5,2
   14ea4:	0014e493          	ori	s1,s1,1
   14ea8:	06f90863          	beq	s2,a5,14f18 <__multf3+0x7a0>
   14eac:	00300793          	li	a5,3
   14eb0:	04f90e63          	beq	s2,a5,14f0c <__multf3+0x794>
   14eb4:	06091463          	bnez	s2,14f1c <__multf3+0x7a4>
   14eb8:	00f57793          	andi	a5,a0,15
   14ebc:	00400713          	li	a4,4
   14ec0:	04e78e63          	beq	a5,a4,14f1c <__multf3+0x7a4>
   14ec4:	00450793          	addi	a5,a0,4
   14ec8:	00a7b533          	sltu	a0,a5,a0
   14ecc:	00a40433          	add	s0,s0,a0
   14ed0:	00078513          	mv	a0,a5
   14ed4:	0480006f          	j	14f1c <__multf3+0x7a4>
   14ed8:	fc07051b          	addiw	a0,a4,-64
   14edc:	04000893          	li	a7,64
   14ee0:	00a45533          	srl	a0,s0,a0
   14ee4:	00000593          	li	a1,0
   14ee8:	01180863          	beq	a6,a7,14ef8 <__multf3+0x780>
   14eec:	08000593          	li	a1,128
   14ef0:	40e5873b          	subw	a4,a1,a4
   14ef4:	00e415b3          	sll	a1,s0,a4
   14ef8:	00f5e7b3          	or	a5,a1,a5
   14efc:	00f037b3          	snez	a5,a5
   14f00:	00f56533          	or	a0,a0,a5
   14f04:	00000413          	li	s0,0
   14f08:	f91ff06f          	j	14e98 <__multf3+0x720>
   14f0c:	00061863          	bnez	a2,14f1c <__multf3+0x7a4>
   14f10:	00850793          	addi	a5,a0,8
   14f14:	fb5ff06f          	j	14ec8 <__multf3+0x750>
   14f18:	fe061ce3          	bnez	a2,14f10 <__multf3+0x798>
   14f1c:	03345793          	srli	a5,s0,0x33
   14f20:	0017f793          	andi	a5,a5,1
   14f24:	02078063          	beqz	a5,14f44 <__multf3+0x7cc>
   14f28:	0014e493          	ori	s1,s1,1
   14f2c:	00000413          	li	s0,0
   14f30:	00000513          	li	a0,0
   14f34:	00100713          	li	a4,1
   14f38:	b00682e3          	beqz	a3,14a3c <__multf3+0x2c4>
   14f3c:	0024e493          	ori	s1,s1,2
   14f40:	afdff06f          	j	14a3c <__multf3+0x2c4>
   14f44:	03d41793          	slli	a5,s0,0x3d
   14f48:	00355513          	srli	a0,a0,0x3
   14f4c:	00a7e533          	or	a0,a5,a0
   14f50:	00345413          	srli	s0,s0,0x3
   14f54:	00000713          	li	a4,0
   14f58:	ae0682e3          	beqz	a3,14a3c <__multf3+0x2c4>
   14f5c:	0014f793          	andi	a5,s1,1
   14f60:	ac078ee3          	beqz	a5,14a3c <__multf3+0x2c4>
   14f64:	fd9ff06f          	j	14f3c <__multf3+0x7c4>
   14f68:	0087e533          	or	a0,a5,s0
   14f6c:	02050463          	beqz	a0,14f94 <__multf3+0x81c>
   14f70:	00200793          	li	a5,2
   14f74:	0014e493          	ori	s1,s1,1
   14f78:	02f90e63          	beq	s2,a5,14fb4 <__multf3+0x83c>
   14f7c:	00300793          	li	a5,3
   14f80:	02f90263          	beq	s2,a5,14fa4 <__multf3+0x82c>
   14f84:	00100793          	li	a5,1
   14f88:	00091463          	bnez	s2,14f90 <__multf3+0x818>
   14f8c:	00500793          	li	a5,5
   14f90:	0037d513          	srli	a0,a5,0x3
   14f94:	0024e493          	ori	s1,s1,2
   14f98:	00000413          	li	s0,0
   14f9c:	00000713          	li	a4,0
   14fa0:	a9dff06f          	j	14a3c <__multf3+0x2c4>
   14fa4:	00900793          	li	a5,9
   14fa8:	fe0604e3          	beqz	a2,14f90 <__multf3+0x818>
   14fac:	00100793          	li	a5,1
   14fb0:	fe1ff06f          	j	14f90 <__multf3+0x818>
   14fb4:	00900793          	li	a5,9
   14fb8:	fc061ce3          	bnez	a2,14f90 <__multf3+0x818>
   14fbc:	ff1ff06f          	j	14fac <__multf3+0x834>
   14fc0:	00008737          	lui	a4,0x8
   14fc4:	00000413          	li	s0,0
   14fc8:	00000513          	li	a0,0
   14fcc:	fff70713          	addi	a4,a4,-1 # 7fff <exit-0x8121>
   14fd0:	a6dff06f          	j	14a3c <__multf3+0x2c4>
   14fd4:	00100413          	li	s0,1
   14fd8:	00008737          	lui	a4,0x8
   14fdc:	02f41413          	slli	s0,s0,0x2f
   14fe0:	00000513          	li	a0,0
   14fe4:	fff70713          	addi	a4,a4,-1 # 7fff <exit-0x8121>
   14fe8:	00000613          	li	a2,0
   14fec:	a51ff06f          	j	14a3c <__multf3+0x2c4>

0000000000014ff0 <__subtf3>:
   14ff0:	fd010113          	addi	sp,sp,-48
   14ff4:	02113423          	sd	ra,40(sp)
   14ff8:	02813023          	sd	s0,32(sp)
   14ffc:	00913c23          	sd	s1,24(sp)
   15000:	01213823          	sd	s2,16(sp)
   15004:	01313423          	sd	s3,8(sp)
   15008:	01413023          	sd	s4,0(sp)
   1500c:	00202473          	frrm	s0
   15010:	fff00713          	li	a4,-1
   15014:	01075713          	srli	a4,a4,0x10
   15018:	00008837          	lui	a6,0x8
   1501c:	0305d893          	srli	a7,a1,0x30
   15020:	fff80813          	addi	a6,a6,-1 # 7fff <exit-0x8121>
   15024:	0306de93          	srli	t4,a3,0x30
   15028:	03f5d493          	srli	s1,a1,0x3f
   1502c:	03f6d313          	srli	t1,a3,0x3f
   15030:	00e5f5b3          	and	a1,a1,a4
   15034:	00e6f6b3          	and	a3,a3,a4
   15038:	00359593          	slli	a1,a1,0x3
   1503c:	03d55793          	srli	a5,a0,0x3d
   15040:	03d65713          	srli	a4,a2,0x3d
   15044:	0108f8b3          	and	a7,a7,a6
   15048:	010efeb3          	and	t4,t4,a6
   1504c:	00369693          	slli	a3,a3,0x3
   15050:	00b7e7b3          	or	a5,a5,a1
   15054:	0004041b          	sext.w	s0,s0
   15058:	00088993          	mv	s3,a7
   1505c:	00351513          	slli	a0,a0,0x3
   15060:	000e8593          	mv	a1,t4
   15064:	00d76733          	or	a4,a4,a3
   15068:	00361613          	slli	a2,a2,0x3
   1506c:	010e9663          	bne	t4,a6,15078 <__subtf3+0x88>
   15070:	00c766b3          	or	a3,a4,a2
   15074:	00069463          	bnez	a3,1507c <__subtf3+0x8c>
   15078:	00134313          	xori	t1,t1,1
   1507c:	41d886bb          	subw	a3,a7,t4
   15080:	00008e37          	lui	t3,0x8
   15084:	0006881b          	sext.w	a6,a3
   15088:	fffe0f13          	addi	t5,t3,-1 # 7fff <exit-0x8121>
   1508c:	4c931663          	bne	t1,s1,15558 <__subtf3+0x568>
   15090:	13005c63          	blez	a6,151c8 <__subtf3+0x1d8>
   15094:	0a0e9663          	bnez	t4,15140 <__subtf3+0x150>
   15098:	00c765b3          	or	a1,a4,a2
   1509c:	00059a63          	bnez	a1,150b0 <__subtf3+0xc0>
   150a0:	09e88063          	beq	a7,t5,15120 <__subtf3+0x130>
   150a4:	00078713          	mv	a4,a5
   150a8:	00088593          	mv	a1,a7
   150ac:	1010006f          	j	159ac <__subtf3+0x9bc>
   150b0:	fff6881b          	addiw	a6,a3,-1
   150b4:	06081463          	bnez	a6,1511c <__subtf3+0x12c>
   150b8:	00a60633          	add	a2,a2,a0
   150bc:	00a63533          	sltu	a0,a2,a0
   150c0:	00f70733          	add	a4,a4,a5
   150c4:	00a70733          	add	a4,a4,a0
   150c8:	00088593          	mv	a1,a7
   150cc:	00060513          	mv	a0,a2
   150d0:	03375793          	srli	a5,a4,0x33
   150d4:	0017f793          	andi	a5,a5,1
   150d8:	0c078ae3          	beqz	a5,159ac <__subtf3+0x9bc>
   150dc:	000086b7          	lui	a3,0x8
   150e0:	00158593          	addi	a1,a1,1
   150e4:	fff68793          	addi	a5,a3,-1 # 7fff <exit-0x8121>
   150e8:	42f58e63          	beq	a1,a5,15524 <__subtf3+0x534>
   150ec:	fff00793          	li	a5,-1
   150f0:	03379793          	slli	a5,a5,0x33
   150f4:	fff78793          	addi	a5,a5,-1
   150f8:	00f777b3          	and	a5,a4,a5
   150fc:	00155713          	srli	a4,a0,0x1
   15100:	00157513          	andi	a0,a0,1
   15104:	00a76533          	or	a0,a4,a0
   15108:	03f79713          	slli	a4,a5,0x3f
   1510c:	00a76533          	or	a0,a4,a0
   15110:	0017d793          	srli	a5,a5,0x1
   15114:	00000813          	li	a6,0
   15118:	3900006f          	j	154a8 <__subtf3+0x4b8>
   1511c:	03e89a63          	bne	a7,t5,15150 <__subtf3+0x160>
   15120:	00a7e733          	or	a4,a5,a0
   15124:	100704e3          	beqz	a4,15a2c <__subtf3+0xa3c>
   15128:	0327d713          	srli	a4,a5,0x32
   1512c:	00177713          	andi	a4,a4,1
   15130:	00088593          	mv	a1,a7
   15134:	00000813          	li	a6,0
   15138:	36071863          	bnez	a4,154a8 <__subtf3+0x4b8>
   1513c:	0c00006f          	j	151fc <__subtf3+0x20c>
   15140:	ffe880e3          	beq	a7,t5,15120 <__subtf3+0x130>
   15144:	00100693          	li	a3,1
   15148:	03369693          	slli	a3,a3,0x33
   1514c:	00d76733          	or	a4,a4,a3
   15150:	07400693          	li	a3,116
   15154:	0706c463          	blt	a3,a6,151bc <__subtf3+0x1cc>
   15158:	03f00693          	li	a3,63
   1515c:	0306c663          	blt	a3,a6,15188 <__subtf3+0x198>
   15160:	04000593          	li	a1,64
   15164:	410585bb          	subw	a1,a1,a6
   15168:	00b716b3          	sll	a3,a4,a1
   1516c:	01065333          	srl	t1,a2,a6
   15170:	00b61633          	sll	a2,a2,a1
   15174:	0066e6b3          	or	a3,a3,t1
   15178:	00c03633          	snez	a2,a2
   1517c:	00c6e633          	or	a2,a3,a2
   15180:	01075733          	srl	a4,a4,a6
   15184:	f35ff06f          	j	150b8 <__subtf3+0xc8>
   15188:	fc08069b          	addiw	a3,a6,-64
   1518c:	04000313          	li	t1,64
   15190:	00d756b3          	srl	a3,a4,a3
   15194:	00000593          	li	a1,0
   15198:	00680863          	beq	a6,t1,151a8 <__subtf3+0x1b8>
   1519c:	08000593          	li	a1,128
   151a0:	410585bb          	subw	a1,a1,a6
   151a4:	00b715b3          	sll	a1,a4,a1
   151a8:	00c5e633          	or	a2,a1,a2
   151ac:	00c03633          	snez	a2,a2
   151b0:	00c6e633          	or	a2,a3,a2
   151b4:	00000713          	li	a4,0
   151b8:	f01ff06f          	j	150b8 <__subtf3+0xc8>
   151bc:	00c76633          	or	a2,a4,a2
   151c0:	00c03633          	snez	a2,a2
   151c4:	ff1ff06f          	j	151b4 <__subtf3+0x1c4>
   151c8:	12080063          	beqz	a6,152e8 <__subtf3+0x2f8>
   151cc:	08089063          	bnez	a7,1524c <__subtf3+0x25c>
   151d0:	00a7e833          	or	a6,a5,a0
   151d4:	02081863          	bnez	a6,15204 <__subtf3+0x214>
   151d8:	00060513          	mv	a0,a2
   151dc:	7dee9863          	bne	t4,t5,159ac <__subtf3+0x9bc>
   151e0:	00c76533          	or	a0,a4,a2
   151e4:	020508e3          	beqz	a0,15a14 <__subtf3+0xa24>
   151e8:	03275793          	srli	a5,a4,0x32
   151ec:	0017f793          	andi	a5,a5,1
   151f0:	020798e3          	bnez	a5,15a20 <__subtf3+0xa30>
   151f4:	00070793          	mv	a5,a4
   151f8:	00060513          	mv	a0,a2
   151fc:	01000693          	li	a3,16
   15200:	1440006f          	j	15344 <__subtf3+0x354>
   15204:	fff6c693          	not	a3,a3
   15208:	0006869b          	sext.w	a3,a3
   1520c:	00069c63          	bnez	a3,15224 <__subtf3+0x234>
   15210:	00c50533          	add	a0,a0,a2
   15214:	00e78733          	add	a4,a5,a4
   15218:	00c53633          	sltu	a2,a0,a2
   1521c:	00c70733          	add	a4,a4,a2
   15220:	eb1ff06f          	j	150d0 <__subtf3+0xe0>
   15224:	03ee9e63          	bne	t4,t5,15260 <__subtf3+0x270>
   15228:	00c76533          	or	a0,a4,a2
   1522c:	7e050463          	beqz	a0,15a14 <__subtf3+0xa24>
   15230:	03275793          	srli	a5,a4,0x32
   15234:	0017f793          	andi	a5,a5,1
   15238:	7e079463          	bnez	a5,15a20 <__subtf3+0xa30>
   1523c:	00070793          	mv	a5,a4
   15240:	00060513          	mv	a0,a2
   15244:	00000813          	li	a6,0
   15248:	fb5ff06f          	j	151fc <__subtf3+0x20c>
   1524c:	fdee8ee3          	beq	t4,t5,15228 <__subtf3+0x238>
   15250:	00100813          	li	a6,1
   15254:	03381813          	slli	a6,a6,0x33
   15258:	40d006bb          	negw	a3,a3
   1525c:	0107e7b3          	or	a5,a5,a6
   15260:	07400813          	li	a6,116
   15264:	06d84c63          	blt	a6,a3,152dc <__subtf3+0x2ec>
   15268:	03f00813          	li	a6,63
   1526c:	02d84e63          	blt	a6,a3,152a8 <__subtf3+0x2b8>
   15270:	04000893          	li	a7,64
   15274:	40d888bb          	subw	a7,a7,a3
   15278:	00d55333          	srl	t1,a0,a3
   1527c:	01179833          	sll	a6,a5,a7
   15280:	01151533          	sll	a0,a0,a7
   15284:	00686833          	or	a6,a6,t1
   15288:	00a03533          	snez	a0,a0
   1528c:	00a86533          	or	a0,a6,a0
   15290:	00d7d6b3          	srl	a3,a5,a3
   15294:	00c50533          	add	a0,a0,a2
   15298:	00e686b3          	add	a3,a3,a4
   1529c:	00c53633          	sltu	a2,a0,a2
   152a0:	00c68733          	add	a4,a3,a2
   152a4:	e2dff06f          	j	150d0 <__subtf3+0xe0>
   152a8:	fc06881b          	addiw	a6,a3,-64
   152ac:	04000313          	li	t1,64
   152b0:	0107d833          	srl	a6,a5,a6
   152b4:	00000893          	li	a7,0
   152b8:	00668863          	beq	a3,t1,152c8 <__subtf3+0x2d8>
   152bc:	08000893          	li	a7,128
   152c0:	40d886bb          	subw	a3,a7,a3
   152c4:	00d798b3          	sll	a7,a5,a3
   152c8:	00a8e533          	or	a0,a7,a0
   152cc:	00a03533          	snez	a0,a0
   152d0:	00a86533          	or	a0,a6,a0
   152d4:	00000693          	li	a3,0
   152d8:	fbdff06f          	j	15294 <__subtf3+0x2a4>
   152dc:	00a7e533          	or	a0,a5,a0
   152e0:	00a03533          	snez	a0,a0
   152e4:	ff1ff06f          	j	152d4 <__subtf3+0x2e4>
   152e8:	00188593          	addi	a1,a7,1
   152ec:	ffee0693          	addi	a3,t3,-2
   152f0:	00d5f333          	and	t1,a1,a3
   152f4:	18031863          	bnez	t1,15484 <__subtf3+0x494>
   152f8:	00a7e5b3          	or	a1,a5,a0
   152fc:	06089263          	bnez	a7,15360 <__subtf3+0x370>
   15300:	6c058263          	beqz	a1,159c4 <__subtf3+0x9d4>
   15304:	00c766b3          	or	a3,a4,a2
   15308:	48068263          	beqz	a3,1578c <__subtf3+0x79c>
   1530c:	00c50633          	add	a2,a0,a2
   15310:	00e787b3          	add	a5,a5,a4
   15314:	00a63533          	sltu	a0,a2,a0
   15318:	00a787b3          	add	a5,a5,a0
   1531c:	0337d713          	srli	a4,a5,0x33
   15320:	00177713          	andi	a4,a4,1
   15324:	6a070263          	beqz	a4,159c8 <__subtf3+0x9d8>
   15328:	fff00713          	li	a4,-1
   1532c:	03371713          	slli	a4,a4,0x33
   15330:	fff70713          	addi	a4,a4,-1
   15334:	00e7f7b3          	and	a5,a5,a4
   15338:	00060513          	mv	a0,a2
   1533c:	00000693          	li	a3,0
   15340:	00100593          	li	a1,1
   15344:	00757713          	andi	a4,a0,7
   15348:	1a071263          	bnez	a4,154ec <__subtf3+0x4fc>
   1534c:	08080663          	beqz	a6,153d8 <__subtf3+0x3e8>
   15350:	0016f713          	andi	a4,a3,1
   15354:	08070263          	beqz	a4,153d8 <__subtf3+0x3e8>
   15358:	0026e693          	ori	a3,a3,2
   1535c:	07c0006f          	j	153d8 <__subtf3+0x3e8>
   15360:	03e89c63          	bne	a7,t5,15398 <__subtf3+0x3a8>
   15364:	78058063          	beqz	a1,15ae4 <__subtf3+0xaf4>
   15368:	0327d693          	srli	a3,a5,0x32
   1536c:	0016f693          	andi	a3,a3,1
   15370:	0016b693          	seqz	a3,a3
   15374:	00469693          	slli	a3,a3,0x4
   15378:	051e9063          	bne	t4,a7,153b8 <__subtf3+0x3c8>
   1537c:	00c768b3          	or	a7,a4,a2
   15380:	02088063          	beqz	a7,153a0 <__subtf3+0x3b0>
   15384:	03275893          	srli	a7,a4,0x32
   15388:	0018f893          	andi	a7,a7,1
   1538c:	00089a63          	bnez	a7,153a0 <__subtf3+0x3b0>
   15390:	01000693          	li	a3,16
   15394:	00c0006f          	j	153a0 <__subtf3+0x3b0>
   15398:	00000693          	li	a3,0
   1539c:	ffee80e3          	beq	t4,t5,1537c <__subtf3+0x38c>
   153a0:	00059c63          	bnez	a1,153b8 <__subtf3+0x3c8>
   153a4:	00070793          	mv	a5,a4
   153a8:	00060513          	mv	a0,a2
   153ac:	000085b7          	lui	a1,0x8
   153b0:	fff58593          	addi	a1,a1,-1 # 7fff <exit-0x8121>
   153b4:	f91ff06f          	j	15344 <__subtf3+0x354>
   153b8:	00c76633          	or	a2,a4,a2
   153bc:	fe0608e3          	beqz	a2,153ac <__subtf3+0x3bc>
   153c0:	00100793          	li	a5,1
   153c4:	000085b7          	lui	a1,0x8
   153c8:	00000493          	li	s1,0
   153cc:	03279793          	slli	a5,a5,0x32
   153d0:	00000513          	li	a0,0
   153d4:	fff58593          	addi	a1,a1,-1 # 7fff <exit-0x8121>
   153d8:	0337d713          	srli	a4,a5,0x33
   153dc:	00177713          	andi	a4,a4,1
   153e0:	02070263          	beqz	a4,15404 <__subtf3+0x414>
   153e4:	00008737          	lui	a4,0x8
   153e8:	00158593          	addi	a1,a1,1
   153ec:	fff70613          	addi	a2,a4,-1 # 7fff <exit-0x8121>
   153f0:	6ac58863          	beq	a1,a2,15aa0 <__subtf3+0xab0>
   153f4:	fff00713          	li	a4,-1
   153f8:	03371713          	slli	a4,a4,0x33
   153fc:	fff70713          	addi	a4,a4,-1
   15400:	00e7f7b3          	and	a5,a5,a4
   15404:	00355713          	srli	a4,a0,0x3
   15408:	03d79513          	slli	a0,a5,0x3d
   1540c:	00e56533          	or	a0,a0,a4
   15410:	00008737          	lui	a4,0x8
   15414:	fff70713          	addi	a4,a4,-1 # 7fff <exit-0x8121>
   15418:	0037d793          	srli	a5,a5,0x3
   1541c:	02e59063          	bne	a1,a4,1543c <__subtf3+0x44c>
   15420:	00f56533          	or	a0,a0,a5
   15424:	00000793          	li	a5,0
   15428:	00050a63          	beqz	a0,1543c <__subtf3+0x44c>
   1542c:	00100793          	li	a5,1
   15430:	02f79793          	slli	a5,a5,0x2f
   15434:	00000513          	li	a0,0
   15438:	00000493          	li	s1,0
   1543c:	03159593          	slli	a1,a1,0x31
   15440:	00f4949b          	slliw	s1,s1,0xf
   15444:	0315d593          	srli	a1,a1,0x31
   15448:	0095e5b3          	or	a1,a1,s1
   1544c:	01079793          	slli	a5,a5,0x10
   15450:	03059493          	slli	s1,a1,0x30
   15454:	0107d593          	srli	a1,a5,0x10
   15458:	0095e5b3          	or	a1,a1,s1
   1545c:	00068463          	beqz	a3,15464 <__subtf3+0x474>
   15460:	0016a073          	csrs	fflags,a3
   15464:	02813083          	ld	ra,40(sp)
   15468:	02013403          	ld	s0,32(sp)
   1546c:	01813483          	ld	s1,24(sp)
   15470:	01013903          	ld	s2,16(sp)
   15474:	00813983          	ld	s3,8(sp)
   15478:	00013a03          	ld	s4,0(sp)
   1547c:	03010113          	addi	sp,sp,48
   15480:	00008067          	ret
   15484:	03e58663          	beq	a1,t5,154b0 <__subtf3+0x4c0>
   15488:	00c50633          	add	a2,a0,a2
   1548c:	00a63533          	sltu	a0,a2,a0
   15490:	00e787b3          	add	a5,a5,a4
   15494:	00a787b3          	add	a5,a5,a0
   15498:	03f79513          	slli	a0,a5,0x3f
   1549c:	00165613          	srli	a2,a2,0x1
   154a0:	00c56533          	or	a0,a0,a2
   154a4:	0017d793          	srli	a5,a5,0x1
   154a8:	00000693          	li	a3,0
   154ac:	e99ff06f          	j	15344 <__subtf3+0x354>
   154b0:	00040863          	beqz	s0,154c0 <__subtf3+0x4d0>
   154b4:	00300793          	li	a5,3
   154b8:	00f41c63          	bne	s0,a5,154d0 <__subtf3+0x4e0>
   154bc:	02049063          	bnez	s1,154dc <__subtf3+0x4ec>
   154c0:	00000793          	li	a5,0
   154c4:	00000513          	li	a0,0
   154c8:	00500693          	li	a3,5
   154cc:	f0dff06f          	j	153d8 <__subtf3+0x3e8>
   154d0:	00200793          	li	a5,2
   154d4:	00f41463          	bne	s0,a5,154dc <__subtf3+0x4ec>
   154d8:	fe0494e3          	bnez	s1,154c0 <__subtf3+0x4d0>
   154dc:	fff00793          	li	a5,-1
   154e0:	fff00513          	li	a0,-1
   154e4:	00068593          	mv	a1,a3
   154e8:	00500693          	li	a3,5
   154ec:	00200713          	li	a4,2
   154f0:	0016e693          	ori	a3,a3,1
   154f4:	5ae40063          	beq	s0,a4,15a94 <__subtf3+0xaa4>
   154f8:	00300713          	li	a4,3
   154fc:	58e40663          	beq	s0,a4,15a88 <__subtf3+0xa98>
   15500:	58041c63          	bnez	s0,15a98 <__subtf3+0xaa8>
   15504:	00f57713          	andi	a4,a0,15
   15508:	00400613          	li	a2,4
   1550c:	58c70663          	beq	a4,a2,15a98 <__subtf3+0xaa8>
   15510:	00450713          	addi	a4,a0,4
   15514:	00a73533          	sltu	a0,a4,a0
   15518:	00a787b3          	add	a5,a5,a0
   1551c:	00070513          	mv	a0,a4
   15520:	5780006f          	j	15a98 <__subtf3+0xaa8>
   15524:	f8040ee3          	beqz	s0,154c0 <__subtf3+0x4d0>
   15528:	00300793          	li	a5,3
   1552c:	00f41e63          	bne	s0,a5,15548 <__subtf3+0x558>
   15530:	f80488e3          	beqz	s1,154c0 <__subtf3+0x4d0>
   15534:	fff00793          	li	a5,-1
   15538:	fff00513          	li	a0,-1
   1553c:	ffe68593          	addi	a1,a3,-2
   15540:	00000813          	li	a6,0
   15544:	fa5ff06f          	j	154e8 <__subtf3+0x4f8>
   15548:	00200793          	li	a5,2
   1554c:	fef414e3          	bne	s0,a5,15534 <__subtf3+0x544>
   15550:	f60498e3          	bnez	s1,154c0 <__subtf3+0x4d0>
   15554:	fe1ff06f          	j	15534 <__subtf3+0x544>
   15558:	0f005063          	blez	a6,15638 <__subtf3+0x648>
   1555c:	080e9463          	bnez	t4,155e4 <__subtf3+0x5f4>
   15560:	00c765b3          	or	a1,a4,a2
   15564:	b2058ee3          	beqz	a1,150a0 <__subtf3+0xb0>
   15568:	fff6881b          	addiw	a6,a3,-1
   1556c:	02081e63          	bnez	a6,155a8 <__subtf3+0x5b8>
   15570:	40c50633          	sub	a2,a0,a2
   15574:	00c53533          	sltu	a0,a0,a2
   15578:	40e78733          	sub	a4,a5,a4
   1557c:	40a70733          	sub	a4,a4,a0
   15580:	00088593          	mv	a1,a7
   15584:	00060513          	mv	a0,a2
   15588:	03375793          	srli	a5,a4,0x33
   1558c:	0017f793          	andi	a5,a5,1
   15590:	40078e63          	beqz	a5,159ac <__subtf3+0x9bc>
   15594:	00d71713          	slli	a4,a4,0xd
   15598:	00d75913          	srli	s2,a4,0xd
   1559c:	00050a13          	mv	s4,a0
   155a0:	00058993          	mv	s3,a1
   155a4:	3240006f          	j	158c8 <__subtf3+0x8d8>
   155a8:	b7e88ce3          	beq	a7,t5,15120 <__subtf3+0x130>
   155ac:	07400693          	li	a3,116
   155b0:	0706ce63          	blt	a3,a6,1562c <__subtf3+0x63c>
   155b4:	03f00693          	li	a3,63
   155b8:	0506c063          	blt	a3,a6,155f8 <__subtf3+0x608>
   155bc:	04000593          	li	a1,64
   155c0:	410585bb          	subw	a1,a1,a6
   155c4:	00b716b3          	sll	a3,a4,a1
   155c8:	01065333          	srl	t1,a2,a6
   155cc:	00b61633          	sll	a2,a2,a1
   155d0:	0066e6b3          	or	a3,a3,t1
   155d4:	00c03633          	snez	a2,a2
   155d8:	00c6e633          	or	a2,a3,a2
   155dc:	01075733          	srl	a4,a4,a6
   155e0:	f91ff06f          	j	15570 <__subtf3+0x580>
   155e4:	b3e88ee3          	beq	a7,t5,15120 <__subtf3+0x130>
   155e8:	00100693          	li	a3,1
   155ec:	03369693          	slli	a3,a3,0x33
   155f0:	00d76733          	or	a4,a4,a3
   155f4:	fb9ff06f          	j	155ac <__subtf3+0x5bc>
   155f8:	fc08069b          	addiw	a3,a6,-64
   155fc:	04000313          	li	t1,64
   15600:	00d756b3          	srl	a3,a4,a3
   15604:	00000593          	li	a1,0
   15608:	00680863          	beq	a6,t1,15618 <__subtf3+0x628>
   1560c:	08000593          	li	a1,128
   15610:	410585bb          	subw	a1,a1,a6
   15614:	00b715b3          	sll	a1,a4,a1
   15618:	00c5e633          	or	a2,a1,a2
   1561c:	00c03633          	snez	a2,a2
   15620:	00c6e633          	or	a2,a3,a2
   15624:	00000713          	li	a4,0
   15628:	f49ff06f          	j	15570 <__subtf3+0x580>
   1562c:	00c76633          	or	a2,a4,a2
   15630:	00c03633          	snez	a2,a2
   15634:	ff1ff06f          	j	15624 <__subtf3+0x634>
   15638:	12080063          	beqz	a6,15758 <__subtf3+0x768>
   1563c:	08089063          	bnez	a7,156bc <__subtf3+0x6cc>
   15640:	00a7e833          	or	a6,a5,a0
   15644:	02081663          	bnez	a6,15670 <__subtf3+0x680>
   15648:	37ee9863          	bne	t4,t5,159b8 <__subtf3+0x9c8>
   1564c:	00c76533          	or	a0,a4,a2
   15650:	3e050663          	beqz	a0,15a3c <__subtf3+0xa4c>
   15654:	03275793          	srli	a5,a4,0x32
   15658:	0017f793          	andi	a5,a5,1
   1565c:	3e079663          	bnez	a5,15a48 <__subtf3+0xa58>
   15660:	00070793          	mv	a5,a4
   15664:	00060513          	mv	a0,a2
   15668:	00030493          	mv	s1,t1
   1566c:	b91ff06f          	j	151fc <__subtf3+0x20c>
   15670:	fff6c693          	not	a3,a3
   15674:	0006869b          	sext.w	a3,a3
   15678:	00069e63          	bnez	a3,15694 <__subtf3+0x6a4>
   1567c:	40a60533          	sub	a0,a2,a0
   15680:	40f70733          	sub	a4,a4,a5
   15684:	00a63633          	sltu	a2,a2,a0
   15688:	40c70733          	sub	a4,a4,a2
   1568c:	00030493          	mv	s1,t1
   15690:	ef9ff06f          	j	15588 <__subtf3+0x598>
   15694:	03ee9e63          	bne	t4,t5,156d0 <__subtf3+0x6e0>
   15698:	00c76533          	or	a0,a4,a2
   1569c:	3a050063          	beqz	a0,15a3c <__subtf3+0xa4c>
   156a0:	03275793          	srli	a5,a4,0x32
   156a4:	0017f793          	andi	a5,a5,1
   156a8:	3a079063          	bnez	a5,15a48 <__subtf3+0xa58>
   156ac:	00070793          	mv	a5,a4
   156b0:	00060513          	mv	a0,a2
   156b4:	00030493          	mv	s1,t1
   156b8:	b8dff06f          	j	15244 <__subtf3+0x254>
   156bc:	fdee8ee3          	beq	t4,t5,15698 <__subtf3+0x6a8>
   156c0:	00100813          	li	a6,1
   156c4:	03381813          	slli	a6,a6,0x33
   156c8:	40d006bb          	negw	a3,a3
   156cc:	0107e7b3          	or	a5,a5,a6
   156d0:	07400813          	li	a6,116
   156d4:	06d84c63          	blt	a6,a3,1574c <__subtf3+0x75c>
   156d8:	03f00813          	li	a6,63
   156dc:	02d84e63          	blt	a6,a3,15718 <__subtf3+0x728>
   156e0:	04000893          	li	a7,64
   156e4:	40d888bb          	subw	a7,a7,a3
   156e8:	01179833          	sll	a6,a5,a7
   156ec:	00d55e33          	srl	t3,a0,a3
   156f0:	01151533          	sll	a0,a0,a7
   156f4:	01c86833          	or	a6,a6,t3
   156f8:	00a03533          	snez	a0,a0
   156fc:	00a86533          	or	a0,a6,a0
   15700:	00d7d7b3          	srl	a5,a5,a3
   15704:	40a60533          	sub	a0,a2,a0
   15708:	40f707b3          	sub	a5,a4,a5
   1570c:	00a63633          	sltu	a2,a2,a0
   15710:	40c78733          	sub	a4,a5,a2
   15714:	f79ff06f          	j	1568c <__subtf3+0x69c>
   15718:	fc06881b          	addiw	a6,a3,-64
   1571c:	04000e13          	li	t3,64
   15720:	0107d833          	srl	a6,a5,a6
   15724:	00000893          	li	a7,0
   15728:	01c68863          	beq	a3,t3,15738 <__subtf3+0x748>
   1572c:	08000893          	li	a7,128
   15730:	40d886bb          	subw	a3,a7,a3
   15734:	00d798b3          	sll	a7,a5,a3
   15738:	00a8e533          	or	a0,a7,a0
   1573c:	00a03533          	snez	a0,a0
   15740:	00a86533          	or	a0,a6,a0
   15744:	00000793          	li	a5,0
   15748:	fbdff06f          	j	15704 <__subtf3+0x714>
   1574c:	00a7e533          	or	a0,a5,a0
   15750:	00a03533          	snez	a0,a0
   15754:	ff1ff06f          	j	15744 <__subtf3+0x754>
   15758:	00188593          	addi	a1,a7,1
   1575c:	ffee0693          	addi	a3,t3,-2
   15760:	00d5f6b3          	and	a3,a1,a3
   15764:	12069a63          	bnez	a3,15898 <__subtf3+0x8a8>
   15768:	00a7ee33          	or	t3,a5,a0
   1576c:	00c765b3          	or	a1,a4,a2
   15770:	0c089663          	bnez	a7,1583c <__subtf3+0x84c>
   15774:	060e1c63          	bnez	t3,157ec <__subtf3+0x7fc>
   15778:	24059c63          	bnez	a1,159d0 <__subtf3+0x9e0>
   1577c:	ffe40493          	addi	s1,s0,-2
   15780:	0014b493          	seqz	s1,s1
   15784:	00000793          	li	a5,0
   15788:	00000513          	li	a0,0
   1578c:	00f56733          	or	a4,a0,a5
   15790:	2e070463          	beqz	a4,15a78 <__subtf3+0xa88>
   15794:	03f55713          	srli	a4,a0,0x3f
   15798:	00179813          	slli	a6,a5,0x1
   1579c:	00e80833          	add	a6,a6,a4
   157a0:	00151713          	slli	a4,a0,0x1
   157a4:	00777613          	andi	a2,a4,7
   157a8:	00000693          	li	a3,0
   157ac:	02060663          	beqz	a2,157d8 <__subtf3+0x7e8>
   157b0:	00200693          	li	a3,2
   157b4:	24d40a63          	beq	s0,a3,15a08 <__subtf3+0xa18>
   157b8:	00300693          	li	a3,3
   157bc:	22d40663          	beq	s0,a3,159e8 <__subtf3+0x9f8>
   157c0:	00100693          	li	a3,1
   157c4:	00041a63          	bnez	s0,157d8 <__subtf3+0x7e8>
   157c8:	00f77613          	andi	a2,a4,15
   157cc:	00400593          	li	a1,4
   157d0:	ffc73713          	sltiu	a4,a4,-4
   157d4:	22b61063          	bne	a2,a1,159f4 <__subtf3+0xa04>
   157d8:	03485813          	srli	a6,a6,0x34
   157dc:	00184813          	xori	a6,a6,1
   157e0:	00187813          	andi	a6,a6,1
   157e4:	00000593          	li	a1,0
   157e8:	b5dff06f          	j	15344 <__subtf3+0x354>
   157ec:	fa0580e3          	beqz	a1,1578c <__subtf3+0x79c>
   157f0:	40c505b3          	sub	a1,a0,a2
   157f4:	00b53833          	sltu	a6,a0,a1
   157f8:	40e786b3          	sub	a3,a5,a4
   157fc:	410686b3          	sub	a3,a3,a6
   15800:	0336d813          	srli	a6,a3,0x33
   15804:	00187813          	andi	a6,a6,1
   15808:	00080e63          	beqz	a6,15824 <__subtf3+0x834>
   1580c:	40a60533          	sub	a0,a2,a0
   15810:	40f707b3          	sub	a5,a4,a5
   15814:	00a63633          	sltu	a2,a2,a0
   15818:	40c787b3          	sub	a5,a5,a2
   1581c:	00030493          	mv	s1,t1
   15820:	f6dff06f          	j	1578c <__subtf3+0x79c>
   15824:	00d5e533          	or	a0,a1,a3
   15828:	1a051a63          	bnez	a0,159dc <__subtf3+0x9ec>
   1582c:	ffe40493          	addi	s1,s0,-2
   15830:	0014b493          	seqz	s1,s1
   15834:	00000793          	li	a5,0
   15838:	f55ff06f          	j	1578c <__subtf3+0x79c>
   1583c:	03e89a63          	bne	a7,t5,15870 <__subtf3+0x880>
   15840:	280e0c63          	beqz	t3,15ad8 <__subtf3+0xae8>
   15844:	0327d693          	srli	a3,a5,0x32
   15848:	0016f693          	andi	a3,a3,1
   1584c:	0016b693          	seqz	a3,a3
   15850:	00469693          	slli	a3,a3,0x4
   15854:	031e9e63          	bne	t4,a7,15890 <__subtf3+0x8a0>
   15858:	02058063          	beqz	a1,15878 <__subtf3+0x888>
   1585c:	03275893          	srli	a7,a4,0x32
   15860:	0018f893          	andi	a7,a7,1
   15864:	00089a63          	bnez	a7,15878 <__subtf3+0x888>
   15868:	01000693          	li	a3,16
   1586c:	00c0006f          	j	15878 <__subtf3+0x888>
   15870:	00000693          	li	a3,0
   15874:	ffee82e3          	beq	t4,t5,15858 <__subtf3+0x868>
   15878:	000e1c63          	bnez	t3,15890 <__subtf3+0x8a0>
   1587c:	1c058e63          	beqz	a1,15a58 <__subtf3+0xa68>
   15880:	00070793          	mv	a5,a4
   15884:	00060513          	mv	a0,a2
   15888:	00030493          	mv	s1,t1
   1588c:	b21ff06f          	j	153ac <__subtf3+0x3bc>
   15890:	b0058ee3          	beqz	a1,153ac <__subtf3+0x3bc>
   15894:	b2dff06f          	j	153c0 <__subtf3+0x3d0>
   15898:	40c50a33          	sub	s4,a0,a2
   1589c:	014536b3          	sltu	a3,a0,s4
   158a0:	40e78933          	sub	s2,a5,a4
   158a4:	40d90933          	sub	s2,s2,a3
   158a8:	03395693          	srli	a3,s2,0x33
   158ac:	0016f693          	andi	a3,a3,1
   158b0:	08068663          	beqz	a3,1593c <__subtf3+0x94c>
   158b4:	40a60a33          	sub	s4,a2,a0
   158b8:	40f70733          	sub	a4,a4,a5
   158bc:	01463633          	sltu	a2,a2,s4
   158c0:	40c70933          	sub	s2,a4,a2
   158c4:	00030493          	mv	s1,t1
   158c8:	08090063          	beqz	s2,15948 <__subtf3+0x958>
   158cc:	00090513          	mv	a0,s2
   158d0:	5b0000ef          	jal	ra,15e80 <__clzdi2>
   158d4:	0005051b          	sext.w	a0,a0
   158d8:	ff45059b          	addiw	a1,a0,-12
   158dc:	03f00793          	li	a5,63
   158e0:	00058693          	mv	a3,a1
   158e4:	06b7ca63          	blt	a5,a1,15958 <__subtf3+0x968>
   158e8:	04000713          	li	a4,64
   158ec:	40b7073b          	subw	a4,a4,a1
   158f0:	00b91933          	sll	s2,s2,a1
   158f4:	00ea5733          	srl	a4,s4,a4
   158f8:	01276733          	or	a4,a4,s2
   158fc:	00ba1533          	sll	a0,s4,a1
   15900:	0935cc63          	blt	a1,s3,15998 <__subtf3+0x9a8>
   15904:	413686bb          	subw	a3,a3,s3
   15908:	0016879b          	addiw	a5,a3,1
   1590c:	03f00613          	li	a2,63
   15910:	04f64c63          	blt	a2,a5,15968 <__subtf3+0x978>
   15914:	04000693          	li	a3,64
   15918:	40f686bb          	subw	a3,a3,a5
   1591c:	00f555b3          	srl	a1,a0,a5
   15920:	00d71633          	sll	a2,a4,a3
   15924:	00d51533          	sll	a0,a0,a3
   15928:	00b66633          	or	a2,a2,a1
   1592c:	00a03533          	snez	a0,a0
   15930:	00a66533          	or	a0,a2,a0
   15934:	00f757b3          	srl	a5,a4,a5
   15938:	e55ff06f          	j	1578c <__subtf3+0x79c>
   1593c:	012a6533          	or	a0,s4,s2
   15940:	f80514e3          	bnez	a0,158c8 <__subtf3+0x8d8>
   15944:	ee9ff06f          	j	1582c <__subtf3+0x83c>
   15948:	000a0513          	mv	a0,s4
   1594c:	534000ef          	jal	ra,15e80 <__clzdi2>
   15950:	0405051b          	addiw	a0,a0,64
   15954:	f85ff06f          	j	158d8 <__subtf3+0x8e8>
   15958:	fb45071b          	addiw	a4,a0,-76
   1595c:	00ea1733          	sll	a4,s4,a4
   15960:	00000513          	li	a0,0
   15964:	f9dff06f          	j	15900 <__subtf3+0x910>
   15968:	fc16869b          	addiw	a3,a3,-63
   1596c:	04000813          	li	a6,64
   15970:	00d75633          	srl	a2,a4,a3
   15974:	00000693          	li	a3,0
   15978:	01078863          	beq	a5,a6,15988 <__subtf3+0x998>
   1597c:	08000693          	li	a3,128
   15980:	40f686bb          	subw	a3,a3,a5
   15984:	00d716b3          	sll	a3,a4,a3
   15988:	00d56533          	or	a0,a0,a3
   1598c:	00a03533          	snez	a0,a0
   15990:	00a66533          	or	a0,a2,a0
   15994:	ea1ff06f          	j	15834 <__subtf3+0x844>
   15998:	fff00793          	li	a5,-1
   1599c:	03379793          	slli	a5,a5,0x33
   159a0:	fff78793          	addi	a5,a5,-1
   159a4:	40b985b3          	sub	a1,s3,a1
   159a8:	00f77733          	and	a4,a4,a5
   159ac:	00070793          	mv	a5,a4
   159b0:	dc058ee3          	beqz	a1,1578c <__subtf3+0x79c>
   159b4:	f60ff06f          	j	15114 <__subtf3+0x124>
   159b8:	00060513          	mv	a0,a2
   159bc:	00030493          	mv	s1,t1
   159c0:	fedff06f          	j	159ac <__subtf3+0x9bc>
   159c4:	00070793          	mv	a5,a4
   159c8:	00060513          	mv	a0,a2
   159cc:	dc1ff06f          	j	1578c <__subtf3+0x79c>
   159d0:	00070793          	mv	a5,a4
   159d4:	00060513          	mv	a0,a2
   159d8:	e45ff06f          	j	1581c <__subtf3+0x82c>
   159dc:	00068793          	mv	a5,a3
   159e0:	00058513          	mv	a0,a1
   159e4:	da9ff06f          	j	1578c <__subtf3+0x79c>
   159e8:	00100693          	li	a3,1
   159ec:	de0496e3          	bnez	s1,157d8 <__subtf3+0x7e8>
   159f0:	ff873713          	sltiu	a4,a4,-8
   159f4:	00174713          	xori	a4,a4,1
   159f8:	02071713          	slli	a4,a4,0x20
   159fc:	02075713          	srli	a4,a4,0x20
   15a00:	00e80833          	add	a6,a6,a4
   15a04:	dd5ff06f          	j	157d8 <__subtf3+0x7e8>
   15a08:	00100693          	li	a3,1
   15a0c:	dc0486e3          	beqz	s1,157d8 <__subtf3+0x7e8>
   15a10:	fe1ff06f          	j	159f0 <__subtf3+0xa00>
   15a14:	00000793          	li	a5,0
   15a18:	00000693          	li	a3,0
   15a1c:	9bdff06f          	j	153d8 <__subtf3+0x3e8>
   15a20:	00070793          	mv	a5,a4
   15a24:	00060513          	mv	a0,a2
   15a28:	eecff06f          	j	15114 <__subtf3+0x124>
   15a2c:	00000793          	li	a5,0
   15a30:	00000513          	li	a0,0
   15a34:	00088593          	mv	a1,a7
   15a38:	fe1ff06f          	j	15a18 <__subtf3+0xa28>
   15a3c:	00000793          	li	a5,0
   15a40:	00030493          	mv	s1,t1
   15a44:	fd5ff06f          	j	15a18 <__subtf3+0xa28>
   15a48:	00070793          	mv	a5,a4
   15a4c:	00060513          	mv	a0,a2
   15a50:	00030493          	mv	s1,t1
   15a54:	ec0ff06f          	j	15114 <__subtf3+0x124>
   15a58:	00100793          	li	a5,1
   15a5c:	000085b7          	lui	a1,0x8
   15a60:	00000513          	li	a0,0
   15a64:	00000493          	li	s1,0
   15a68:	03279793          	slli	a5,a5,0x32
   15a6c:	fff58593          	addi	a1,a1,-1 # 7fff <exit-0x8121>
   15a70:	01000693          	li	a3,16
   15a74:	965ff06f          	j	153d8 <__subtf3+0x3e8>
   15a78:	00000793          	li	a5,0
   15a7c:	00000513          	li	a0,0
   15a80:	00000593          	li	a1,0
   15a84:	f95ff06f          	j	15a18 <__subtf3+0xa28>
   15a88:	00049863          	bnez	s1,15a98 <__subtf3+0xaa8>
   15a8c:	00850713          	addi	a4,a0,8
   15a90:	a85ff06f          	j	15514 <__subtf3+0x524>
   15a94:	fe049ce3          	bnez	s1,15a8c <__subtf3+0xa9c>
   15a98:	8c0810e3          	bnez	a6,15358 <__subtf3+0x368>
   15a9c:	93dff06f          	j	153d8 <__subtf3+0x3e8>
   15aa0:	00000513          	li	a0,0
   15aa4:	02040463          	beqz	s0,15acc <__subtf3+0xadc>
   15aa8:	00300793          	li	a5,3
   15aac:	00f41a63          	bne	s0,a5,15ac0 <__subtf3+0xad0>
   15ab0:	00048e63          	beqz	s1,15acc <__subtf3+0xadc>
   15ab4:	fff00513          	li	a0,-1
   15ab8:	ffe70593          	addi	a1,a4,-2
   15abc:	0100006f          	j	15acc <__subtf3+0xadc>
   15ac0:	00200793          	li	a5,2
   15ac4:	fef418e3          	bne	s0,a5,15ab4 <__subtf3+0xac4>
   15ac8:	fe0486e3          	beqz	s1,15ab4 <__subtf3+0xac4>
   15acc:	0056e693          	ori	a3,a3,5
   15ad0:	00050793          	mv	a5,a0
   15ad4:	931ff06f          	j	15404 <__subtf3+0x414>
   15ad8:	00000693          	li	a3,0
   15adc:	db1e90e3          	bne	t4,a7,1587c <__subtf3+0x88c>
   15ae0:	d79ff06f          	j	15858 <__subtf3+0x868>
   15ae4:	00000693          	li	a3,0
   15ae8:	8b1e9ee3          	bne	t4,a7,153a4 <__subtf3+0x3b4>
   15aec:	891ff06f          	j	1537c <__subtf3+0x38c>

0000000000015af0 <__fixtfsi>:
   15af0:	00050613          	mv	a2,a0
   15af4:	002027f3          	frrm	a5
   15af8:	00004837          	lui	a6,0x4
   15afc:	00159693          	slli	a3,a1,0x1
   15b00:	01059793          	slli	a5,a1,0x10
   15b04:	0316d693          	srli	a3,a3,0x31
   15b08:	ffe80713          	addi	a4,a6,-2 # 3ffe <exit-0xc122>
   15b0c:	0107d793          	srli	a5,a5,0x10
   15b10:	03f5d593          	srli	a1,a1,0x3f
   15b14:	00d74c63          	blt	a4,a3,15b2c <__fixtfsi+0x3c>
   15b18:	08069c63          	bnez	a3,15bb0 <__fixtfsi+0xc0>
   15b1c:	00f567b3          	or	a5,a0,a5
   15b20:	00000513          	li	a0,0
   15b24:	04079063          	bnez	a5,15b64 <__fixtfsi+0x74>
   15b28:	00008067          	ret
   15b2c:	01d80713          	addi	a4,a6,29
   15b30:	02d75e63          	bge	a4,a3,15b6c <__fixtfsi+0x7c>
   15b34:	80000737          	lui	a4,0x80000
   15b38:	fff74713          	not	a4,a4
   15b3c:	00b7053b          	addw	a0,a4,a1
   15b40:	06058c63          	beqz	a1,15bb8 <__fixtfsi+0xc8>
   15b44:	01e80813          	addi	a6,a6,30
   15b48:	01000713          	li	a4,16
   15b4c:	07069863          	bne	a3,a6,15bbc <__fixtfsi+0xcc>
   15b50:	0117d693          	srli	a3,a5,0x11
   15b54:	06069463          	bnez	a3,15bbc <__fixtfsi+0xcc>
   15b58:	02f79793          	slli	a5,a5,0x2f
   15b5c:	00c7e7b3          	or	a5,a5,a2
   15b60:	04078663          	beqz	a5,15bac <__fixtfsi+0xbc>
   15b64:	00100713          	li	a4,1
   15b68:	0540006f          	j	15bbc <__fixtfsi+0xcc>
   15b6c:	00100513          	li	a0,1
   15b70:	03051513          	slli	a0,a0,0x30
   15b74:	00a7e7b3          	or	a5,a5,a0
   15b78:	ffffc737          	lui	a4,0xffffc
   15b7c:	00004537          	lui	a0,0x4
   15b80:	02f5051b          	addiw	a0,a0,47
   15b84:	0117071b          	addiw	a4,a4,17
   15b88:	00d7073b          	addw	a4,a4,a3
   15b8c:	40d506bb          	subw	a3,a0,a3
   15b90:	00e79733          	sll	a4,a5,a4
   15b94:	00d7d7b3          	srl	a5,a5,a3
   15b98:	00c76733          	or	a4,a4,a2
   15b9c:	0007851b          	sext.w	a0,a5
   15ba0:	00058463          	beqz	a1,15ba8 <__fixtfsi+0xb8>
   15ba4:	40a0053b          	negw	a0,a0
   15ba8:	fa071ee3          	bnez	a4,15b64 <__fixtfsi+0x74>
   15bac:	00008067          	ret
   15bb0:	00000513          	li	a0,0
   15bb4:	fb1ff06f          	j	15b64 <__fixtfsi+0x74>
   15bb8:	01000713          	li	a4,16
   15bbc:	00172073          	csrs	fflags,a4
   15bc0:	fedff06f          	j	15bac <__fixtfsi+0xbc>

0000000000015bc4 <__fixunstfsi>:
   15bc4:	00050693          	mv	a3,a0
   15bc8:	002027f3          	frrm	a5
   15bcc:	00004637          	lui	a2,0x4
   15bd0:	00159713          	slli	a4,a1,0x1
   15bd4:	01059793          	slli	a5,a1,0x10
   15bd8:	03175713          	srli	a4,a4,0x31
   15bdc:	ffe60513          	addi	a0,a2,-2 # 3ffe <exit-0xc122>
   15be0:	0107d793          	srli	a5,a5,0x10
   15be4:	03f5d593          	srli	a1,a1,0x3f
   15be8:	00e54c63          	blt	a0,a4,15c00 <__fixunstfsi+0x3c>
   15bec:	06071c63          	bnez	a4,15c64 <__fixunstfsi+0xa0>
   15bf0:	00f6e7b3          	or	a5,a3,a5
   15bf4:	06079863          	bnez	a5,15c64 <__fixunstfsi+0xa0>
   15bf8:	00000513          	li	a0,0
   15bfc:	00008067          	ret
   15c00:	01e60513          	addi	a0,a2,30
   15c04:	00059463          	bnez	a1,15c0c <__fixunstfsi+0x48>
   15c08:	01f60513          	addi	a0,a2,31
   15c0c:	04a75463          	bge	a4,a0,15c54 <__fixunstfsi+0x90>
   15c10:	06059063          	bnez	a1,15c70 <__fixunstfsi+0xac>
   15c14:	00100613          	li	a2,1
   15c18:	03061613          	slli	a2,a2,0x30
   15c1c:	00c7e7b3          	or	a5,a5,a2
   15c20:	000045b7          	lui	a1,0x4
   15c24:	ffffc637          	lui	a2,0xffffc
   15c28:	02f5859b          	addiw	a1,a1,47
   15c2c:	0116061b          	addiw	a2,a2,17
   15c30:	40e585bb          	subw	a1,a1,a4
   15c34:	00e6073b          	addw	a4,a2,a4
   15c38:	00b7d5b3          	srl	a1,a5,a1
   15c3c:	00e797b3          	sll	a5,a5,a4
   15c40:	00d7e7b3          	or	a5,a5,a3
   15c44:	0005851b          	sext.w	a0,a1
   15c48:	00078c63          	beqz	a5,15c60 <__fixunstfsi+0x9c>
   15c4c:	00100793          	li	a5,1
   15c50:	00c0006f          	j	15c5c <__fixunstfsi+0x98>
   15c54:	fff58513          	addi	a0,a1,-1 # 3fff <exit-0xc121>
   15c58:	01000793          	li	a5,16
   15c5c:	0017a073          	csrs	fflags,a5
   15c60:	00008067          	ret
   15c64:	00100793          	li	a5,1
   15c68:	00000513          	li	a0,0
   15c6c:	ff1ff06f          	j	15c5c <__fixunstfsi+0x98>
   15c70:	01000793          	li	a5,16
   15c74:	ff5ff06f          	j	15c68 <__fixunstfsi+0xa4>

0000000000015c78 <__floatsitf>:
   15c78:	fe010113          	addi	sp,sp,-32
   15c7c:	00113c23          	sd	ra,24(sp)
   15c80:	00813823          	sd	s0,16(sp)
   15c84:	00913423          	sd	s1,8(sp)
   15c88:	06050463          	beqz	a0,15cf0 <__floatsitf+0x78>
   15c8c:	0005079b          	sext.w	a5,a0
   15c90:	03f55493          	srli	s1,a0,0x3f
   15c94:	00055463          	bgez	a0,15c9c <__floatsitf+0x24>
   15c98:	40f007bb          	negw	a5,a5
   15c9c:	02079413          	slli	s0,a5,0x20
   15ca0:	02045413          	srli	s0,s0,0x20
   15ca4:	00040513          	mv	a0,s0
   15ca8:	1d8000ef          	jal	ra,15e80 <__clzdi2>
   15cac:	000045b7          	lui	a1,0x4
   15cb0:	03e5859b          	addiw	a1,a1,62
   15cb4:	ff15079b          	addiw	a5,a0,-15
   15cb8:	40a585bb          	subw	a1,a1,a0
   15cbc:	00f417b3          	sll	a5,s0,a5
   15cc0:	01813083          	ld	ra,24(sp)
   15cc4:	01013403          	ld	s0,16(sp)
   15cc8:	00f4949b          	slliw	s1,s1,0xf
   15ccc:	0095e5b3          	or	a1,a1,s1
   15cd0:	01079793          	slli	a5,a5,0x10
   15cd4:	03059593          	slli	a1,a1,0x30
   15cd8:	0107d793          	srli	a5,a5,0x10
   15cdc:	00813483          	ld	s1,8(sp)
   15ce0:	00000513          	li	a0,0
   15ce4:	00b7e5b3          	or	a1,a5,a1
   15ce8:	02010113          	addi	sp,sp,32
   15cec:	00008067          	ret
   15cf0:	00000793          	li	a5,0
   15cf4:	00000593          	li	a1,0
   15cf8:	00000493          	li	s1,0
   15cfc:	fc5ff06f          	j	15cc0 <__floatsitf+0x48>

0000000000015d00 <__floatunsitf>:
   15d00:	ff010113          	addi	sp,sp,-16
   15d04:	00113423          	sd	ra,8(sp)
   15d08:	00813023          	sd	s0,0(sp)
   15d0c:	04050663          	beqz	a0,15d58 <__floatunsitf+0x58>
   15d10:	02051413          	slli	s0,a0,0x20
   15d14:	02045413          	srli	s0,s0,0x20
   15d18:	00040513          	mv	a0,s0
   15d1c:	164000ef          	jal	ra,15e80 <__clzdi2>
   15d20:	000045b7          	lui	a1,0x4
   15d24:	03e5859b          	addiw	a1,a1,62
   15d28:	40a585bb          	subw	a1,a1,a0
   15d2c:	ff15051b          	addiw	a0,a0,-15
   15d30:	00a41433          	sll	s0,s0,a0
   15d34:	01041413          	slli	s0,s0,0x10
   15d38:	01045413          	srli	s0,s0,0x10
   15d3c:	03059593          	slli	a1,a1,0x30
   15d40:	00813083          	ld	ra,8(sp)
   15d44:	00b465b3          	or	a1,s0,a1
   15d48:	00013403          	ld	s0,0(sp)
   15d4c:	00000513          	li	a0,0
   15d50:	01010113          	addi	sp,sp,16
   15d54:	00008067          	ret
   15d58:	00000413          	li	s0,0
   15d5c:	00000593          	li	a1,0
   15d60:	fd5ff06f          	j	15d34 <__floatunsitf+0x34>

0000000000015d64 <__extenddftf2>:
   15d64:	fe010113          	addi	sp,sp,-32
   15d68:	00113c23          	sd	ra,24(sp)
   15d6c:	00813823          	sd	s0,16(sp)
   15d70:	00913423          	sd	s1,8(sp)
   15d74:	002027f3          	frrm	a5
   15d78:	03455793          	srli	a5,a0,0x34
   15d7c:	7ff7f793          	andi	a5,a5,2047
   15d80:	00178713          	addi	a4,a5,1
   15d84:	00c51413          	slli	s0,a0,0xc
   15d88:	7fe77713          	andi	a4,a4,2046
   15d8c:	00c45413          	srli	s0,s0,0xc
   15d90:	03f55493          	srli	s1,a0,0x3f
   15d94:	02070063          	beqz	a4,15db4 <__extenddftf2+0x50>
   15d98:	00004737          	lui	a4,0x4
   15d9c:	c0070713          	addi	a4,a4,-1024 # 3c00 <exit-0xc520>
   15da0:	00445593          	srli	a1,s0,0x4
   15da4:	00e787b3          	add	a5,a5,a4
   15da8:	03c41413          	slli	s0,s0,0x3c
   15dac:	00000713          	li	a4,0
   15db0:	0880006f          	j	15e38 <__extenddftf2+0xd4>
   15db4:	04079a63          	bnez	a5,15e08 <__extenddftf2+0xa4>
   15db8:	00000593          	li	a1,0
   15dbc:	fe0408e3          	beqz	s0,15dac <__extenddftf2+0x48>
   15dc0:	00040513          	mv	a0,s0
   15dc4:	0bc000ef          	jal	ra,15e80 <__clzdi2>
   15dc8:	0005071b          	sext.w	a4,a0
   15dcc:	00e00793          	li	a5,14
   15dd0:	02e7c463          	blt	a5,a4,15df8 <__extenddftf2+0x94>
   15dd4:	00f00593          	li	a1,15
   15dd8:	40a585bb          	subw	a1,a1,a0
   15ddc:	0315079b          	addiw	a5,a0,49
   15de0:	00b455b3          	srl	a1,s0,a1
   15de4:	00f41433          	sll	s0,s0,a5
   15de8:	00004737          	lui	a4,0x4
   15dec:	c0c7071b          	addiw	a4,a4,-1012
   15df0:	40a707bb          	subw	a5,a4,a0
   15df4:	fb9ff06f          	j	15dac <__extenddftf2+0x48>
   15df8:	ff15059b          	addiw	a1,a0,-15
   15dfc:	00b415b3          	sll	a1,s0,a1
   15e00:	00000413          	li	s0,0
   15e04:	fe5ff06f          	j	15de8 <__extenddftf2+0x84>
   15e08:	06040463          	beqz	s0,15e70 <__extenddftf2+0x10c>
   15e0c:	00100793          	li	a5,1
   15e10:	03379713          	slli	a4,a5,0x33
   15e14:	00e47733          	and	a4,s0,a4
   15e18:	00445593          	srli	a1,s0,0x4
   15e1c:	02f79793          	slli	a5,a5,0x2f
   15e20:	00173713          	seqz	a4,a4
   15e24:	00f5e5b3          	or	a1,a1,a5
   15e28:	000087b7          	lui	a5,0x8
   15e2c:	00471713          	slli	a4,a4,0x4
   15e30:	03c41413          	slli	s0,s0,0x3c
   15e34:	fff78793          	addi	a5,a5,-1 # 7fff <exit-0x8121>
   15e38:	00f4951b          	slliw	a0,s1,0xf
   15e3c:	01059593          	slli	a1,a1,0x10
   15e40:	00a7e7b3          	or	a5,a5,a0
   15e44:	03079793          	slli	a5,a5,0x30
   15e48:	0105d593          	srli	a1,a1,0x10
   15e4c:	00f5e5b3          	or	a1,a1,a5
   15e50:	00070463          	beqz	a4,15e58 <__extenddftf2+0xf4>
   15e54:	00172073          	csrs	fflags,a4
   15e58:	01813083          	ld	ra,24(sp)
   15e5c:	00040513          	mv	a0,s0
   15e60:	01013403          	ld	s0,16(sp)
   15e64:	00813483          	ld	s1,8(sp)
   15e68:	02010113          	addi	sp,sp,32
   15e6c:	00008067          	ret
   15e70:	000087b7          	lui	a5,0x8
   15e74:	00000593          	li	a1,0
   15e78:	fff78793          	addi	a5,a5,-1 # 7fff <exit-0x8121>
   15e7c:	f31ff06f          	j	15dac <__extenddftf2+0x48>

0000000000015e80 <__clzdi2>:
   15e80:	03800793          	li	a5,56
   15e84:	00f55733          	srl	a4,a0,a5
   15e88:	0ff77713          	andi	a4,a4,255
   15e8c:	00071663          	bnez	a4,15e98 <__clzdi2+0x18>
   15e90:	ff878793          	addi	a5,a5,-8
   15e94:	fe0798e3          	bnez	a5,15e84 <__clzdi2+0x4>
   15e98:	04000713          	li	a4,64
   15e9c:	40f70733          	sub	a4,a4,a5
   15ea0:	00f55533          	srl	a0,a0,a5
   15ea4:	00002797          	auipc	a5,0x2
   15ea8:	25c7b783          	ld	a5,604(a5) # 18100 <_GLOBAL_OFFSET_TABLE_+0x8>
   15eac:	00a78533          	add	a0,a5,a0
   15eb0:	00054503          	lbu	a0,0(a0) # 4000 <exit-0xc120>
   15eb4:	40a7053b          	subw	a0,a4,a0
   15eb8:	00008067          	ret
