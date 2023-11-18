#[doc = "Register `CHMAP1` reader"]
pub type R = crate::R<CHMAP1_SPEC>;
#[doc = "Register `CHMAP1` writer"]
pub type W = crate::W<CHMAP1_SPEC>;
#[doc = "Field `UDMA_CHMAP1_CH8SEL` reader - uDMA Channel 8 Source Select"]
pub type UDMA_CHMAP1_CH8SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP1_CH8SEL` writer - uDMA Channel 8 Source Select"]
pub type UDMA_CHMAP1_CH8SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP1_CH9SEL` reader - uDMA Channel 9 Source Select"]
pub type UDMA_CHMAP1_CH9SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP1_CH9SEL` writer - uDMA Channel 9 Source Select"]
pub type UDMA_CHMAP1_CH9SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP1_CH10SEL` reader - uDMA Channel 10 Source Select"]
pub type UDMA_CHMAP1_CH10SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP1_CH10SEL` writer - uDMA Channel 10 Source Select"]
pub type UDMA_CHMAP1_CH10SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP1_CH11SEL` reader - uDMA Channel 11 Source Select"]
pub type UDMA_CHMAP1_CH11SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP1_CH11SEL` writer - uDMA Channel 11 Source Select"]
pub type UDMA_CHMAP1_CH11SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP1_CH12SEL` reader - uDMA Channel 12 Source Select"]
pub type UDMA_CHMAP1_CH12SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP1_CH12SEL` writer - uDMA Channel 12 Source Select"]
pub type UDMA_CHMAP1_CH12SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP1_CH13SEL` reader - uDMA Channel 13 Source Select"]
pub type UDMA_CHMAP1_CH13SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP1_CH13SEL` writer - uDMA Channel 13 Source Select"]
pub type UDMA_CHMAP1_CH13SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP1_CH14SEL` reader - uDMA Channel 14 Source Select"]
pub type UDMA_CHMAP1_CH14SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP1_CH14SEL` writer - uDMA Channel 14 Source Select"]
pub type UDMA_CHMAP1_CH14SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP1_CH15SEL` reader - uDMA Channel 15 Source Select"]
pub type UDMA_CHMAP1_CH15SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP1_CH15SEL` writer - uDMA Channel 15 Source Select"]
pub type UDMA_CHMAP1_CH15SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 8 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch8sel(&self) -> UDMA_CHMAP1_CH8SEL_R {
        UDMA_CHMAP1_CH8SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 9 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch9sel(&self) -> UDMA_CHMAP1_CH9SEL_R {
        UDMA_CHMAP1_CH9SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 10 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch10sel(&self) -> UDMA_CHMAP1_CH10SEL_R {
        UDMA_CHMAP1_CH10SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 11 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch11sel(&self) -> UDMA_CHMAP1_CH11SEL_R {
        UDMA_CHMAP1_CH11SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 12 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch12sel(&self) -> UDMA_CHMAP1_CH12SEL_R {
        UDMA_CHMAP1_CH12SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 13 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch13sel(&self) -> UDMA_CHMAP1_CH13SEL_R {
        UDMA_CHMAP1_CH13SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 14 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch14sel(&self) -> UDMA_CHMAP1_CH14SEL_R {
        UDMA_CHMAP1_CH14SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 15 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch15sel(&self) -> UDMA_CHMAP1_CH15SEL_R {
        UDMA_CHMAP1_CH15SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 8 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap1_ch8sel(&mut self) -> UDMA_CHMAP1_CH8SEL_W<CHMAP1_SPEC, 0> {
        UDMA_CHMAP1_CH8SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA Channel 9 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap1_ch9sel(&mut self) -> UDMA_CHMAP1_CH9SEL_W<CHMAP1_SPEC, 4> {
        UDMA_CHMAP1_CH9SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA Channel 10 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap1_ch10sel(&mut self) -> UDMA_CHMAP1_CH10SEL_W<CHMAP1_SPEC, 8> {
        UDMA_CHMAP1_CH10SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA Channel 11 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap1_ch11sel(&mut self) -> UDMA_CHMAP1_CH11SEL_W<CHMAP1_SPEC, 12> {
        UDMA_CHMAP1_CH11SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA Channel 12 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap1_ch12sel(&mut self) -> UDMA_CHMAP1_CH12SEL_W<CHMAP1_SPEC, 16> {
        UDMA_CHMAP1_CH12SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA Channel 13 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap1_ch13sel(&mut self) -> UDMA_CHMAP1_CH13SEL_W<CHMAP1_SPEC, 20> {
        UDMA_CHMAP1_CH13SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA Channel 14 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap1_ch14sel(&mut self) -> UDMA_CHMAP1_CH14SEL_W<CHMAP1_SPEC, 24> {
        UDMA_CHMAP1_CH14SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA Channel 15 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap1_ch15sel(&mut self) -> UDMA_CHMAP1_CH15SEL_W<CHMAP1_SPEC, 28> {
        UDMA_CHMAP1_CH15SEL_W::new(self)
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
#[doc = "DMA Channel Map Select 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHMAP1_SPEC;
impl crate::RegisterSpec for CHMAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap1::R`](R) reader structure"]
impl crate::Readable for CHMAP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chmap1::W`](W) writer structure"]
impl crate::Writable for CHMAP1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHMAP1 to value 0"]
impl crate::Resettable for CHMAP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
