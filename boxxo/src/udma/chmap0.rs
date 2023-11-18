#[doc = "Register `CHMAP0` reader"]
pub type R = crate::R<CHMAP0_SPEC>;
#[doc = "Register `CHMAP0` writer"]
pub type W = crate::W<CHMAP0_SPEC>;
#[doc = "Field `UDMA_CHMAP0_CH0SEL` reader - uDMA Channel 0 Source Select"]
pub type UDMA_CHMAP0_CH0SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP0_CH0SEL` writer - uDMA Channel 0 Source Select"]
pub type UDMA_CHMAP0_CH0SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP0_CH1SEL` reader - uDMA Channel 1 Source Select"]
pub type UDMA_CHMAP0_CH1SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP0_CH1SEL` writer - uDMA Channel 1 Source Select"]
pub type UDMA_CHMAP0_CH1SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP0_CH2SEL` reader - uDMA Channel 2 Source Select"]
pub type UDMA_CHMAP0_CH2SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP0_CH2SEL` writer - uDMA Channel 2 Source Select"]
pub type UDMA_CHMAP0_CH2SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP0_CH3SEL` reader - uDMA Channel 3 Source Select"]
pub type UDMA_CHMAP0_CH3SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP0_CH3SEL` writer - uDMA Channel 3 Source Select"]
pub type UDMA_CHMAP0_CH3SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP0_CH4SEL` reader - uDMA Channel 4 Source Select"]
pub type UDMA_CHMAP0_CH4SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP0_CH4SEL` writer - uDMA Channel 4 Source Select"]
pub type UDMA_CHMAP0_CH4SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP0_CH5SEL` reader - uDMA Channel 5 Source Select"]
pub type UDMA_CHMAP0_CH5SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP0_CH5SEL` writer - uDMA Channel 5 Source Select"]
pub type UDMA_CHMAP0_CH5SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP0_CH6SEL` reader - uDMA Channel 6 Source Select"]
pub type UDMA_CHMAP0_CH6SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP0_CH6SEL` writer - uDMA Channel 6 Source Select"]
pub type UDMA_CHMAP0_CH6SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP0_CH7SEL` reader - uDMA Channel 7 Source Select"]
pub type UDMA_CHMAP0_CH7SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP0_CH7SEL` writer - uDMA Channel 7 Source Select"]
pub type UDMA_CHMAP0_CH7SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 0 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch0sel(&self) -> UDMA_CHMAP0_CH0SEL_R {
        UDMA_CHMAP0_CH0SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 1 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch1sel(&self) -> UDMA_CHMAP0_CH1SEL_R {
        UDMA_CHMAP0_CH1SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 2 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch2sel(&self) -> UDMA_CHMAP0_CH2SEL_R {
        UDMA_CHMAP0_CH2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 3 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch3sel(&self) -> UDMA_CHMAP0_CH3SEL_R {
        UDMA_CHMAP0_CH3SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 4 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch4sel(&self) -> UDMA_CHMAP0_CH4SEL_R {
        UDMA_CHMAP0_CH4SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 5 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch5sel(&self) -> UDMA_CHMAP0_CH5SEL_R {
        UDMA_CHMAP0_CH5SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 6 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch6sel(&self) -> UDMA_CHMAP0_CH6SEL_R {
        UDMA_CHMAP0_CH6SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 7 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch7sel(&self) -> UDMA_CHMAP0_CH7SEL_R {
        UDMA_CHMAP0_CH7SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 0 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap0_ch0sel(&mut self) -> UDMA_CHMAP0_CH0SEL_W<CHMAP0_SPEC, 0> {
        UDMA_CHMAP0_CH0SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA Channel 1 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap0_ch1sel(&mut self) -> UDMA_CHMAP0_CH1SEL_W<CHMAP0_SPEC, 4> {
        UDMA_CHMAP0_CH1SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA Channel 2 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap0_ch2sel(&mut self) -> UDMA_CHMAP0_CH2SEL_W<CHMAP0_SPEC, 8> {
        UDMA_CHMAP0_CH2SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA Channel 3 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap0_ch3sel(&mut self) -> UDMA_CHMAP0_CH3SEL_W<CHMAP0_SPEC, 12> {
        UDMA_CHMAP0_CH3SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA Channel 4 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap0_ch4sel(&mut self) -> UDMA_CHMAP0_CH4SEL_W<CHMAP0_SPEC, 16> {
        UDMA_CHMAP0_CH4SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA Channel 5 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap0_ch5sel(&mut self) -> UDMA_CHMAP0_CH5SEL_W<CHMAP0_SPEC, 20> {
        UDMA_CHMAP0_CH5SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA Channel 6 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap0_ch6sel(&mut self) -> UDMA_CHMAP0_CH6SEL_W<CHMAP0_SPEC, 24> {
        UDMA_CHMAP0_CH6SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA Channel 7 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap0_ch7sel(&mut self) -> UDMA_CHMAP0_CH7SEL_W<CHMAP0_SPEC, 28> {
        UDMA_CHMAP0_CH7SEL_W::new(self)
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
#[doc = "DMA Channel Map Select 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHMAP0_SPEC;
impl crate::RegisterSpec for CHMAP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap0::R`](R) reader structure"]
impl crate::Readable for CHMAP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chmap0::W`](W) writer structure"]
impl crate::Writable for CHMAP0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHMAP0 to value 0"]
impl crate::Resettable for CHMAP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
