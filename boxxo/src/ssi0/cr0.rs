#[doc = "Register `CR0` reader"]
pub type R = crate::R<CR0_SPEC>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<CR0_SPEC>;
#[doc = "Field `SSI_CR0_DSS` reader - SSI Data Size Select"]
pub type SSI_CR0_DSS_R = crate::FieldReader<SSI_CR0_DSS_A>;
#[doc = "SSI Data Size Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSI_CR0_DSS_A {
    #[doc = "3: 4-bit data"]
    SSI_CR0_DSS_4 = 3,
    #[doc = "4: 5-bit data"]
    SSI_CR0_DSS_5 = 4,
    #[doc = "5: 6-bit data"]
    SSI_CR0_DSS_6 = 5,
    #[doc = "6: 7-bit data"]
    SSI_CR0_DSS_7 = 6,
    #[doc = "7: 8-bit data"]
    SSI_CR0_DSS_8 = 7,
    #[doc = "8: 9-bit data"]
    SSI_CR0_DSS_9 = 8,
    #[doc = "9: 10-bit data"]
    SSI_CR0_DSS_10 = 9,
    #[doc = "10: 11-bit data"]
    SSI_CR0_DSS_11 = 10,
    #[doc = "11: 12-bit data"]
    SSI_CR0_DSS_12 = 11,
    #[doc = "12: 13-bit data"]
    SSI_CR0_DSS_13 = 12,
    #[doc = "13: 14-bit data"]
    SSI_CR0_DSS_14 = 13,
    #[doc = "14: 15-bit data"]
    SSI_CR0_DSS_15 = 14,
    #[doc = "15: 16-bit data"]
    SSI_CR0_DSS_16 = 15,
}
impl From<SSI_CR0_DSS_A> for u8 {
    #[inline(always)]
    fn from(variant: SSI_CR0_DSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SSI_CR0_DSS_A {
    type Ux = u8;
}
impl SSI_CR0_DSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SSI_CR0_DSS_A> {
        match self.bits {
            3 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_4),
            4 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_5),
            5 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_6),
            6 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_7),
            7 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_8),
            8 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_9),
            9 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_10),
            10 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_11),
            11 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_12),
            12 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_13),
            13 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_14),
            14 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_15),
            15 => Some(SSI_CR0_DSS_A::SSI_CR0_DSS_16),
            _ => None,
        }
    }
    #[doc = "4-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_4(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_4
    }
    #[doc = "5-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_5(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_5
    }
    #[doc = "6-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_6(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_6
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_7(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_7
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_8(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_8
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_9(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_9
    }
    #[doc = "10-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_10(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_10
    }
    #[doc = "11-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_11(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_11
    }
    #[doc = "12-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_12(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_12
    }
    #[doc = "13-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_13(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_13
    }
    #[doc = "14-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_14(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_14
    }
    #[doc = "15-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_15(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_15
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_16(&self) -> bool {
        *self == SSI_CR0_DSS_A::SSI_CR0_DSS_16
    }
}
#[doc = "Field `SSI_CR0_DSS` writer - SSI Data Size Select"]
pub type SSI_CR0_DSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, SSI_CR0_DSS_A>;
impl<'a, REG, const O: u8> SSI_CR0_DSS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_4(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_4)
    }
    #[doc = "5-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_5(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_5)
    }
    #[doc = "6-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_6(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_6)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_7(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_7)
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_8(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_8)
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_9(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_9)
    }
    #[doc = "10-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_10(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_10)
    }
    #[doc = "11-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_11(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_11)
    }
    #[doc = "12-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_12(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_12)
    }
    #[doc = "13-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_13(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_13)
    }
    #[doc = "14-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_14(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_14)
    }
    #[doc = "15-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_15(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_15)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_16(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_DSS_A::SSI_CR0_DSS_16)
    }
}
#[doc = "Field `SSI_CR0_FRF` reader - SSI Frame Format Select"]
pub type SSI_CR0_FRF_R = crate::FieldReader<SSI_CR0_FRF_A>;
#[doc = "SSI Frame Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSI_CR0_FRF_A {
    #[doc = "0: Freescale SPI Frame Format"]
    SSI_CR0_FRF_MOTO = 0,
    #[doc = "1: Texas Instruments Synchronous Serial Frame Format"]
    SSI_CR0_FRF_TI = 1,
    #[doc = "2: MICROWIRE Frame Format"]
    SSI_CR0_FRF_NMW = 2,
}
impl From<SSI_CR0_FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: SSI_CR0_FRF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SSI_CR0_FRF_A {
    type Ux = u8;
}
impl SSI_CR0_FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SSI_CR0_FRF_A> {
        match self.bits {
            0 => Some(SSI_CR0_FRF_A::SSI_CR0_FRF_MOTO),
            1 => Some(SSI_CR0_FRF_A::SSI_CR0_FRF_TI),
            2 => Some(SSI_CR0_FRF_A::SSI_CR0_FRF_NMW),
            _ => None,
        }
    }
    #[doc = "Freescale SPI Frame Format"]
    #[inline(always)]
    pub fn is_ssi_cr0_frf_moto(&self) -> bool {
        *self == SSI_CR0_FRF_A::SSI_CR0_FRF_MOTO
    }
    #[doc = "Texas Instruments Synchronous Serial Frame Format"]
    #[inline(always)]
    pub fn is_ssi_cr0_frf_ti(&self) -> bool {
        *self == SSI_CR0_FRF_A::SSI_CR0_FRF_TI
    }
    #[doc = "MICROWIRE Frame Format"]
    #[inline(always)]
    pub fn is_ssi_cr0_frf_nmw(&self) -> bool {
        *self == SSI_CR0_FRF_A::SSI_CR0_FRF_NMW
    }
}
#[doc = "Field `SSI_CR0_FRF` writer - SSI Frame Format Select"]
pub type SSI_CR0_FRF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SSI_CR0_FRF_A>;
impl<'a, REG, const O: u8> SSI_CR0_FRF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Freescale SPI Frame Format"]
    #[inline(always)]
    pub fn ssi_cr0_frf_moto(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_FRF_A::SSI_CR0_FRF_MOTO)
    }
    #[doc = "Texas Instruments Synchronous Serial Frame Format"]
    #[inline(always)]
    pub fn ssi_cr0_frf_ti(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_FRF_A::SSI_CR0_FRF_TI)
    }
    #[doc = "MICROWIRE Frame Format"]
    #[inline(always)]
    pub fn ssi_cr0_frf_nmw(self) -> &'a mut crate::W<REG> {
        self.variant(SSI_CR0_FRF_A::SSI_CR0_FRF_NMW)
    }
}
#[doc = "Field `SSI_CR0_SPO` reader - SSI Serial Clock Polarity"]
pub type SSI_CR0_SPO_R = crate::BitReader;
#[doc = "Field `SSI_CR0_SPO` writer - SSI Serial Clock Polarity"]
pub type SSI_CR0_SPO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_CR0_SPH` reader - SSI Serial Clock Phase"]
pub type SSI_CR0_SPH_R = crate::BitReader;
#[doc = "Field `SSI_CR0_SPH` writer - SSI Serial Clock Phase"]
pub type SSI_CR0_SPH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_CR0_SCR` reader - SSI Serial Clock Rate"]
pub type SSI_CR0_SCR_R = crate::FieldReader;
#[doc = "Field `SSI_CR0_SCR` writer - SSI Serial Clock Rate"]
pub type SSI_CR0_SCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline(always)]
    pub fn ssi_cr0_dss(&self) -> SSI_CR0_DSS_R {
        SSI_CR0_DSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline(always)]
    pub fn ssi_cr0_frf(&self) -> SSI_CR0_FRF_R {
        SSI_CR0_FRF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline(always)]
    pub fn ssi_cr0_spo(&self) -> SSI_CR0_SPO_R {
        SSI_CR0_SPO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline(always)]
    pub fn ssi_cr0_sph(&self) -> SSI_CR0_SPH_R {
        SSI_CR0_SPH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline(always)]
    pub fn ssi_cr0_scr(&self) -> SSI_CR0_SCR_R {
        SSI_CR0_SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_cr0_dss(&mut self) -> SSI_CR0_DSS_W<CR0_SPEC, 0> {
        SSI_CR0_DSS_W::new(self)
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_cr0_frf(&mut self) -> SSI_CR0_FRF_W<CR0_SPEC, 4> {
        SSI_CR0_FRF_W::new(self)
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_cr0_spo(&mut self) -> SSI_CR0_SPO_W<CR0_SPEC, 6> {
        SSI_CR0_SPO_W::new(self)
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_cr0_sph(&mut self) -> SSI_CR0_SPH_W<CR0_SPEC, 7> {
        SSI_CR0_SPH_W::new(self)
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_cr0_scr(&mut self) -> SSI_CR0_SCR_W<CR0_SPEC, 8> {
        SSI_CR0_SCR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SSI Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for CR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
