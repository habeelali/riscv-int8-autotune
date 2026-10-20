"""Independent integer oracle. NHWC, OHWI / HW-channel depthwise weights."""
def requantize(x, m, shift):
    x *= 1 << max(shift, 0)
    assert -(1 << 31) <= x < (1 << 31)
    prod = x * m
    nudged = prod + ((1 << 30) if (x < 0) == (m < 0) else 1 - (1 << 30))
    high = nudged // (1 << 31) if nudged >= 0 else -((-nudged) // (1 << 31))
    if x == m == -(1 << 31): high = (1 << 31) - 1
    e = max(-shift, 0)
    base = high >> e
    return base + ((high & ((1 << e) - 1)) > (((1 << e) - 1) // 2 + (base < 0)))

def vectors(w):
    state = w['seed']
    def rand(n):
        nonlocal state
        a=[]
        for _ in range(n):
            state ^= (state << 13) & 0xffffffff
            state ^= state >> 17
            state ^= (state << 5) & 0xffffffff
            state &= 0xffffffff
            a.append((state & 255)-128)
        return a
    h,wd,ic,oc,k = (w[x] for x in ('height','width','input_channels','output_channels','kernel'))
    inp=rand(h*wd*ic)
    weights=rand(k*k*(ic if w['operator']=='depthwise' else ic*oc))
    bias=[v*17 for v in rand(oc)]
    mult=w.get('multipliers') or [1073741824+(c%7)*11234567 for c in range(oc)]
    shift=w.get('shifts') or [w['quant_shift']-(c%3) for c in range(oc)]
    oh=(h+2*w['padding']-k)//w['stride']+1
    ow=(wd+2*w['padding']-k)//w['stride']+1
    out=[]
    for y in range(oh):
        for x in range(ow):
            for c in range(oc):
                acc=bias[c]
                for ky in range(k):
                    iy=y*w['stride']+ky-w['padding']
                    for kx in range(k):
                        ix=x*w['stride']+kx-w['padding']
                        if not (0<=iy<h and 0<=ix<wd): continue
                        if w['operator']=='depthwise':
                            acc+=(inp[(iy*wd+ix)*ic+c]+w['input_offset'])*weights[(ky*k+kx)*ic+c]
                        else:
                            a=(iy*wd+ix)*ic; b=((c*k+ky)*k+kx)*ic
                            acc+=sum((inp[a+i]+w['input_offset'])*weights[b+i] for i in range(ic))
                out.append(max(w['activation_min'],min(w['activation_max'],requantize(acc,mult[c],shift[c])+w['output_offset'])))
    return inp,weights,bias,mult,shift,out
