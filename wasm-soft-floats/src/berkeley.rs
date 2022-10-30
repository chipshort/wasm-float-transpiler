use crate::F32;


pub(crate) fn softfloat_addMagsF32( uiA: F32, uiB: F32 ) -> F32
{
    let expA: u16;
    let sigA: u32;
    let expB: u16;
    let sigB: u32;
    let expDiff: u16;
    let uiZ: u32;
    let signZ: bool;
    let expZ: u16;
    let sigZ: u32;
    let uZ: F32;

    /*------------------------------------------------------------------------
    *------------------------------------------------------------------------*/
    expA = uiA.exponent() as u16;
    sigA = uiA.fraction();
    expB = uiB.exponent() as u16;
    sigB = uiB.fraction();
    /*------------------------------------------------------------------------
    *------------------------------------------------------------------------*/
    expDiff = expA - expB;
    if expDiff != 0 {
        /*--------------------------------------------------------------------
        *--------------------------------------------------------------------*/
        if expA != 0 {
            uiZ = uiA.0 + sigB;
            return goto_uiZ(uZ, uiZ);
        }
        if expA == 0xFF {
            if ( sigA | sigB ) goto propagateNaN;
            uiZ = uiA;
            goto uiZ;
        }
        signZ = signF32UI( uiA );
        expZ = expA;
        sigZ = 0x01000000 + sigA + sigB;
        if ( ! (sigZ & 1) && (expZ < 0xFE) ) {
            uiZ = packToF32UI( signZ, expZ, sigZ>>1 );
            goto uiZ;
        }
        sigZ <<= 6;
    } else {
        /*--------------------------------------------------------------------
        *--------------------------------------------------------------------*/
        signZ = signF32UI( uiA );
        sigA <<= 6;
        sigB <<= 6;
        if ( expDiff < 0 ) {
            if ( expB == 0xFF ) {
                if ( sigB ) goto propagateNaN;
                uiZ = packToF32UI( signZ, 0xFF, 0 );
                goto uiZ;
            }
            expZ = expB;
            sigA += expA ? 0x20000000 : sigA;
            sigA = softfloat_shiftRightJam32( sigA, -expDiff );
        } else {
            if ( expA == 0xFF ) {
                if ( sigA ) goto propagateNaN;
                uiZ = uiA;
                goto uiZ;
            }
            expZ = expA;
            sigB += expB ? 0x20000000 : sigB;
            sigB = softfloat_shiftRightJam32( sigB, expDiff );
        }
        sigZ = 0x20000000 + sigA + sigB;
        if ( sigZ < 0x40000000 ) {
            --expZ;
            sigZ <<= 1;
        }
    }
    return softfloat_roundPackToF32( signZ, expZ, sigZ );
    /*------------------------------------------------------------------------
    *------------------------------------------------------------------------*/
 propagateNaN:
    uiZ = softfloat_propagateNaNF32UI( uiA, uiB );
 uiZ:
    uZ.0 = uiZ;
    return uZ.f;

}

fn goto_uiZ(mut uZ: F32, uiZ: u32) -> F32 {
    uZ.0 = uiZ;
    uZ
}

fn goto_propagateNaN(mut uZ: F32, uiZ: u32, uiA: F32, uiB: F32) -> F32 {
    uiZ = softfloat_propagateNaNF32UI( uiA, uiB );
    goto_uiZ(uZ, uiZ)
}