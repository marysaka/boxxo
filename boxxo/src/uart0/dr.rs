#[doc = "Register `DR` reader"]
pub type R = crate::R<DR_SPEC>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DR_SPEC>;
#[doc = "Field `UART_DR_DATA` reader - Data Transmitted or Received"]
pub type UART_DR_DATA_R = crate::FieldReader;
#[doc = "Field `UART_DR_DATA` writer - Data Transmitted or Received"]
pub type UART_DR_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `UART_DR_FE` reader - UART Framing Error"]
pub type UART_DR_FE_R = crate::BitReader;
#[doc = "Field `UART_DR_FE` writer - UART Framing Error"]
pub type UART_DR_FE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_DR_PE` reader - UART Parity Error"]
pub type UART_DR_PE_R = crate::BitReader;
#[doc = "Field `UART_DR_PE` writer - UART Parity Error"]
pub type UART_DR_PE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_DR_BE` reader - UART Break Error"]
pub type UART_DR_BE_R = crate::BitReader;
#[doc = "Field `UART_DR_BE` writer - UART Break Error"]
pub type UART_DR_BE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_DR_OE` reader - UART Overrun Error"]
pub type UART_DR_OE_R = crate::BitReader;
#[doc = "Field `UART_DR_OE` writer - UART Overrun Error"]
pub type UART_DR_OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - Data Transmitted or Received"]
    #[inline(always)]
    pub fn uart_dr_data(&self) -> UART_DR_DATA_R {
        UART_DR_DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_dr_fe(&self) -> UART_DR_FE_R {
        UART_DR_FE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_dr_pe(&self) -> UART_DR_PE_R {
        UART_DR_PE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Break Error"]
    #[inline(always)]
    pub fn uart_dr_be(&self) -> UART_DR_BE_R {
        UART_DR_BE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_dr_oe(&self) -> UART_DR_OE_R {
        UART_DR_OE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Transmitted or Received"]
    #[inline(always)]
    #[must_use]
    pub fn uart_dr_data(&mut self) -> UART_DR_DATA_W<DR_SPEC, 0> {
        UART_DR_DATA_W::new(self)
    }
    #[doc = "Bit 8 - UART Framing Error"]
    #[inline(always)]
    #[must_use]
    pub fn uart_dr_fe(&mut self) -> UART_DR_FE_W<DR_SPEC, 8> {
        UART_DR_FE_W::new(self)
    }
    #[doc = "Bit 9 - UART Parity Error"]
    #[inline(always)]
    #[must_use]
    pub fn uart_dr_pe(&mut self) -> UART_DR_PE_W<DR_SPEC, 9> {
        UART_DR_PE_W::new(self)
    }
    #[doc = "Bit 10 - UART Break Error"]
    #[inline(always)]
    #[must_use]
    pub fn uart_dr_be(&mut self) -> UART_DR_BE_W<DR_SPEC, 10> {
        UART_DR_BE_W::new(self)
    }
    #[doc = "Bit 11 - UART Overrun Error"]
    #[inline(always)]
    #[must_use]
    pub fn uart_dr_oe(&mut self) -> UART_DR_OE_W<DR_SPEC, 11> {
        UART_DR_OE_W::new(self)
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
#[doc = "UART Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
