import React, { useState } from 'react';
import axios from 'axios';

const FileUpload = () => {
  const [selectedFile, setSelectedFile] = useState(null);

  const handleFileSelect = (event) => {
    setSelectedFile(event.target.files[0]);
  };

  const handleFileUpload = async () => {
    if (!selectedFile) {
      alert('Please select a file first!');
      return;
    }

    const formData = new FormData();
    formData.append('file', selectedFile);

    try {
      const response = await axios.post(process.env.REACT_APP_UPLOAD_URL, formData, {
        headers: {
          'Content-Type': 'multipart/form/data',
        },
      });
      alert('File uploaded successfully');
    } catch (error) {
      alert('File upload failed');
      console.error('Error uploading file:', error);
    }
  };

  return (
    <div>
      <h2>Upload a file</h2>
      <input type="file" onChange={handleFileSelect} />
      <button onClick={handleFileUpload}>Upload</button>
    </div>
  );
};

export default FileUpload;