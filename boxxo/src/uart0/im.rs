#[doc = "Register `IM` reader"]
pub type R = crate::R<IM_SPEC>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<IM_SPEC>;
#[doc = "Field `UART_IM_CTSMIM` reader - UART Clear to Send Modem Interrupt Mask"]
pub type UART_IM_CTSMIM_R = crate::BitReader;
#[doc = "Field `UART_IM_CTSMIM` writer - UART Clear to Send Modem Interrupt Mask"]
pub type UART_IM_CTSMIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_IM_RXIM` reader - UART Receive Interrupt Mask"]
pub type UART_IM_RXIM_R = crate::BitReader;
#[doc = "Field `UART_IM_RXIM` writer - UART Receive Interrupt Mask"]
pub type UART_IM_RXIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_IM_TXIM` reader - UART Transmit Interrupt Mask"]
pub type UART_IM_TXIM_R = crate::BitReader;
#[doc = "Field `UART_IM_TXIM` writer - UART Transmit Interrupt Mask"]
pub type UART_IM_TXIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_IM_RTIM` reader - UART Receive Time-Out Interrupt Mask"]
pub type UART_IM_RTIM_R = crate::BitReader;
#[doc = "Field `UART_IM_RTIM` writer - UART Receive Time-Out Interrupt Mask"]
pub type UART_IM_RTIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_IM_FEIM` reader - UART Framing Error Interrupt Mask"]
pub type UART_IM_FEIM_R = crate::BitReader;
#[doc = "Field `UART_IM_FEIM` writer - UART Framing Error Interrupt Mask"]
pub type UART_IM_FEIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_IM_PEIM` reader - UART Parity Error Interrupt Mask"]
pub type UART_IM_PEIM_R = crate::BitReader;
#[doc = "Field `UART_IM_PEIM` writer - UART Parity Error Interrupt Mask"]
pub type UART_IM_PEIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_IM_BEIM` reader - UART Break Error Interrupt Mask"]
pub type UART_IM_BEIM_R = crate::BitReader;
#[doc = "Field `UART_IM_BEIM` writer - UART Break Error Interrupt Mask"]
pub type UART_IM_BEIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_IM_OEIM` reader - UART Overrun Error Interrupt Mask"]
pub type UART_IM_OEIM_R = crate::BitReader;
#[doc = "Field `UART_IM_OEIM` writer - UART Overrun Error Interrupt Mask"]
pub type UART_IM_OEIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_IM_9BITIM` reader - 9-Bit Mode Interrupt Mask"]
pub type UART_IM_9BITIM_R = crate::BitReader;
#[doc = "Field `UART_IM_9BITIM` writer - 9-Bit Mode Interrupt Mask"]
pub type UART_IM_9BITIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - UART Clear to Send Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_ctsmim(&self) -> UART_IM_CTSMIM_R {
        UART_IM_CTSMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Receive Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rxim(&self) -> UART_IM_RXIM_R {
        UART_IM_RXIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_txim(&self) -> UART_IM_TXIM_R {
        UART_IM_TXIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rtim(&self) -> UART_IM_RTIM_R {
        UART_IM_RTIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_feim(&self) -> UART_IM_FEIM_R {
        UART_IM_FEIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_peim(&self) -> UART_IM_PEIM_R {
        UART_IM_PEIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_beim(&self) -> UART_IM_BEIM_R {
        UART_IM_BEIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_oeim(&self) -> UART_IM_OEIM_R {
        UART_IM_OEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 9-Bit Mode Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_9bitim(&self) -> UART_IM_9BITIM_R {
        UART_IM_9BITIM_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - UART Clear to Send Modem Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn uart_im_ctsmim(&mut self) -> UART_IM_CTSMIM_W<IM_SPEC, 1> {
        UART_IM_CTSMIM_W::new(self)
    }
    #[doc = "Bit 4 - UART Receive Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn uart_im_rxim(&mut self) -> UART_IM_RXIM_W<IM_SPEC, 4> {
        UART_IM_RXIM_W::new(self)
    }
    #[doc = "Bit 5 - UART Transmit Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn uart_im_txim(&mut self) -> UART_IM_TXIM_W<IM_SPEC, 5> {
        UART_IM_TXIM_W::new(self)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn uart_im_rtim(&mut self) -> UART_IM_RTIM_W<IM_SPEC, 6> {
        UART_IM_RTIM_W::new(self)
    }
    #[doc = "Bit 7 - UART Framing Error Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn uart_im_feim(&mut self) -> UART_IM_FEIM_W<IM_SPEC, 7> {
        UART_IM_FEIM_W::new(self)
    }
    #[doc = "Bit 8 - UART Parity Error Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn uart_im_peim(&mut self) -> UART_IM_PEIM_W<IM_SPEC, 8> {
        UART_IM_PEIM_W::new(self)
    }
    #[doc = "Bit 9 - UART Break Error Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn uart_im_beim(&mut self) -> UART_IM_BEIM_W<IM_SPEC, 9> {
        UART_IM_BEIM_W::new(self)
    }
    #[doc = "Bit 10 - UART Overrun Error Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn uart_im_oeim(&mut self) -> UART_IM_OEIM_W<IM_SPEC, 10> {
        UART_IM_OEIM_W::new(self)
    }
    #[doc = "Bit 12 - 9-Bit Mode Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn uart_im_9bitim(&mut self) -> UART_IM_9BITIM_W<IM_SPEC, 12> {
        UART_IM_9BITIM_W::new(self)
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
#[doc = "UART Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for IM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for IM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
