#[doc = "Register `PAD_MUX_2` reader"]
pub struct R(crate::R<PAD_MUX_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_MUX_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_MUX_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_MUX_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_MUX_2` writer"]
pub struct W(crate::W<PAD_MUX_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_MUX_2_SPEC>;
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
impl From<crate::W<PAD_MUX_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_MUX_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pad_32` reader - "]
pub struct PAD_32_R(crate::FieldReader<u8>);
impl PAD_32_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_32_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_32` writer - "]
pub struct PAD_32_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `pad_33` reader - "]
pub struct PAD_33_R(crate::FieldReader<u8>);
impl PAD_33_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_33_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_33` writer - "]
pub struct PAD_33_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_33_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `pad_34` reader - "]
pub struct PAD_34_R(crate::FieldReader<u8>);
impl PAD_34_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_34_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_34` writer - "]
pub struct PAD_34_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_34_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `pad_35` reader - "]
pub struct PAD_35_R(crate::FieldReader<u8>);
impl PAD_35_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_35_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_35` writer - "]
pub struct PAD_35_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_35_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `pad_36` reader - "]
pub struct PAD_36_R(crate::FieldReader<u8>);
impl PAD_36_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_36_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_36` writer - "]
pub struct PAD_36_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_36_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `pad_37` reader - "]
pub struct PAD_37_R(crate::FieldReader<u8>);
impl PAD_37_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_37_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_37` writer - "]
pub struct PAD_37_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_37_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `pad_38` reader - "]
pub struct PAD_38_R(crate::FieldReader<u8>);
impl PAD_38_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_38_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_38` writer - "]
pub struct PAD_38_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_38_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `pad_39` reader - "]
pub struct PAD_39_R(crate::FieldReader<u8>);
impl PAD_39_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_39_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_39` writer - "]
pub struct PAD_39_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_39_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `pad_40` reader - "]
pub struct PAD_40_R(crate::FieldReader<u8>);
impl PAD_40_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_40_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_40` writer - "]
pub struct PAD_40_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_40_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `pad_41` reader - "]
pub struct PAD_41_R(crate::FieldReader<u8>);
impl PAD_41_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_41_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_41` writer - "]
pub struct PAD_41_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_41_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `pad_42` reader - "]
pub struct PAD_42_R(crate::FieldReader<u8>);
impl PAD_42_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_42_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_42` writer - "]
pub struct PAD_42_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_42_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `pad_43` reader - "]
pub struct PAD_43_R(crate::FieldReader<u8>);
impl PAD_43_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_43_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_43` writer - "]
pub struct PAD_43_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_43_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `pad_44` reader - "]
pub struct PAD_44_R(crate::FieldReader<u8>);
impl PAD_44_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_44_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_44_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_44` writer - "]
pub struct PAD_44_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_44_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `pad_45` reader - "]
pub struct PAD_45_R(crate::FieldReader<u8>);
impl PAD_45_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_45_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_45` writer - "]
pub struct PAD_45_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_45_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `pad_46` reader - "]
pub struct PAD_46_R(crate::FieldReader<u8>);
impl PAD_46_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_46_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_46_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_46` writer - "]
pub struct PAD_46_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_46_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `pad_47` reader - "]
pub struct PAD_47_R(crate::FieldReader<u8>);
impl PAD_47_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_47_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_47_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_47` writer - "]
pub struct PAD_47_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_47_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pad_32(&self) -> PAD_32_R {
        PAD_32_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_33(&self) -> PAD_33_R {
        PAD_33_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pad_34(&self) -> PAD_34_R {
        PAD_34_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pad_35(&self) -> PAD_35_R {
        PAD_35_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pad_36(&self) -> PAD_36_R {
        PAD_36_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_37(&self) -> PAD_37_R {
        PAD_37_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_38(&self) -> PAD_38_R {
        PAD_38_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pad_39(&self) -> PAD_39_R {
        PAD_39_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pad_40(&self) -> PAD_40_R {
        PAD_40_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pad_41(&self) -> PAD_41_R {
        PAD_41_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_42(&self) -> PAD_42_R {
        PAD_42_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_43(&self) -> PAD_43_R {
        PAD_43_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pad_44(&self) -> PAD_44_R {
        PAD_44_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pad_45(&self) -> PAD_45_R {
        PAD_45_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pad_46(&self) -> PAD_46_R {
        PAD_46_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pad_47(&self) -> PAD_47_R {
        PAD_47_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pad_32(&mut self) -> PAD_32_W {
        PAD_32_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_33(&mut self) -> PAD_33_W {
        PAD_33_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pad_34(&mut self) -> PAD_34_W {
        PAD_34_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pad_35(&mut self) -> PAD_35_W {
        PAD_35_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pad_36(&mut self) -> PAD_36_W {
        PAD_36_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_37(&mut self) -> PAD_37_W {
        PAD_37_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_38(&mut self) -> PAD_38_W {
        PAD_38_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pad_39(&mut self) -> PAD_39_W {
        PAD_39_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pad_40(&mut self) -> PAD_40_W {
        PAD_40_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pad_41(&mut self) -> PAD_41_W {
        PAD_41_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_42(&mut self) -> PAD_42_W {
        PAD_42_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_43(&mut self) -> PAD_43_W {
        PAD_43_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pad_44(&mut self) -> PAD_44_W {
        PAD_44_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pad_45(&mut self) -> PAD_45_W {
        PAD_45_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pad_46(&mut self) -> PAD_46_W {
        PAD_46_W { w: self }
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pad_47(&mut self) -> PAD_47_W {
        PAD_47_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The content of these registers can be used to multiplex pads when targeting an ASIC. The third register (0x1A10_4018) can be used to sets the mux (2 bit select) from pin 32 (bits \\[1:0\\]) to 47 (bits \\[31:30\\]).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_mux_2](index.html) module"]
pub struct PAD_MUX_2_SPEC;
impl crate::RegisterSpec for PAD_MUX_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_mux_2::R](R) reader structure"]
impl crate::Readable for PAD_MUX_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_mux_2::W](W) writer structure"]
impl crate::Writable for PAD_MUX_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_MUX_2 to value 0"]
impl crate::Resettable for PAD_MUX_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
