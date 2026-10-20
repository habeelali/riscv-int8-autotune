"""Deterministic hardware selection; training inputs 0/1, validation inputs 2/3.

An alternative must beat the incumbent by >=2% on BOTH training input medians.
This guards against marginal wins, not a statistical confidence interval.
Shared graph operators and person kernels retain incumbent Rust (backend 3).
"""
import math
import statistics as st
BACKENDS=['esp_nn','mu_scalar','mu_fixed','rust','tile8_b2p4','tile16_b2p4','tile16_b2p4_block_sram','tile16_b4p2','selected_rust','selected_hybrid']
LEARNED=[list(range(27))+[28],[0,1,2,4,5,6,8,9,10,14]]
OPERATORS=[31,16]

def select(profiles):
    assert len(profiles)==96
    choices=[];evidence=[]
    for kind in range(2):
        for m,n in enumerate(OPERATORS):
            for layer in range(n):
                def costs(b):
                    values=[]
                    for sample in [0,1]:
                        rows=[r for r in profiles if (r['model'],r['backend'],r['sample'])==(m,b,sample)]
                        assert len(rows)==3
                        values.append(st.median(r['layers'][layer][0] for r in rows))
                    return values
                old=costs(3);winner=3;winner_cost=1.
                # Keep common C operations and the proven person Rust path.
                eligible=range(0 if kind else 4,8) if layer in LEARNED[m] else []
                for b in eligible:
                    if b==3 or (m==0 and b>=3):continue
                    values=costs(b)
                    if all(v<=.98*o for v,o in zip(values,old)):
                        cost=math.sqrt(math.prod(v/o for v,o in zip(values,old)))
                        if cost<winner_cost:winner=b;winner_cost=cost
                choices.append(winner)
                evidence.append(dict(kind=kind,model=m,layer=layer,backend=winner,training_ratio_to_incumbent=winner_cost))
    return dict(schema=4,choices=choices,training_samples=[0,1],held_out_samples=[2,3],minimum_improvement_each_training_input=.02,evidence=evidence,
                status='Frozen from layer profiles; whole-model held-out validation is still required')

def validate_choices(values):
    return len(values)==94 and all(isinstance(b,int) and (3 if j<47 else 0)<=b<8 for j,b in enumerate(values))
