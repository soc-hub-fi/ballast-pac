#[doc = "Register `PAD_MUX_1` reader"]
pub struct R(crate::R<PAD_MUX_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_MUX_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_MUX_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_MUX_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_MUX_1` writer"]
pub struct W(crate::W<PAD_MUX_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_MUX_1_SPEC>;
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
impl From<crate::W<PAD_MUX_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_MUX_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pad_16` reader - "]
pub struct PAD_16_R(crate::FieldReader<u8, u8>);
impl PAD_16_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_16` writer - "]
pub struct PAD_16_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `pad_17` reader - "]
pub struct PAD_17_R(crate::FieldReader<u8, u8>);
impl PAD_17_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_17_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_17` writer - "]
pub struct PAD_17_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `pad_18` reader - "]
pub struct PAD_18_R(crate::FieldReader<u8, u8>);
impl PAD_18_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_18_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_18` writer - "]
pub struct PAD_18_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `pad_19` reader - "]
pub struct PAD_19_R(crate::FieldReader<u8, u8>);
impl PAD_19_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_19_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_19` writer - "]
pub struct PAD_19_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `pad_20` reader - "]
pub struct PAD_20_R(crate::FieldReader<u8, u8>);
impl PAD_20_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_20_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_20` writer - "]
pub struct PAD_20_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `pad_21` reader - "]
pub struct PAD_21_R(crate::FieldReader<u8, u8>);
impl PAD_21_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_21_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_21` writer - "]
pub struct PAD_21_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `pad_22` reader - "]
pub struct PAD_22_R(crate::FieldReader<u8, u8>);
impl PAD_22_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_22_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_22` writer - "]
pub struct PAD_22_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `pad_23` reader - "]
pub struct PAD_23_R(crate::FieldReader<u8, u8>);
impl PAD_23_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_23_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_23` writer - "]
pub struct PAD_23_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `pad_24` reader - "]
pub struct PAD_24_R(crate::FieldReader<u8, u8>);
impl PAD_24_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_24` writer - "]
pub struct PAD_24_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `pad_25` reader - "]
pub struct PAD_25_R(crate::FieldReader<u8, u8>);
impl PAD_25_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_25_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_25` writer - "]
pub struct PAD_25_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `pad_26` reader - "]
pub struct PAD_26_R(crate::FieldReader<u8, u8>);
impl PAD_26_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_26_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_26` writer - "]
pub struct PAD_26_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `pad_27` reader - "]
pub struct PAD_27_R(crate::FieldReader<u8, u8>);
impl PAD_27_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_27_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_27` writer - "]
pub struct PAD_27_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `pad_28` reader - "]
pub struct PAD_28_R(crate::FieldReader<u8, u8>);
impl PAD_28_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_28_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_28` writer - "]
pub struct PAD_28_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `pad_29` reader - "]
pub struct PAD_29_R(crate::FieldReader<u8, u8>);
impl PAD_29_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_29_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_29` writer - "]
pub struct PAD_29_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `pad_30` reader - "]
pub struct PAD_30_R(crate::FieldReader<u8, u8>);
impl PAD_30_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_30_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_30` writer - "]
pub struct PAD_30_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `pad_31` reader - "]
pub struct PAD_31_R(crate::FieldReader<u8, u8>);
impl PAD_31_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_31_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_31` writer - "]
pub struct PAD_31_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_31_W<'a> {
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
    pub fn pad_16(&self) -> PAD_16_R {
        PAD_16_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_17(&self) -> PAD_17_R {
        PAD_17_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pad_18(&self) -> PAD_18_R {
        PAD_18_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pad_19(&self) -> PAD_19_R {
        PAD_19_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pad_20(&self) -> PAD_20_R {
        PAD_20_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_21(&self) -> PAD_21_R {
        PAD_21_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_22(&self) -> PAD_22_R {
        PAD_22_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pad_23(&self) -> PAD_23_R {
        PAD_23_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pad_24(&self) -> PAD_24_R {
        PAD_24_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pad_25(&self) -> PAD_25_R {
        PAD_25_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_26(&self) -> PAD_26_R {
        PAD_26_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_27(&self) -> PAD_27_R {
        PAD_27_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pad_28(&self) -> PAD_28_R {
        PAD_28_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pad_29(&self) -> PAD_29_R {
        PAD_29_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pad_30(&self) -> PAD_30_R {
        PAD_30_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pad_31(&self) -> PAD_31_R {
        PAD_31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pad_16(&mut self) -> PAD_16_W {
        PAD_16_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_17(&mut self) -> PAD_17_W {
        PAD_17_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pad_18(&mut self) -> PAD_18_W {
        PAD_18_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pad_19(&mut self) -> PAD_19_W {
        PAD_19_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pad_20(&mut self) -> PAD_20_W {
        PAD_20_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_21(&mut self) -> PAD_21_W {
        PAD_21_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_22(&mut self) -> PAD_22_W {
        PAD_22_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pad_23(&mut self) -> PAD_23_W {
        PAD_23_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pad_24(&mut self) -> PAD_24_W {
        PAD_24_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pad_25(&mut self) -> PAD_25_W {
        PAD_25_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_26(&mut self) -> PAD_26_W {
        PAD_26_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_27(&mut self) -> PAD_27_W {
        PAD_27_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pad_28(&mut self) -> PAD_28_W {
        PAD_28_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pad_29(&mut self) -> PAD_29_W {
        PAD_29_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pad_30(&mut self) -> PAD_30_W {
        PAD_30_W { w: self }
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pad_31(&mut self) -> PAD_31_W {
        PAD_31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The content of these registers can be used to multiplex pads when targeting an ASIC. The second register (0x1A10_4014) can be used to sets the mux (2 bit select) from pin 16 (bits \\[1:0\\]) to 31 (bits \\[31:30\\]).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_mux_1](index.html) module"]
pub struct PAD_MUX_1_SPEC;
impl crate::RegisterSpec for PAD_MUX_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_mux_1::R](R) reader structure"]
impl crate::Readable for PAD_MUX_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_mux_1::W](W) writer structure"]
impl crate::Writable for PAD_MUX_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_MUX_1 to value 0"]
impl crate::Resettable for PAD_MUX_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
