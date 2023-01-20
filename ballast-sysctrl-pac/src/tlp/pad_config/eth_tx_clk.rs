#[doc = "Register `eth_tx_clk` reader"]
pub struct R(crate::R<ETH_TX_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_TX_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_TX_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_TX_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `eth_tx_clk` writer"]
pub struct W(crate::W<ETH_TX_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_TX_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ETH_TX_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_TX_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pull_down` reader - "]
pub type PULL_DOWN_R = crate::BitReader<bool>;
#[doc = "Field `pull_down` writer - "]
pub type PULL_DOWN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ETH_TX_CLK_SPEC, bool, O>;
#[doc = "Field `pull_up` reader - "]
pub type PULL_UP_R = crate::BitReader<bool>;
#[doc = "Field `pull_up` writer - "]
pub type PULL_UP_W<'a, const O: u8> = crate::BitWriter<'a, u8, ETH_TX_CLK_SPEC, bool, O>;
#[doc = "Field `drive_strength` reader - "]
pub type DRIVE_STRENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `drive_strength` writer - "]
pub type DRIVE_STRENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ETH_TX_CLK_SPEC, u8, u8, 3, O>;
#[doc = "Field `output_enable` reader - "]
pub type OUTPUT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `output_enable` writer - "]
pub type OUTPUT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ETH_TX_CLK_SPEC, bool, O>;
#[doc = "Field `input_enable` reader - "]
pub type INPUT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `input_enable` writer - "]
pub type INPUT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ETH_TX_CLK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pull_down(&self) -> PULL_DOWN_R {
        PULL_DOWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pull_up(&self) -> PULL_UP_R {
        PULL_UP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn drive_strength(&self) -> DRIVE_STRENGTH_R {
        DRIVE_STRENGTH_R::new((self.bits >> 2) & 7)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn output_enable(&self) -> OUTPUT_ENABLE_R {
        OUTPUT_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn input_enable(&self) -> INPUT_ENABLE_R {
        INPUT_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down(&mut self) -> PULL_DOWN_W<0> {
        PULL_DOWN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up(&mut self) -> PULL_UP_W<1> {
        PULL_UP_W::new(self)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    #[must_use]
    pub fn drive_strength(&mut self) -> DRIVE_STRENGTH_W<2> {
        DRIVE_STRENGTH_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn output_enable(&mut self) -> OUTPUT_ENABLE_W<5> {
        OUTPUT_ENABLE_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn input_enable(&mut self) -> INPUT_ENABLE_W<7> {
        INPUT_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_tx_clk](index.html) module"]
pub struct ETH_TX_CLK_SPEC;
impl crate::RegisterSpec for ETH_TX_CLK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eth_tx_clk::R](R) reader structure"]
impl crate::Readable for ETH_TX_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_tx_clk::W](W) writer structure"]
impl crate::Writable for ETH_TX_CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets eth_tx_clk to value 0"]
impl crate::Resettable for ETH_TX_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
