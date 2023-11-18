#[doc = "Register `DC7` reader"]
pub type R = crate::R<DC7_SPEC>;
#[doc = "Register `DC7` writer"]
pub type W = crate::W<DC7_SPEC>;
#[doc = "Field `SYSCTL_DC7_DMACH0` reader - USB_EP1_RX / UART2_RX"]
pub type SYSCTL_DC7_DMACH0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH0` writer - USB_EP1_RX / UART2_RX"]
pub type SYSCTL_DC7_DMACH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH1` reader - USB_EP1_TX / UART2_TX"]
pub type SYSCTL_DC7_DMACH1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH1` writer - USB_EP1_TX / UART2_TX"]
pub type SYSCTL_DC7_DMACH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH2` reader - USB_EP2_RX / Timer3A"]
pub type SYSCTL_DC7_DMACH2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH2` writer - USB_EP2_RX / Timer3A"]
pub type SYSCTL_DC7_DMACH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH3` reader - USB_EP2_TX / Timer3B"]
pub type SYSCTL_DC7_DMACH3_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH3` writer - USB_EP2_TX / Timer3B"]
pub type SYSCTL_DC7_DMACH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH4` reader - USB_EP3_RX / Timer2A"]
pub type SYSCTL_DC7_DMACH4_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH4` writer - USB_EP3_RX / Timer2A"]
pub type SYSCTL_DC7_DMACH4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH5` reader - USB_EP3_TX / Timer2B"]
pub type SYSCTL_DC7_DMACH5_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH5` writer - USB_EP3_TX / Timer2B"]
pub type SYSCTL_DC7_DMACH5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH6` reader - ETH_RX / Timer2A"]
pub type SYSCTL_DC7_DMACH6_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH6` writer - ETH_RX / Timer2A"]
pub type SYSCTL_DC7_DMACH6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH7` reader - ETH_TX / Timer2B"]
pub type SYSCTL_DC7_DMACH7_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH7` writer - ETH_TX / Timer2B"]
pub type SYSCTL_DC7_DMACH7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH8` reader - UART0_RX / UART1_RX"]
pub type SYSCTL_DC7_DMACH8_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH8` writer - UART0_RX / UART1_RX"]
pub type SYSCTL_DC7_DMACH8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH9` reader - UART0_TX / UART1_TX"]
pub type SYSCTL_DC7_DMACH9_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH9` writer - UART0_TX / UART1_TX"]
pub type SYSCTL_DC7_DMACH9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH10` reader - SSI0_RX / SSI1_RX"]
pub type SYSCTL_DC7_DMACH10_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH10` writer - SSI0_RX / SSI1_RX"]
pub type SYSCTL_DC7_DMACH10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH11` reader - SSI0_TX / SSI1_TX"]
pub type SYSCTL_DC7_DMACH11_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH11` writer - SSI0_TX / SSI1_TX"]
pub type SYSCTL_DC7_DMACH11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH12` reader - CAN0_RX / UART2_RX"]
pub type SYSCTL_DC7_DMACH12_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH12` writer - CAN0_RX / UART2_RX"]
pub type SYSCTL_DC7_DMACH12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH13` reader - CAN0_TX / UART2_TX"]
pub type SYSCTL_DC7_DMACH13_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH13` writer - CAN0_TX / UART2_TX"]
pub type SYSCTL_DC7_DMACH13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH14` reader - ADC0_SS0 / Timer2A"]
pub type SYSCTL_DC7_DMACH14_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH14` writer - ADC0_SS0 / Timer2A"]
pub type SYSCTL_DC7_DMACH14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH15` reader - ADC0_SS1 / Timer2B"]
pub type SYSCTL_DC7_DMACH15_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH15` writer - ADC0_SS1 / Timer2B"]
pub type SYSCTL_DC7_DMACH15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH16` reader - ADC0_SS2"]
pub type SYSCTL_DC7_DMACH16_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH16` writer - ADC0_SS2"]
pub type SYSCTL_DC7_DMACH16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH17` reader - ADC0_SS3"]
pub type SYSCTL_DC7_DMACH17_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH17` writer - ADC0_SS3"]
pub type SYSCTL_DC7_DMACH17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH18` reader - Timer0A / Timer1A"]
pub type SYSCTL_DC7_DMACH18_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH18` writer - Timer0A / Timer1A"]
pub type SYSCTL_DC7_DMACH18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH19` reader - Timer0B / Timer1B"]
pub type SYSCTL_DC7_DMACH19_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH19` writer - Timer0B / Timer1B"]
pub type SYSCTL_DC7_DMACH19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH20` reader - Timer1A / EPI0_NBRFIFO"]
pub type SYSCTL_DC7_DMACH20_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH20` writer - Timer1A / EPI0_NBRFIFO"]
pub type SYSCTL_DC7_DMACH20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH21` reader - Timer1B / EPI0_WFIFO"]
pub type SYSCTL_DC7_DMACH21_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH21` writer - Timer1B / EPI0_WFIFO"]
pub type SYSCTL_DC7_DMACH21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH22` reader - UART1_RX / CAN2_RX"]
pub type SYSCTL_DC7_DMACH22_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH22` writer - UART1_RX / CAN2_RX"]
pub type SYSCTL_DC7_DMACH22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH23` reader - UART1_TX / CAN2_TX"]
pub type SYSCTL_DC7_DMACH23_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH23` writer - UART1_TX / CAN2_TX"]
pub type SYSCTL_DC7_DMACH23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH24` reader - SSI1_RX / ADC1_SS0"]
pub type SYSCTL_DC7_DMACH24_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH24` writer - SSI1_RX / ADC1_SS0"]
pub type SYSCTL_DC7_DMACH24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH25` reader - SSI1_TX / ADC1_SS1"]
pub type SYSCTL_DC7_DMACH25_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH25` writer - SSI1_TX / ADC1_SS1"]
pub type SYSCTL_DC7_DMACH25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH26` reader - CAN1_RX / ADC1_SS2"]
pub type SYSCTL_DC7_DMACH26_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH26` writer - CAN1_RX / ADC1_SS2"]
pub type SYSCTL_DC7_DMACH26_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH27` reader - CAN1_TX / ADC1_SS3"]
pub type SYSCTL_DC7_DMACH27_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH27` writer - CAN1_TX / ADC1_SS3"]
pub type SYSCTL_DC7_DMACH27_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH28` reader - I2S0_RX / CAN1_RX"]
pub type SYSCTL_DC7_DMACH28_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH28` writer - I2S0_RX / CAN1_RX"]
pub type SYSCTL_DC7_DMACH28_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH29` reader - I2S0_TX / CAN1_TX"]
pub type SYSCTL_DC7_DMACH29_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH29` writer - I2S0_TX / CAN1_TX"]
pub type SYSCTL_DC7_DMACH29_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC7_DMACH30` reader - SW"]
pub type SYSCTL_DC7_DMACH30_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC7_DMACH30` writer - SW"]
pub type SYSCTL_DC7_DMACH30_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB_EP1_RX / UART2_RX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach0(&self) -> SYSCTL_DC7_DMACH0_R {
        SYSCTL_DC7_DMACH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB_EP1_TX / UART2_TX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach1(&self) -> SYSCTL_DC7_DMACH1_R {
        SYSCTL_DC7_DMACH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB_EP2_RX / Timer3A"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach2(&self) -> SYSCTL_DC7_DMACH2_R {
        SYSCTL_DC7_DMACH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB_EP2_TX / Timer3B"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach3(&self) -> SYSCTL_DC7_DMACH3_R {
        SYSCTL_DC7_DMACH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB_EP3_RX / Timer2A"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach4(&self) -> SYSCTL_DC7_DMACH4_R {
        SYSCTL_DC7_DMACH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB_EP3_TX / Timer2B"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach5(&self) -> SYSCTL_DC7_DMACH5_R {
        SYSCTL_DC7_DMACH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ETH_RX / Timer2A"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach6(&self) -> SYSCTL_DC7_DMACH6_R {
        SYSCTL_DC7_DMACH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ETH_TX / Timer2B"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach7(&self) -> SYSCTL_DC7_DMACH7_R {
        SYSCTL_DC7_DMACH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART0_RX / UART1_RX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach8(&self) -> SYSCTL_DC7_DMACH8_R {
        SYSCTL_DC7_DMACH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART0_TX / UART1_TX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach9(&self) -> SYSCTL_DC7_DMACH9_R {
        SYSCTL_DC7_DMACH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SSI0_RX / SSI1_RX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach10(&self) -> SYSCTL_DC7_DMACH10_R {
        SYSCTL_DC7_DMACH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SSI0_TX / SSI1_TX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach11(&self) -> SYSCTL_DC7_DMACH11_R {
        SYSCTL_DC7_DMACH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CAN0_RX / UART2_RX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach12(&self) -> SYSCTL_DC7_DMACH12_R {
        SYSCTL_DC7_DMACH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CAN0_TX / UART2_TX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach13(&self) -> SYSCTL_DC7_DMACH13_R {
        SYSCTL_DC7_DMACH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC0_SS0 / Timer2A"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach14(&self) -> SYSCTL_DC7_DMACH14_R {
        SYSCTL_DC7_DMACH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC0_SS1 / Timer2B"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach15(&self) -> SYSCTL_DC7_DMACH15_R {
        SYSCTL_DC7_DMACH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC0_SS2"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach16(&self) -> SYSCTL_DC7_DMACH16_R {
        SYSCTL_DC7_DMACH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC0_SS3"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach17(&self) -> SYSCTL_DC7_DMACH17_R {
        SYSCTL_DC7_DMACH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer0A / Timer1A"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach18(&self) -> SYSCTL_DC7_DMACH18_R {
        SYSCTL_DC7_DMACH18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer0B / Timer1B"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach19(&self) -> SYSCTL_DC7_DMACH19_R {
        SYSCTL_DC7_DMACH19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer1A / EPI0_NBRFIFO"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach20(&self) -> SYSCTL_DC7_DMACH20_R {
        SYSCTL_DC7_DMACH20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer1B / EPI0_WFIFO"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach21(&self) -> SYSCTL_DC7_DMACH21_R {
        SYSCTL_DC7_DMACH21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - UART1_RX / CAN2_RX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach22(&self) -> SYSCTL_DC7_DMACH22_R {
        SYSCTL_DC7_DMACH22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - UART1_TX / CAN2_TX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach23(&self) -> SYSCTL_DC7_DMACH23_R {
        SYSCTL_DC7_DMACH23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SSI1_RX / ADC1_SS0"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach24(&self) -> SYSCTL_DC7_DMACH24_R {
        SYSCTL_DC7_DMACH24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SSI1_TX / ADC1_SS1"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach25(&self) -> SYSCTL_DC7_DMACH25_R {
        SYSCTL_DC7_DMACH25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN1_RX / ADC1_SS2"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach26(&self) -> SYSCTL_DC7_DMACH26_R {
        SYSCTL_DC7_DMACH26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CAN1_TX / ADC1_SS3"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach27(&self) -> SYSCTL_DC7_DMACH27_R {
        SYSCTL_DC7_DMACH27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - I2S0_RX / CAN1_RX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach28(&self) -> SYSCTL_DC7_DMACH28_R {
        SYSCTL_DC7_DMACH28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - I2S0_TX / CAN1_TX"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach29(&self) -> SYSCTL_DC7_DMACH29_R {
        SYSCTL_DC7_DMACH29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SW"]
    #[inline(always)]
    pub fn sysctl_dc7_dmach30(&self) -> SYSCTL_DC7_DMACH30_R {
        SYSCTL_DC7_DMACH30_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB_EP1_RX / UART2_RX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach0(&mut self) -> SYSCTL_DC7_DMACH0_W<DC7_SPEC, 0> {
        SYSCTL_DC7_DMACH0_W::new(self)
    }
    #[doc = "Bit 1 - USB_EP1_TX / UART2_TX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach1(&mut self) -> SYSCTL_DC7_DMACH1_W<DC7_SPEC, 1> {
        SYSCTL_DC7_DMACH1_W::new(self)
    }
    #[doc = "Bit 2 - USB_EP2_RX / Timer3A"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach2(&mut self) -> SYSCTL_DC7_DMACH2_W<DC7_SPEC, 2> {
        SYSCTL_DC7_DMACH2_W::new(self)
    }
    #[doc = "Bit 3 - USB_EP2_TX / Timer3B"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach3(&mut self) -> SYSCTL_DC7_DMACH3_W<DC7_SPEC, 3> {
        SYSCTL_DC7_DMACH3_W::new(self)
    }
    #[doc = "Bit 4 - USB_EP3_RX / Timer2A"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach4(&mut self) -> SYSCTL_DC7_DMACH4_W<DC7_SPEC, 4> {
        SYSCTL_DC7_DMACH4_W::new(self)
    }
    #[doc = "Bit 5 - USB_EP3_TX / Timer2B"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach5(&mut self) -> SYSCTL_DC7_DMACH5_W<DC7_SPEC, 5> {
        SYSCTL_DC7_DMACH5_W::new(self)
    }
    #[doc = "Bit 6 - ETH_RX / Timer2A"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach6(&mut self) -> SYSCTL_DC7_DMACH6_W<DC7_SPEC, 6> {
        SYSCTL_DC7_DMACH6_W::new(self)
    }
    #[doc = "Bit 7 - ETH_TX / Timer2B"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach7(&mut self) -> SYSCTL_DC7_DMACH7_W<DC7_SPEC, 7> {
        SYSCTL_DC7_DMACH7_W::new(self)
    }
    #[doc = "Bit 8 - UART0_RX / UART1_RX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach8(&mut self) -> SYSCTL_DC7_DMACH8_W<DC7_SPEC, 8> {
        SYSCTL_DC7_DMACH8_W::new(self)
    }
    #[doc = "Bit 9 - UART0_TX / UART1_TX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach9(&mut self) -> SYSCTL_DC7_DMACH9_W<DC7_SPEC, 9> {
        SYSCTL_DC7_DMACH9_W::new(self)
    }
    #[doc = "Bit 10 - SSI0_RX / SSI1_RX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach10(&mut self) -> SYSCTL_DC7_DMACH10_W<DC7_SPEC, 10> {
        SYSCTL_DC7_DMACH10_W::new(self)
    }
    #[doc = "Bit 11 - SSI0_TX / SSI1_TX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach11(&mut self) -> SYSCTL_DC7_DMACH11_W<DC7_SPEC, 11> {
        SYSCTL_DC7_DMACH11_W::new(self)
    }
    #[doc = "Bit 12 - CAN0_RX / UART2_RX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach12(&mut self) -> SYSCTL_DC7_DMACH12_W<DC7_SPEC, 12> {
        SYSCTL_DC7_DMACH12_W::new(self)
    }
    #[doc = "Bit 13 - CAN0_TX / UART2_TX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach13(&mut self) -> SYSCTL_DC7_DMACH13_W<DC7_SPEC, 13> {
        SYSCTL_DC7_DMACH13_W::new(self)
    }
    #[doc = "Bit 14 - ADC0_SS0 / Timer2A"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach14(&mut self) -> SYSCTL_DC7_DMACH14_W<DC7_SPEC, 14> {
        SYSCTL_DC7_DMACH14_W::new(self)
    }
    #[doc = "Bit 15 - ADC0_SS1 / Timer2B"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach15(&mut self) -> SYSCTL_DC7_DMACH15_W<DC7_SPEC, 15> {
        SYSCTL_DC7_DMACH15_W::new(self)
    }
    #[doc = "Bit 16 - ADC0_SS2"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach16(&mut self) -> SYSCTL_DC7_DMACH16_W<DC7_SPEC, 16> {
        SYSCTL_DC7_DMACH16_W::new(self)
    }
    #[doc = "Bit 17 - ADC0_SS3"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach17(&mut self) -> SYSCTL_DC7_DMACH17_W<DC7_SPEC, 17> {
        SYSCTL_DC7_DMACH17_W::new(self)
    }
    #[doc = "Bit 18 - Timer0A / Timer1A"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach18(&mut self) -> SYSCTL_DC7_DMACH18_W<DC7_SPEC, 18> {
        SYSCTL_DC7_DMACH18_W::new(self)
    }
    #[doc = "Bit 19 - Timer0B / Timer1B"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach19(&mut self) -> SYSCTL_DC7_DMACH19_W<DC7_SPEC, 19> {
        SYSCTL_DC7_DMACH19_W::new(self)
    }
    #[doc = "Bit 20 - Timer1A / EPI0_NBRFIFO"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach20(&mut self) -> SYSCTL_DC7_DMACH20_W<DC7_SPEC, 20> {
        SYSCTL_DC7_DMACH20_W::new(self)
    }
    #[doc = "Bit 21 - Timer1B / EPI0_WFIFO"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach21(&mut self) -> SYSCTL_DC7_DMACH21_W<DC7_SPEC, 21> {
        SYSCTL_DC7_DMACH21_W::new(self)
    }
    #[doc = "Bit 22 - UART1_RX / CAN2_RX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach22(&mut self) -> SYSCTL_DC7_DMACH22_W<DC7_SPEC, 22> {
        SYSCTL_DC7_DMACH22_W::new(self)
    }
    #[doc = "Bit 23 - UART1_TX / CAN2_TX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach23(&mut self) -> SYSCTL_DC7_DMACH23_W<DC7_SPEC, 23> {
        SYSCTL_DC7_DMACH23_W::new(self)
    }
    #[doc = "Bit 24 - SSI1_RX / ADC1_SS0"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach24(&mut self) -> SYSCTL_DC7_DMACH24_W<DC7_SPEC, 24> {
        SYSCTL_DC7_DMACH24_W::new(self)
    }
    #[doc = "Bit 25 - SSI1_TX / ADC1_SS1"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach25(&mut self) -> SYSCTL_DC7_DMACH25_W<DC7_SPEC, 25> {
        SYSCTL_DC7_DMACH25_W::new(self)
    }
    #[doc = "Bit 26 - CAN1_RX / ADC1_SS2"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach26(&mut self) -> SYSCTL_DC7_DMACH26_W<DC7_SPEC, 26> {
        SYSCTL_DC7_DMACH26_W::new(self)
    }
    #[doc = "Bit 27 - CAN1_TX / ADC1_SS3"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach27(&mut self) -> SYSCTL_DC7_DMACH27_W<DC7_SPEC, 27> {
        SYSCTL_DC7_DMACH27_W::new(self)
    }
    #[doc = "Bit 28 - I2S0_RX / CAN1_RX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach28(&mut self) -> SYSCTL_DC7_DMACH28_W<DC7_SPEC, 28> {
        SYSCTL_DC7_DMACH28_W::new(self)
    }
    #[doc = "Bit 29 - I2S0_TX / CAN1_TX"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach29(&mut self) -> SYSCTL_DC7_DMACH29_W<DC7_SPEC, 29> {
        SYSCTL_DC7_DMACH29_W::new(self)
    }
    #[doc = "Bit 30 - SW"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc7_dmach30(&mut self) -> SYSCTL_DC7_DMACH30_W<DC7_SPEC, 30> {
        SYSCTL_DC7_DMACH30_W::new(self)
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
#[doc = "Device Capabilities 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC7_SPEC;
impl crate::RegisterSpec for DC7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc7::R`](R) reader structure"]
impl crate::Readable for DC7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc7::W`](W) writer structure"]
impl crate::Writable for DC7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC7 to value 0"]
impl crate::Resettable for DC7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
