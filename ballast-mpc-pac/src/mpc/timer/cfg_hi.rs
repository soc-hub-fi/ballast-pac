#[doc = "Register `CFG_HI` reader"]
pub struct R(crate::R<CFG_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_HI` writer"]
pub struct W(crate::W<CFG_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_HI_SPEC>;
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
impl From<crate::W<CFG_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Timer enable (starts couting) bit"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Timer enable (starts couting) bit"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `RST` reader - Timer high reset, cleared after the reset happened"]
pub struct RST_R(crate::FieldReader<bool, bool>);
impl RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST` writer - Timer high reset, cleared after the reset happened"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `IRQEN` reader - Timer hi interrupt generation on compare match enable"]
pub struct IRQEN_R(crate::FieldReader<bool, bool>);
impl IRQEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQEN` writer - Timer hi interrupt generation on compare match enable"]
pub struct IRQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `MODE` reader - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
pub struct MODE_R(crate::FieldReader<bool, bool>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `ONES` reader - Timer hi one shot configuration"]
pub struct ONES_R(crate::FieldReader<bool, bool>);
impl ONES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONES` writer - Timer hi one shot configuration"]
pub struct ONES_W<'a> {
    w: &'a mut W,
}
impl<'a> ONES_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `PEN` reader - Timer hi prescaler enable bit"]
pub struct PEN_R(crate::FieldReader<bool, bool>);
impl PEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN` writer - Timer hi prescaler enable bit"]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `CCFG` reader - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
pub struct CCFG_R(crate::FieldReader<bool, bool>);
impl CCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCFG` writer - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
pub struct CCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CCFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `PVAL` reader - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
pub struct PVAL_R(crate::FieldReader<u8, u8>);
impl PVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVAL` writer - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
pub struct PVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer enable (starts couting) bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer high reset, cleared after the reset happened"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer hi interrupt generation on compare match enable"]
    #[inline(always)]
    pub fn irqen(&self) -> IRQEN_R {
        IRQEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer hi one shot configuration"]
    #[inline(always)]
    pub fn ones(&self) -> ONES_R {
        ONES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer hi prescaler enable bit"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
    #[inline(always)]
    pub fn ccfg(&self) -> CCFG_R {
        CCFG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
    #[inline(always)]
    pub fn pval(&self) -> PVAL_R {
        PVAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer enable (starts couting) bit"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Timer high reset, cleared after the reset happened"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 2 - Timer hi interrupt generation on compare match enable"]
    #[inline(always)]
    pub fn irqen(&mut self) -> IRQEN_W {
        IRQEN_W { w: self }
    }
    #[doc = "Bit 4 - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 5 - Timer hi one shot configuration"]
    #[inline(always)]
    pub fn ones(&mut self) -> ONES_W {
        ONES_W { w: self }
    }
    #[doc = "Bit 6 - Timer hi prescaler enable bit"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
    #[doc = "Bit 7 - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
    #[inline(always)]
    pub fn ccfg(&mut self) -> CCFG_W {
        CCFG_W { w: self }
    }
    #[doc = "Bits 8:15 - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
    #[inline(always)]
    pub fn pval(&mut self) -> PVAL_W {
        PVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer High Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_hi](index.html) module"]
pub struct CFG_HI_SPEC;
impl crate::RegisterSpec for CFG_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_hi::R](R) reader structure"]
impl crate::Readable for CFG_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_hi::W](W) writer structure"]
impl crate::Writable for CFG_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_HI to value 0"]
impl crate::Resettable for CFG_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
