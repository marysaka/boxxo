#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `UART_CTL_UARTEN` reader - UART Enable"]
pub type UART_CTL_UARTEN_R = crate::BitReader;
#[doc = "Field `UART_CTL_UARTEN` writer - UART Enable"]
pub type UART_CTL_UARTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_CTL_SIREN` reader - UART SIR Enable"]
pub type UART_CTL_SIREN_R = crate::BitReader;
#[doc = "Field `UART_CTL_SIREN` writer - UART SIR Enable"]
pub type UART_CTL_SIREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_CTL_SIRLP` reader - UART SIR Low-Power Mode"]
pub type UART_CTL_SIRLP_R = crate::BitReader;
#[doc = "Field `UART_CTL_SIRLP` writer - UART SIR Low-Power Mode"]
pub type UART_CTL_SIRLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_CTL_SMART` reader - ISO 7816 Smart Card Support"]
pub type UART_CTL_SMART_R = crate::BitReader;
#[doc = "Field `UART_CTL_SMART` writer - ISO 7816 Smart Card Support"]
pub type UART_CTL_SMART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_CTL_EOT` reader - End of Transmission"]
pub type UART_CTL_EOT_R = crate::BitReader;
#[doc = "Field `UART_CTL_EOT` writer - End of Transmission"]
pub type UART_CTL_EOT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_CTL_HSE` reader - High-Speed Enable"]
pub type UART_CTL_HSE_R = crate::BitReader;
#[doc = "Field `UART_CTL_HSE` writer - High-Speed Enable"]
pub type UART_CTL_HSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_CTL_LBE` reader - UART Loop Back Enable"]
pub type UART_CTL_LBE_R = crate::BitReader;
#[doc = "Field `UART_CTL_LBE` writer - UART Loop Back Enable"]
pub type UART_CTL_LBE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_CTL_TXE` reader - UART Transmit Enable"]
pub type UART_CTL_TXE_R = crate::BitReader;
#[doc = "Field `UART_CTL_TXE` writer - UART Transmit Enable"]
pub type UART_CTL_TXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_CTL_RXE` reader - UART Receive Enable"]
pub type UART_CTL_RXE_R = crate::BitReader;
#[doc = "Field `UART_CTL_RXE` writer - UART Receive Enable"]
pub type UART_CTL_RXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_CTL_RTS` reader - Request to Send"]
pub type UART_CTL_RTS_R = crate::BitReader;
#[doc = "Field `UART_CTL_RTS` writer - Request to Send"]
pub type UART_CTL_RTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_CTL_RTSEN` reader - Enable Request to Send"]
pub type UART_CTL_RTSEN_R = crate::BitReader;
#[doc = "Field `UART_CTL_RTSEN` writer - Enable Request to Send"]
pub type UART_CTL_RTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_CTL_CTSEN` reader - Enable Clear To Send"]
pub type UART_CTL_CTSEN_R = crate::BitReader;
#[doc = "Field `UART_CTL_CTSEN` writer - Enable Clear To Send"]
pub type UART_CTL_CTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    pub fn uart_ctl_uarten(&self) -> UART_CTL_UARTEN_R {
        UART_CTL_UARTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART SIR Enable"]
    #[inline(always)]
    pub fn uart_ctl_siren(&self) -> UART_CTL_SIREN_R {
        UART_CTL_SIREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART SIR Low-Power Mode"]
    #[inline(always)]
    pub fn uart_ctl_sirlp(&self) -> UART_CTL_SIRLP_R {
        UART_CTL_SIRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ISO 7816 Smart Card Support"]
    #[inline(always)]
    pub fn uart_ctl_smart(&self) -> UART_CTL_SMART_R {
        UART_CTL_SMART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn uart_ctl_eot(&self) -> UART_CTL_EOT_R {
        UART_CTL_EOT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High-Speed Enable"]
    #[inline(always)]
    pub fn uart_ctl_hse(&self) -> UART_CTL_HSE_R {
        UART_CTL_HSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Loop Back Enable"]
    #[inline(always)]
    pub fn uart_ctl_lbe(&self) -> UART_CTL_LBE_R {
        UART_CTL_LBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART Transmit Enable"]
    #[inline(always)]
    pub fn uart_ctl_txe(&self) -> UART_CTL_TXE_R {
        UART_CTL_TXE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Receive Enable"]
    #[inline(always)]
    pub fn uart_ctl_rxe(&self) -> UART_CTL_RXE_R {
        UART_CTL_RXE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Request to Send"]
    #[inline(always)]
    pub fn uart_ctl_rts(&self) -> UART_CTL_RTS_R {
        UART_CTL_RTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Request to Send"]
    #[inline(always)]
    pub fn uart_ctl_rtsen(&self) -> UART_CTL_RTSEN_R {
        UART_CTL_RTSEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Clear To Send"]
    #[inline(always)]
    pub fn uart_ctl_ctsen(&self) -> UART_CTL_CTSEN_R {
        UART_CTL_CTSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_uarten(&mut self) -> UART_CTL_UARTEN_W<CTL_SPEC, 0> {
        UART_CTL_UARTEN_W::new(self)
    }
    #[doc = "Bit 1 - UART SIR Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_siren(&mut self) -> UART_CTL_SIREN_W<CTL_SPEC, 1> {
        UART_CTL_SIREN_W::new(self)
    }
    #[doc = "Bit 2 - UART SIR Low-Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_sirlp(&mut self) -> UART_CTL_SIRLP_W<CTL_SPEC, 2> {
        UART_CTL_SIRLP_W::new(self)
    }
    #[doc = "Bit 3 - ISO 7816 Smart Card Support"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_smart(&mut self) -> UART_CTL_SMART_W<CTL_SPEC, 3> {
        UART_CTL_SMART_W::new(self)
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_eot(&mut self) -> UART_CTL_EOT_W<CTL_SPEC, 4> {
        UART_CTL_EOT_W::new(self)
    }
    #[doc = "Bit 5 - High-Speed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_hse(&mut self) -> UART_CTL_HSE_W<CTL_SPEC, 5> {
        UART_CTL_HSE_W::new(self)
    }
    #[doc = "Bit 7 - UART Loop Back Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_lbe(&mut self) -> UART_CTL_LBE_W<CTL_SPEC, 7> {
        UART_CTL_LBE_W::new(self)
    }
    #[doc = "Bit 8 - UART Transmit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_txe(&mut self) -> UART_CTL_TXE_W<CTL_SPEC, 8> {
        UART_CTL_TXE_W::new(self)
    }
    #[doc = "Bit 9 - UART Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_rxe(&mut self) -> UART_CTL_RXE_W<CTL_SPEC, 9> {
        UART_CTL_RXE_W::new(self)
    }
    #[doc = "Bit 11 - Request to Send"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_rts(&mut self) -> UART_CTL_RTS_W<CTL_SPEC, 11> {
        UART_CTL_RTS_W::new(self)
    }
    #[doc = "Bit 14 - Enable Request to Send"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_rtsen(&mut self) -> UART_CTL_RTSEN_W<CTL_SPEC, 14> {
        UART_CTL_RTSEN_W::new(self)
    }
    #[doc = "Bit 15 - Enable Clear To Send"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ctl_ctsen(&mut self) -> UART_CTL_CTSEN_W<CTL_SPEC, 15> {
        UART_CTL_CTSEN_W::new(self)
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
#[doc = "UART Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
