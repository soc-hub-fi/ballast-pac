#[doc = "Register `PAD_MUX_0` reader"]
pub struct R(crate::R<PAD_MUX_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_MUX_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_MUX_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_MUX_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_MUX_0` writer"]
pub struct W(crate::W<PAD_MUX_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_MUX_0_SPEC>;
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
impl From<crate::W<PAD_MUX_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_MUX_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pad_0` reader - "]
pub struct PAD_0_R(crate::FieldReader<u8, u8>);
impl PAD_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_0` writer - "]
pub struct PAD_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `pad_1` reader - "]
pub struct PAD_1_R(crate::FieldReader<u8, u8>);
impl PAD_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_1` writer - "]
pub struct PAD_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `pad_2` reader - "]
pub struct PAD_2_R(crate::FieldReader<u8, u8>);
impl PAD_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_2` writer - "]
pub struct PAD_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `pad_3` reader - "]
pub struct PAD_3_R(crate::FieldReader<u8, u8>);
impl PAD_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_3` writer - "]
pub struct PAD_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `pad_4` reader - "]
pub struct PAD_4_R(crate::FieldReader<u8, u8>);
impl PAD_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_4` writer - "]
pub struct PAD_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `pad_5` reader - "]
pub struct PAD_5_R(crate::FieldReader<u8, u8>);
impl PAD_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_5` writer - "]
pub struct PAD_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `pad_6` reader - "]
pub struct PAD_6_R(crate::FieldReader<u8, u8>);
impl PAD_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_6` writer - "]
pub struct PAD_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `pad_7` reader - "]
pub struct PAD_7_R(crate::FieldReader<u8, u8>);
impl PAD_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_7` writer - "]
pub struct PAD_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `pad_8` reader - "]
pub struct PAD_8_R(crate::FieldReader<u8, u8>);
impl PAD_8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_8` writer - "]
pub struct PAD_8_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `pad_9` reader - "]
pub struct PAD_9_R(crate::FieldReader<u8, u8>);
impl PAD_9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_9` writer - "]
pub struct PAD_9_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `pad_10` reader - "]
pub struct PAD_10_R(crate::FieldReader<u8, u8>);
impl PAD_10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_10` writer - "]
pub struct PAD_10_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `pad_11` reader - "]
pub struct PAD_11_R(crate::FieldReader<u8, u8>);
impl PAD_11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_11` writer - "]
pub struct PAD_11_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `pad_12` reader - "]
pub struct PAD_12_R(crate::FieldReader<u8, u8>);
impl PAD_12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_12` writer - "]
pub struct PAD_12_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `pad_13` reader - "]
pub struct PAD_13_R(crate::FieldReader<u8, u8>);
impl PAD_13_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_13` writer - "]
pub struct PAD_13_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `pad_14` reader - "]
pub struct PAD_14_R(crate::FieldReader<u8, u8>);
impl PAD_14_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_14` writer - "]
pub struct PAD_14_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `pad_15` reader - "]
pub struct PAD_15_R(crate::FieldReader<u8, u8>);
impl PAD_15_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_15` writer - "]
pub struct PAD_15_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_15_W<'a> {
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
    pub fn pad_0(&self) -> PAD_0_R {
        PAD_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_1(&self) -> PAD_1_R {
        PAD_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pad_2(&self) -> PAD_2_R {
        PAD_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pad_3(&self) -> PAD_3_R {
        PAD_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pad_4(&self) -> PAD_4_R {
        PAD_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_5(&self) -> PAD_5_R {
        PAD_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_6(&self) -> PAD_6_R {
        PAD_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pad_7(&self) -> PAD_7_R {
        PAD_7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pad_8(&self) -> PAD_8_R {
        PAD_8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pad_9(&self) -> PAD_9_R {
        PAD_9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_10(&self) -> PAD_10_R {
        PAD_10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_11(&self) -> PAD_11_R {
        PAD_11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pad_12(&self) -> PAD_12_R {
        PAD_12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pad_13(&self) -> PAD_13_R {
        PAD_13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pad_14(&self) -> PAD_14_R {
        PAD_14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pad_15(&self) -> PAD_15_R {
        PAD_15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pad_0(&mut self) -> PAD_0_W {
        PAD_0_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_1(&mut self) -> PAD_1_W {
        PAD_1_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pad_2(&mut self) -> PAD_2_W {
        PAD_2_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pad_3(&mut self) -> PAD_3_W {
        PAD_3_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pad_4(&mut self) -> PAD_4_W {
        PAD_4_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_5(&mut self) -> PAD_5_W {
        PAD_5_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_6(&mut self) -> PAD_6_W {
        PAD_6_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pad_7(&mut self) -> PAD_7_W {
        PAD_7_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pad_8(&mut self) -> PAD_8_W {
        PAD_8_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pad_9(&mut self) -> PAD_9_W {
        PAD_9_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_10(&mut self) -> PAD_10_W {
        PAD_10_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_11(&mut self) -> PAD_11_W {
        PAD_11_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pad_12(&mut self) -> PAD_12_W {
        PAD_12_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pad_13(&mut self) -> PAD_13_W {
        PAD_13_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pad_14(&mut self) -> PAD_14_W {
        PAD_14_W { w: self }
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pad_15(&mut self) -> PAD_15_W {
        PAD_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The content of these registers can be used to multiplex pads when targeting an ASIC. The first register (0x1A10_4010) can be used to set the mux (2 bit select) from pin 0 (bits \\[1:0\\]) to 15 (bits \\[31:30\\]).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_mux_0](index.html) module"]
pub struct PAD_MUX_0_SPEC;
impl crate::RegisterSpec for PAD_MUX_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_mux_0::R](R) reader structure"]
impl crate::Readable for PAD_MUX_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_mux_0::W](W) writer structure"]
impl crate::Writable for PAD_MUX_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_MUX_0 to value 0"]
impl crate::Resettable for PAD_MUX_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
