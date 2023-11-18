#[doc = "Register `CHMAP2` reader"]
pub type R = crate::R<CHMAP2_SPEC>;
#[doc = "Register `CHMAP2` writer"]
pub type W = crate::W<CHMAP2_SPEC>;
#[doc = "Field `UDMA_CHMAP2_CH16SEL` reader - uDMA Channel 16 Source Select"]
pub type UDMA_CHMAP2_CH16SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP2_CH16SEL` writer - uDMA Channel 16 Source Select"]
pub type UDMA_CHMAP2_CH16SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP2_CH17SEL` reader - uDMA Channel 17 Source Select"]
pub type UDMA_CHMAP2_CH17SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP2_CH17SEL` writer - uDMA Channel 17 Source Select"]
pub type UDMA_CHMAP2_CH17SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP2_CH18SEL` reader - uDMA Channel 18 Source Select"]
pub type UDMA_CHMAP2_CH18SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP2_CH18SEL` writer - uDMA Channel 18 Source Select"]
pub type UDMA_CHMAP2_CH18SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP2_CH19SEL` reader - uDMA Channel 19 Source Select"]
pub type UDMA_CHMAP2_CH19SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP2_CH19SEL` writer - uDMA Channel 19 Source Select"]
pub type UDMA_CHMAP2_CH19SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP2_CH20SEL` reader - uDMA Channel 20 Source Select"]
pub type UDMA_CHMAP2_CH20SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP2_CH20SEL` writer - uDMA Channel 20 Source Select"]
pub type UDMA_CHMAP2_CH20SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP2_CH21SEL` reader - uDMA Channel 21 Source Select"]
pub type UDMA_CHMAP2_CH21SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP2_CH21SEL` writer - uDMA Channel 21 Source Select"]
pub type UDMA_CHMAP2_CH21SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP2_CH22SEL` reader - uDMA Channel 22 Source Select"]
pub type UDMA_CHMAP2_CH22SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP2_CH22SEL` writer - uDMA Channel 22 Source Select"]
pub type UDMA_CHMAP2_CH22SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UDMA_CHMAP2_CH23SEL` reader - uDMA Channel 23 Source Select"]
pub type UDMA_CHMAP2_CH23SEL_R = crate::FieldReader;
#[doc = "Field `UDMA_CHMAP2_CH23SEL` writer - uDMA Channel 23 Source Select"]
pub type UDMA_CHMAP2_CH23SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - uDMA Channel 16 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch16sel(&self) -> UDMA_CHMAP2_CH16SEL_R {
        UDMA_CHMAP2_CH16SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA Channel 17 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch17sel(&self) -> UDMA_CHMAP2_CH17SEL_R {
        UDMA_CHMAP2_CH17SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA Channel 18 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch18sel(&self) -> UDMA_CHMAP2_CH18SEL_R {
        UDMA_CHMAP2_CH18SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA Channel 19 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch19sel(&self) -> UDMA_CHMAP2_CH19SEL_R {
        UDMA_CHMAP2_CH19SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA Channel 20 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch20sel(&self) -> UDMA_CHMAP2_CH20SEL_R {
        UDMA_CHMAP2_CH20SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA Channel 21 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch21sel(&self) -> UDMA_CHMAP2_CH21SEL_R {
        UDMA_CHMAP2_CH21SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA Channel 22 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch22sel(&self) -> UDMA_CHMAP2_CH22SEL_R {
        UDMA_CHMAP2_CH22SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA Channel 23 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch23sel(&self) -> UDMA_CHMAP2_CH23SEL_R {
        UDMA_CHMAP2_CH23SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA Channel 16 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap2_ch16sel(&mut self) -> UDMA_CHMAP2_CH16SEL_W<CHMAP2_SPEC, 0> {
        UDMA_CHMAP2_CH16SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA Channel 17 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap2_ch17sel(&mut self) -> UDMA_CHMAP2_CH17SEL_W<CHMAP2_SPEC, 4> {
        UDMA_CHMAP2_CH17SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA Channel 18 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap2_ch18sel(&mut self) -> UDMA_CHMAP2_CH18SEL_W<CHMAP2_SPEC, 8> {
        UDMA_CHMAP2_CH18SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA Channel 19 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap2_ch19sel(&mut self) -> UDMA_CHMAP2_CH19SEL_W<CHMAP2_SPEC, 12> {
        UDMA_CHMAP2_CH19SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA Channel 20 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap2_ch20sel(&mut self) -> UDMA_CHMAP2_CH20SEL_W<CHMAP2_SPEC, 16> {
        UDMA_CHMAP2_CH20SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA Channel 21 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap2_ch21sel(&mut self) -> UDMA_CHMAP2_CH21SEL_W<CHMAP2_SPEC, 20> {
        UDMA_CHMAP2_CH21SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA Channel 22 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap2_ch22sel(&mut self) -> UDMA_CHMAP2_CH22SEL_W<CHMAP2_SPEC, 24> {
        UDMA_CHMAP2_CH22SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA Channel 23 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn udma_chmap2_ch23sel(&mut self) -> UDMA_CHMAP2_CH23SEL_W<CHMAP2_SPEC, 28> {
        UDMA_CHMAP2_CH23SEL_W::new(self)
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
#[doc = "DMA Channel Map Select 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHMAP2_SPEC;
impl crate::RegisterSpec for CHMAP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap2::R`](R) reader structure"]
impl crate::Readable for CHMAP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chmap2::W`](W) writer structure"]
impl crate::Writable for CHMAP2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHMAP2 to value 0"]
impl crate::Resettable for CHMAP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
