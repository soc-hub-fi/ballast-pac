#[doc = "Register `eth_rx_clk` reader"]
pub struct R(crate::R<ETH_RX_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_RX_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_RX_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_RX_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `eth_rx_clk` writer"]
pub struct W(crate::W<ETH_RX_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_RX_CLK_SPEC>;
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
impl From<crate::W<ETH_RX_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_RX_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pull_down` reader - "]
pub struct PULL_DOWN_R(crate::FieldReader<bool>);
impl PULL_DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pull_down` writer - "]
pub struct PULL_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u8 & 1);
        self.w
    }
}
#[doc = "Field `pull_up` reader - "]
pub struct PULL_UP_R(crate::FieldReader<bool>);
impl PULL_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pull_up` writer - "]
pub struct PULL_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u8 & 1) << 1);
        self.w
    }
}
#[doc = "Field `drive_strength` reader - "]
pub struct DRIVE_STRENGTH_R(crate::FieldReader<u8>);
impl DRIVE_STRENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_STRENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIVE_STRENGTH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `drive_strength` writer - "]
pub struct DRIVE_STRENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_STRENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 2)) | ((value as u8 & 7) << 2);
        self.w
    }
}
#[doc = "Field `output_enable` reader - "]
pub struct OUTPUT_ENABLE_R(crate::FieldReader<bool>);
impl OUTPUT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTPUT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPUT_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `output_enable` writer - "]
pub struct OUTPUT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u8 & 1) << 5);
        self.w
    }
}
#[doc = "Field `input_enable` reader - "]
pub struct INPUT_ENABLE_R(crate::FieldReader<bool>);
impl INPUT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INPUT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `input_enable` writer - "]
pub struct INPUT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u8 & 1) << 7);
        self.w
    }
}
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
        DRIVE_STRENGTH_R::new(((self.bits >> 2) & 7) as u8)
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
    pub fn pull_down(&mut self) -> PULL_DOWN_W {
        PULL_DOWN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pull_up(&mut self) -> PULL_UP_W {
        PULL_UP_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn drive_strength(&mut self) -> DRIVE_STRENGTH_W {
        DRIVE_STRENGTH_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn output_enable(&mut self) -> OUTPUT_ENABLE_W {
        OUTPUT_ENABLE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn input_enable(&mut self) -> INPUT_ENABLE_W {
        INPUT_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_rx_clk](index.html) module"]
pub struct ETH_RX_CLK_SPEC;
impl crate::RegisterSpec for ETH_RX_CLK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eth_rx_clk::R](R) reader structure"]
impl crate::Readable for ETH_RX_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_rx_clk::W](W) writer structure"]
impl crate::Writable for ETH_RX_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets eth_rx_clk to value 0xa0"]
impl crate::Resettable for ETH_RX_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa0
    }
}
