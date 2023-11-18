#[doc = "Register `CHMAP3` reader"]
pub type R = crate::R<CHMAP3_SPEC>;
#[doc = "Register `CHMAP3` writer"]
pub type W = crate::W<CHMAP3_SPEC>;
#[doc = "Field `UDMA_CHMAP3_CH24SEL` reader - uDMA Channel 24 Source Select"]
pub type UDMA_CHMAP3_CH24SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP3_CH24SEL` writer - uDMA Channel 24 Source Select"]
pub type UDMA_CHMAP3_CH24SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP3_CH25SEL` reader - uDMA Channel 25 Source Select"]
pub type UDMA_CHMAP3_CH25SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP3_CH25SEL` writer - uDMA Channel 25 Source Select"]
pub type UDMA_CHMAP3_CH25SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP3_CH26SEL` reader - uDMA Channel 26 Source Select"]
pub type UDMA_CHMAP3_CH26SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP3_CH26SEL` writer - uDMA Channel 26 Source Select"]
pub type UDMA_CHMAP3_CH26SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP3_CH27SEL` reader - uDMA Channel 27 Source Select"]
pub type UDMA_CHMAP3_CH27SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP3_CH27SEL` writer - uDMA Channel 27 Source Select"]
pub type UDMA_CHMAP3_CH27SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP3_CH28SEL` reader - uDMA Channel 28 Source Select"]
pub type UDMA_CHMAP3_CH28SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP3_CH28SEL` writer - uDMA Channel 28 Source Select"]
pub type UDMA_CHMAP3_CH28SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP3_CH29SEL` reader - uDMA Channel 29 Source Select"]
pub type UDMA_CHMAP3_CH29SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP3_CH29SEL` writer - uDMA Channel 29 Source Select"]
pub type UDMA_CHMAP3_CH29SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP3_CH30SEL` reader - uDMA Channel 30 Source Select"]
pub type UDMA_CHMAP3_CH30SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP3_CH30SEL` writer - uDMA Channel 30 Source Select"]
pub type UDMA_CHMAP3_CH30SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP3_CH31SEL` reader - uDMA Channel 31 Source Select"]
pub type UDMA_CHMAP3_CH31SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP3_CH31SEL` writer - uDMA Channel 31 Source Select"]
pub type UDMA_CHMAP3_CH31SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 24 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch24sel(&self) -> UDMA_CHMAP3_CH24SEL_R {
        UDMA_CHMAP3_CH24SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 25 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch25sel(&self) -> UDMA_CHMAP3_CH25SEL_R {
        UDMA_CHMAP3_CH25SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 26 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch26sel(&self) -> UDMA_CHMAP3_CH26SEL_R {
        UDMA_CHMAP3_CH26SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 27 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch27sel(&self) -> UDMA_CHMAP3_CH27SEL_R {
        UDMA_CHMAP3_CH27SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 28 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch28sel(&self) -> UDMA_CHMAP3_CH28SEL_R {
        UDMA_CHMAP3_CH28SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 29 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch29sel(&self) -> UDMA_CHMAP3_CH29SEL_R {
        UDMA_CHMAP3_CH29SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 30 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch30sel(&self) -> UDMA_CHMAP3_CH30SEL_R {
        UDMA_CHMAP3_CH30SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 31 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch31sel(&self) -> UDMA_CHMAP3_CH31SEL_R {
        UDMA_CHMAP3_CH31SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 24 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap3_ch24sel(&mut self) -> UDMA_CHMAP3_CH24SEL_W<CHMAP3_SPEC, 0> {
        UDMA_CHMAP3_CH24SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA Channel 25 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap3_ch25sel(&mut self) -> UDMA_CHMAP3_CH25SEL_W<CHMAP3_SPEC, 4> {
        UDMA_CHMAP3_CH25SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA Channel 26 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap3_ch26sel(&mut self) -> UDMA_CHMAP3_CH26SEL_W<CHMAP3_SPEC, 8> {
        UDMA_CHMAP3_CH26SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA Channel 27 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap3_ch27sel(&mut self) -> UDMA_CHMAP3_CH27SEL_W<CHMAP3_SPEC, 12> {
        UDMA_CHMAP3_CH27SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA Channel 28 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap3_ch28sel(&mut self) -> UDMA_CHMAP3_CH28SEL_W<CHMAP3_SPEC, 16> {
        UDMA_CHMAP3_CH28SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA Channel 29 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap3_ch29sel(&mut self) -> UDMA_CHMAP3_CH29SEL_W<CHMAP3_SPEC, 20> {
        UDMA_CHMAP3_CH29SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA Channel 30 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap3_ch30sel(&mut self) -> UDMA_CHMAP3_CH30SEL_W<CHMAP3_SPEC, 24> {
        UDMA_CHMAP3_CH30SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA Channel 31 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap3_ch31sel(&mut self) -> UDMA_CHMAP3_CH31SEL_W<CHMAP3_SPEC, 28> {
        UDMA_CHMAP3_CH31SEL_W::new(self)
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
#[doc = "DMA Channel Map Select 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHMAP3_SPEC;
impl crate::RegisterSpec for CHMAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap3::R`](R) reader structure"]
impl crate::Readable for CHMAP3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chmap3::W`](W) writer structure"]
impl crate::Writable for CHMAP3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHMAP3 to value 0"]
impl crate::Resettable for CHMAP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
